use iso639_1::from_iso639_1;
use rand::{seq::SliceRandom, thread_rng};
use reqwest::StatusCode;
use serde_json::Value;

use super::{Configurable, Sourceable};

pub struct WikipediaSource {
    http_address: String,
    languages: Vec<String>,
}

impl Default for WikipediaSource {
    fn default() -> Self {
        Self {
            http_address: "https://$lang.wikipedia.org/api/rest_v1/page/random/summary".to_owned(),
            languages: vec!["en".to_string()],
        }
    }
}

impl Configurable for WikipediaSource {
    fn from_config(config: crate::modules::config::Config) -> Self {
        let ws = WikipediaSource {
            http_address: "https://$lang.wikipedia.org/api/rest_v1/page/random/summary".to_owned(),
            languages: config
                .wikipedia
                .languages
                .into_iter()
                .map(|v| {
                    let mut t = v.to_string();
                    t.retain(|c| c.is_alphabetic());
                    t
                })
                .filter(|code| from_iso639_1(code).is_ok())
                .collect(),
        };
        if ws.languages.is_empty() {
            panic!("No languages parsed successfully! Please check the config and try again.");
        };
        ws
    }
}

impl Sourceable for WikipediaSource {
    fn get_new_sentence(&self) -> Result<String, String> {
        let url = &self
            .http_address
            .replace("$lang", self.languages.choose(&mut thread_rng()).unwrap());
        match reqwest::blocking::get(url) {
            Ok(resp) => {
                if resp.status() == StatusCode::OK {
                    let obj: Value = serde_json::from_str(resp.text().unwrap().as_str()).unwrap();
                    let extract: String = obj
                        .get("extract")
                        .unwrap()
                        .to_string()
                        .strip_prefix('\"')
                        .unwrap()
                        .strip_suffix('\"')
                        .unwrap()
                        .replace('\\', "");
                    Ok(any_ascii::any_ascii(extract.as_str()))
                } else {
                    Err(format!(
                        "Received status code {} instead of {}",
                        resp.status(),
                        StatusCode::OK
                    ))
                }
            }
            Err(err) => Err(format!("Error ocurred while sending request: {err}")),
        }
    }
}
