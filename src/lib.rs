use std::{fmt::Display, path::Path, time::Instant};

use colored::Colorize;
use console::{Key, Term};
use reqwest::StatusCode;
use serde_json::Value;

pub trait Sourceable {
    fn get_new_sentence(&self) -> Result<String, String>;
}

#[derive(Clone, PartialEq)]
enum TypedAs {
    Current,
    Pending,
    Correct,
    Wrong,
    Corrected,
}

pub struct SentenceTyper {
    contents: String,
    typed_arr: Vec<TypedAs>,
    current_idx: usize,
    errors: u32,
    typed_chars: u32,
    typed_words: u16,
    start_time: Instant,
}

impl SentenceTyper {
    pub fn new(source: &impl Sourceable) -> Self {
        match source.get_new_sentence() {
            Ok(contents) => {
                let typed_arr_len = contents.len();
                let mut sen = SentenceTyper {
                    contents,
                    typed_arr: vec![TypedAs::Pending; typed_arr_len],
                    current_idx: 0,
                    errors: 0,
                    typed_chars: 0,
                    typed_words: 0,
                    start_time: Instant::now(),
                };
                *sen.typed_arr.get_mut(0).unwrap() = TypedAs::Current;
                sen
            }
            Err(err) => SentenceTyper::error(err),
        }
    }

    pub fn get_next_sentence(&mut self, source: &impl Sourceable) {
        match source.get_new_sentence() {
            Ok(contents) => {
                let typed_arr_len = contents.len();
                self.contents = contents;
                self.typed_arr = vec![TypedAs::Pending; typed_arr_len];
                self.current_idx = 0;
                *self.typed_arr.get_mut(0).unwrap() = TypedAs::Current;
            }
            Err(err) => panic!(),
        }
    }

    fn error(error_msg: impl ToString) -> Self {
        let typed_arr_len = error_msg.to_string().len();
        SentenceTyper {
            contents: error_msg.to_string(),
            typed_arr: vec![TypedAs::Wrong; typed_arr_len],
            current_idx: typed_arr_len,
            ..Default::default()
        }
    }

    fn type_next_char(&mut self) {
        let desired_char = self.contents.as_bytes().get(self.current_idx).unwrap();
        let stdout = Term::buffered_stdout();
        if let Ok(key) = stdout.read_key() {
            match key {
                Key::Escape => self.current_idx = self.contents.len(),
                Key::Char(c) => {
                    if c == *desired_char as char {
                        if *self.typed_arr.get(self.current_idx).unwrap() == TypedAs::Wrong {
                            *self.typed_arr.get_mut(self.current_idx).unwrap() = TypedAs::Corrected;
                        } else {
                            *self.typed_arr.get_mut(self.current_idx).unwrap() = TypedAs::Correct;
                        }
                        if c == ' ' {
                            self.typed_words += 1;
                        }
                        self.typed_chars += 1;
                        self.current_idx += 1;
                        match self.typed_arr.get_mut(self.current_idx) {
                            Some(s) => *s = TypedAs::Current,
                            None => (), // End of the sentence
                        }
                    } else {
                        self.errors += 1;
                        *self.typed_arr.get_mut(self.current_idx).unwrap() = TypedAs::Wrong;
                    }
                }
                _ => (),
            }
        }
    }

    fn print_prompt(&self) {
        print!("\x1B[2J\x1B[1;1H");
        println!("{}", self);
        println!(
            "Errors: {} | Accuracy: {:.02}% | WPM: {:.02}",
            self.errors,
            self.get_accuracy(),
            60.0 * self.typed_words as f32 / self.start_time.elapsed().as_secs_f32()
        );
    }

    fn get_accuracy(&self) -> f32 {
        (100.0 - (self.errors as f32 * 100.0 / self.typed_chars as f32)).max(0.0)
    }

    pub fn type_sentences(&mut self, n_of_sentences: u8, sentence_source: &impl Sourceable) {
        for _ in 0..n_of_sentences {
            while self.current_idx != self.contents.len() {
                self.print_prompt();
                self.type_next_char();
            }
            self.print_prompt();
            self.get_next_sentence(sentence_source);
        }
    }
}

impl Display for SentenceTyper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.contents
            .chars()
            .into_iter()
            .enumerate()
            .map(|(idx, char)| match self.typed_arr.get(idx).unwrap() {
                TypedAs::Pending => write!(f, "{}", char.to_string()),
                TypedAs::Correct => write!(f, "{}", char.to_string().bold().green()),
                TypedAs::Wrong => write!(f, "{}", char.to_string().bold().red().reversed()),
                TypedAs::Current => write!(f, "{}", char.to_string().reversed()),
                TypedAs::Corrected => {
                    write!(f, "{}", char.to_string().bold().bright_red().underline())
                }
            })
            .collect()
    }
}

impl Default for SentenceTyper {
    fn default() -> Self {
        Self {
            contents: Default::default(),
            typed_arr: Default::default(),
            current_idx: Default::default(),
            errors: Default::default(),
            typed_words: Default::default(),
            typed_chars: Default::default(),
            start_time: Instant::now(),
        }
    }
}

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

pub struct TextFileSource {
    file_path: String,
}

impl TextFileSource {
    pub fn from_file(file_path: impl ToString) -> Self {
        TextFileSource {
            file_path: file_path.to_string(),
        }
    }
}

impl Sourceable for TextFileSource {
    fn get_new_sentence(&self) -> Result<String, String> {
        todo!()
    }
}
