use serde::Deserialize;
use toml::value::Array;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub application: Application,
    pub wikipedia: Wikipedia,
}

#[derive(Deserialize, Debug)]
pub struct Application {
    pub default_args: String,
}

#[derive(Deserialize, Debug)]
pub struct Wikipedia {
    pub languages: Array,
}
