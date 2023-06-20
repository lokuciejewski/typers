mod modules;

use std::{
    env,
    fs::File,
    io::{self, BufRead, Read},
    path::Path,
};

use clap::{Parser, ValueHint};
use modules::{
    config::Config,
    sentence_typer::SentenceTyper,
    sources::{
        file_source::FileSource, text_source::TextSource, wiki_source::WikipediaSource,
        Configurable,
    },
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

impl Args {
    pub fn any(&self) -> bool {
        self.wikipedia || self.file_paths.is_some()
    }
}

fn main() {
    // Get config location
    let config_path = match home::home_dir() {
        Some(mut home_path) => {
            home_path.push(".typers/config.toml");
            home_path
        }
        None => {
            let mut home_path = env::current_dir().expect("Error while getting current directory!");
            home_path.push("/config.toml");
            eprintln!("No home path found! Using config in {:?}", home_path);
            home_path
        }
    };

    // Remove config file if in debug mode
    #[cfg(debug_assertions)]
    if config_path.exists() {
        std::fs::remove_file(&config_path).ok();
    }

    // Check if config exists
    if !config_path.exists() {
        std::fs::create_dir_all(config_path.parent().unwrap()).unwrap();
        let mut default_config_path =
            Path::new(&env::var("CARGO_HOME").unwrap_or("~/.cargo/".to_owned())).to_path_buf();
        default_config_path.push(Path::new("typers/default_config.toml"));
        std::fs::copy(&default_config_path, &config_path).unwrap_or_else(|_| {
            panic!(
                "Could not copy the default config from {:?} to {:?}",
                default_config_path, config_path
            );
        });
        println!("Default config copied to {:?}", config_path);
    }

    let mut conf_file = File::open(config_path)
        .expect("Could not open the config file! Verify your permissions and try again.");
    let mut contents = String::new();
    conf_file
        .read_to_string(&mut contents)
        .expect("Could not read the config file!");
    let config: Config = toml::from_str(&contents).expect("TOML parse error!");

    let mut sentence_typer = SentenceTyper::default();
    let mut args = Args::parse();

    // Use default args if none were provided
    if !args.any() {
        let mut default_args = config
            .application
            .default_args
            .split(' ')
            .collect::<Vec<&str>>();
        default_args.insert(0, "typers");
        args = Args::parse_from(default_args);
    }

    // Piped input
    if atty::isnt(atty::Stream::Stdin) {
        let stdin = io::stdin();
        let read_line: String = stdin.lock().lines().next().unwrap().expect("IO error!");
        let text_source = TextSource::new(read_line, '.');
        sentence_typer.add_source(text_source);
    }

    // Wikipedia
    if args.wikipedia {
        let wiki_source = WikipediaSource::from_config(config);
        sentence_typer.add_source(wiki_source);
    }

    // Files
    if let Some(mut v) = args.file_paths {
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
