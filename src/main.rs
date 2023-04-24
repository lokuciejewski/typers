mod modules;

use std::io::{self, BufRead};

use clap::{Parser, ValueHint};
use modules::{
    sentence_typer::SentenceTyper,
    sources::{file_source::FileSource, text_source::TextSource, wiki_source::WikipediaSource},
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
    let mut sentence_typer = SentenceTyper::default();
    let args = Args::parse();

    // Piped input
    if atty::isnt(atty::Stream::Stdin) {
        let stdin = io::stdin();
        let read_line: String = stdin.lock().lines().next().unwrap().expect("");
        let text_source = TextSource::new(read_line, '.');
        sentence_typer.add_source(text_source);
    }

    // Wikipedia
    if args.wikipedia {
        let wiki_source = WikipediaSource::default();
        sentence_typer.add_source(wiki_source);
    }

    // Files
    match args.file_paths {
        Some(mut v) => {
            v.sort();
            v.dedup();
            v.into_iter().for_each(
                |file_path| match FileSource::from_file(file_path.to_owned()) {
                    Ok(file_source) => {
                        sentence_typer.add_source(file_source);
                    }
                    Err(err) => eprintln!("Could not add source: {} - {}", file_path, err),
                },
            );
        }
        None => (),
    }
    sentence_typer.type_sentences(args.number);
}
