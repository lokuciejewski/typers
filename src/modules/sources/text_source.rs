use rand::{seq::SliceRandom, thread_rng};

use super::Sourceable;

pub struct TextSource {
    sentences: Vec<String>,
}

impl TextSource {
    pub fn new(text: String, separator: char) -> Self {
        TextSource {
            sentences: text
                .split_inclusive(separator)
                .map(|s| s.trim().to_string())
                .collect(),
        }
    }
}

impl Sourceable for TextSource {
    fn get_new_sentence(&self) -> Result<String, String> {
        match self.sentences.choose(&mut thread_rng()) {
            Some(s) => Ok(s.to_owned()),
            None => Err("Could not get a sentence!".to_string()),
        }
    }
}
