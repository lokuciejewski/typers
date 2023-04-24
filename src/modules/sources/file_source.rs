use std::{fs, io::Read, path::PathBuf};

use rand::{seq::SliceRandom, thread_rng};

use super::Sourceable;

pub struct FileSource {
    file_path: PathBuf,
    lines: Vec<String>,
}

impl FileSource {
    pub fn from_file(file_path: impl ToString) -> Result<Self, String> {
        let mut tfs = FileSource {
            file_path: PathBuf::from(file_path.to_string()),
            lines: vec![],
        };
        match tfs.load_source() {
            Ok(_) => Ok(tfs),
            Err(err) => Err(err),
        }
    }

    fn load_source(&mut self) -> Result<(), String> {
        // Try parsing by extension:
        // 1. JSON -> json list with sentences/text: [ "text1", "text2", ... ]
        // 2. Plain text -> plain text object where the '\n' denotes the end of each text/sentence
        // 3. YAML?
        // 4. CSV?
        println!("Trying to read file: {:?}", self.file_path);
        if self.file_path.exists() {
            match self.file_path.extension() {
                Some(extension) => match extension.to_str().unwrap() {
                    "json" => match fs::File::open(self.file_path.to_owned()) {
                        Ok(mut f) => {
                            let mut contents = String::new();
                            f.read_to_string(&mut contents).unwrap();
                            match serde_json::from_str(&contents) {
                                Ok(json) => {
                                    self.lines = json;
                                    Ok(())
                                }
                                Err(err) => Err(format!("JSON parse error: {:?}", err)),
                            }
                        }
                        Err(err) => Err(format!("File open error: {}", err)),
                    },
                    "yaml" | "yml" => todo!("yaml!"),
                    "csv" => todo!("csv!"),
                    "txt" => todo!("txt!"),
                    _ => todo!("other!"),
                },
                None => todo!("no extension!"),
            }
        } else {
            Err("File does not exist!".to_string())
        }
    }
}

impl Sourceable for FileSource {
    fn get_new_sentence(&self) -> Result<String, String> {
        match self.lines.choose(&mut thread_rng()) {
            Some(line) => Ok(line.to_owned()),
            None => todo!(),
        }
    }
}
