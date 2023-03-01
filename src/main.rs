use clap::Parser;

use typers::{SentenceTyper, TextFileSource, WikipediaSource};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Source of the sentence. Can be `wiki` for Wikipedia random page or any text file path
    #[arg(short, long)]
    source: Option<String>,

    /// Number of sentences to be run
    #[arg(short, long, default_value_t = 1)]
    number: u8
}

fn main() {
    let args = Args::parse();

    match args.source {
        Some(src) => match src.to_ascii_lowercase().as_str() {
            "wiki" => {
                let wiki_source = WikipediaSource::default();
                let mut sentence = SentenceTyper::new(&wiki_source);
                for _ in 0..args.number {
                    sentence.type_sentence();
                    sentence.get_next_sentence(&wiki_source);
                }
            }
            file_path => {
                let file_source = TextFileSource::from_file(file_path);
                let mut sentence = SentenceTyper::new(&file_source);
                sentence.type_sentence();
            }
        },
        None => todo!(),
    }
}
