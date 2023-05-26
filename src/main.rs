mod modules;

use std::{
    env,
    io::{self, BufRead},
    os, path,
};

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
    // Get config location
    let config_path = match home::home_dir() {
        Some(mut home_path) => {
            home_path.push(".typers/config.yaml");
            home_path
        }
        None => {
            let mut home_path = env::current_dir().unwrap();
            home_path.push("/config.yaml");
            eprintln!("No home path found! Using config in {:?}", home_path);
            home_path
        }
    };

    // Check if config exists
    if !config_path.exists() {
        std::fs::create_dir_all(config_path.parent().unwrap()).unwrap();
        let mut default_config_path = env::current_dir().unwrap();
        default_config_path.push("default_config.yaml");
        std::fs::copy(default_config_path.to_owned(), config_path.to_owned()).expect(&format!(
            "Could not copy the default config from {:?} to {:?}",
            default_config_path, config_path
        ));
        println!(
            "Default config copied to {:?}",
            config_path.parent().unwrap()
        );
    }

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

    match sentence_typer.get_accuracy() {
        x if x <= 20.0 => println!("You could do better for sure!"),
        x if x <= 40.0 => println!("Not great, you can try again next time!"),
        x if x <= 60.0 => println!("Not too bad, but you can still be better!"),
        x if x <= 80.0 => println!("Quite nice! But there is still some room for improvement!"),
        x if x <= 95.0 => println!("Good job! Not too far from perfect!"),
        x if x < 100.0 => println!("Wow! Almost perfect! Great job!"),
        _ => println!("Perfect! You made no mistakes! Go grab yourself a cookie!"),
    }
}
