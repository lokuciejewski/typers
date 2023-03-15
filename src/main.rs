mod modules;

use clap::{Parser, ValueHint};
use modules::sentence_typer::SentenceTyper;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Specify if the random summaries from Wikipedia should be used
    #[arg(short, long)]
    wikipedia: Option<bool>,

    /// Specify a file path for sourcing sentences
    #[arg(short, long, value_hint = ValueHint::FilePath)]
    file_path: Option<String>,

    /// Number of sentences to be run
    #[arg(short, long, default_value_t = 1)]
    number: u8,
}

fn main() {
    let args = Args::parse();

    let mut sentence_typer = SentenceTyper::default();
}
