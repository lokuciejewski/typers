use reqwest::StatusCode;
use serde_json::Value;

use super::Sourceable;

pub struct WikipediaSource {
    http_address: String,
}

impl Default for WikipediaSource {
    fn default() -> Self {
        Self {
            http_address: "https://en.wikipedia.org/api/rest_v1/page/random/summary".to_owned(),
        }
    }
}

impl Sourceable for WikipediaSource {
    fn get_new_sentence(&self) -> Result<String, String> {
        match reqwest::blocking::get(&self.http_address) {
            Ok(resp) => {
                if resp.status() == StatusCode::OK {
                    let obj: Value = serde_json::from_str(resp.text().unwrap().as_str()).unwrap();
                    let extract: String = obj
                        .get("extract")
                        .unwrap()
                        .to_string()
                        .strip_prefix("\"")
                        .unwrap()
                        .strip_suffix("\"")
                        .unwrap()
                        .replace("\\", "")
                        .to_string();
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
