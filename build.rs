use std::{env, path::Path};

use home::home_dir;

fn main() {
    let config_path = match home_dir() {
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
    todo!()
}
