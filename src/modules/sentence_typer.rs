use std::{fmt::Display, time::Instant};

use colored::Colorize;
use console::{Key, Term};
use rand::{seq::SliceRandom, thread_rng};

use super::sources::Sourceable;

#[derive(Clone, PartialEq)]
enum TypedAs {
    Current,
    Pending,
    Correct,
    Wrong,
    Corrected,
}

pub struct SentenceTyper {
    current_contents: String,
    typed_arr: Vec<TypedAs>,
    current_idx: usize,
    errors: u32,
    typed_chars: u32,
    typed_words: u16,
    start_time: Instant,
    sources: Vec<Box<dyn Sourceable>>,
}

impl SentenceTyper {
    pub fn new(source: &impl Sourceable) -> Self {
        match source.get_new_sentence() {
            Ok(contents) => {
                let typed_arr_len = contents.len();
                let mut sen = SentenceTyper {
                    current_contents: contents,
                    typed_arr: vec![TypedAs::Pending; typed_arr_len],
                    current_idx: 0,
                    errors: 0,
                    typed_chars: 0,
                    typed_words: 0,
                    start_time: Instant::now(),
                    sources: vec![],
                };
                *sen.typed_arr.get_mut(0).unwrap() = TypedAs::Current;
                sen
            }
            Err(err) => SentenceTyper::error(err),
        }
    }

    pub fn type_sentences(&mut self, n_of_sentences: u8) {
        self.get_next_sentence();
        for _ in 0..n_of_sentences {
            while self.current_idx != self.current_contents.len() {
                self.print_prompt();
                self.type_next_char();
            }
            self.print_prompt();
            self.get_next_sentence();
        }
    }

    pub fn add_source(&mut self, source: impl Sourceable + 'static) {
        self.sources.push(Box::new(source));
    }

    fn get_next_sentence(&mut self) {
        match self.sources.choose(&mut thread_rng()) {
            Some(source) => match source.get_new_sentence() {
                Ok(contents) => {
                    let typed_arr_len = contents.len();
                    self.current_contents = contents;
                    self.typed_arr = vec![TypedAs::Pending; typed_arr_len];
                    self.current_idx = 0;
                    *self.typed_arr.get_mut(0).unwrap() = TypedAs::Current;
                }
                Err(_err) => panic!(),
            },
            None => {
                eprintln!("No source provided! Please provide source according to `typers --help`")
            }
        }
    }

    fn error(error_msg: impl ToString) -> Self {
        let typed_arr_len = error_msg.to_string().len();
        SentenceTyper {
            current_contents: error_msg.to_string(),
            typed_arr: vec![TypedAs::Wrong; typed_arr_len],
            current_idx: typed_arr_len,
            ..Default::default()
        }
    }

    fn type_next_char(&mut self) {
        let desired_char = self
            .current_contents
            .as_bytes()
            .get(self.current_idx)
            .unwrap();
        let stdout = Term::buffered_stdout();
        if let Ok(key) = stdout.read_key() {
            match key {
                Key::Escape => self.current_idx = self.current_contents.len(),
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
}

impl Display for SentenceTyper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.current_contents
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
            current_contents: Default::default(),
            typed_arr: Default::default(),
            current_idx: Default::default(),
            errors: Default::default(),
            typed_words: Default::default(),
            typed_chars: Default::default(),
            start_time: Instant::now(),
            sources: vec![],
        }
    }
}
