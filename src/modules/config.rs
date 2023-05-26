use std::path::PathBuf;

use serde::Deserialize;
use toml::value::Array;

#[derive(Deserialize, Debug)]
pub struct Config {
    application: Application,
    wikipedia: Wikipedia,
}

#[derive(Deserialize, Debug)]
struct Application {
    default_args: String,
}

#[derive(Deserialize, Debug)]
struct Wikipedia {
    languages: Array,
}
