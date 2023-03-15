mod modules;

use clap::{Parser, ValueHint};
use modules::{
    sentence_typer::SentenceTyper,
    sources::{file_source::TextFileSource, wiki_source::WikipediaSource},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Specify if the random summaries from Wikipedia should be used
    #[arg(short, long, default_value_t = false)]
    wikipedia: bool,

    /// Specify a file path(s) for sourcing sentences
    #[arg(short, long, num_args = 1.., value_hint = ValueHint::FilePath)]
    file_paths: Option<Vec<String>>,

    /// Number of sentences to be run
    #[arg(short, long, default_value_t = 1)]
    number: u8,
}

fn main() {
    let args = Args::parse();
    let mut sentence_typer = SentenceTyper::default();
    if args.wikipedia {
        let wiki_source = WikipediaSource::default();
        sentence_typer.add_source(wiki_source);
    }
    match args.file_paths {
        Some(mut v) => {
            v.sort();
            v.dedup();
            v.into_iter().for_each(|file_path| {
                match TextFileSource::from_file(file_path.to_owned()) {
                    Ok(file_source) => {
                        sentence_typer.add_source(file_source);
                    }
                    Err(err) => eprintln!("Could not add source: {} - {}", file_path, err),
                }
            });
        }
        None => todo!(),
    }
    sentence_typer.type_sentences(args.number);
}
