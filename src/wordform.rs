//Note to reader
// This file will be used for Creating sentence for typing test
// User can provide length and Mode
// enum Mode has Three modes Easy,Medium,Hard
//Sentence for test will be given on bases of mode(by default 'Easy') and length(by default 10)

#![allow(unused)]

const EASY_FILE_WORDLIST: &str = include_str!("../wordlist/top5000.txt");
const MEDIUM_FILE_WORDLIST: &str = include_str!("../wordlist/common_misspelled.txt");
const HARD_WORD_WORDLIST: &str = include_str!("../wordlist/hard_words.txt");

#[derive(Clone, Debug)]
pub enum Mode {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug)]
pub struct WordSentence {
    pub sentence: String,
    length: u16,
    pub mode: Mode, //set pub
    pub average_len: usize,
}

use rand::{thread_rng, Rng};
use std::{
    collections::HashSet,
    fs::OpenOptions,
    io::{BufReader, Read},
    path::Path,
};
use Mode::{Easy, Hard, Medium};
impl WordSentence {
    pub fn new() -> Self {
        WordSentence {
            sentence: String::new(),
            length: 0,
            mode: Easy, //By default Easy mode is selected
            average_len: 0,
        }
    }
    pub fn generate_sentence(&mut self, len: u16) {
        match self.mode {
            // Easy Mode contains 70% easy words and rest are medium
            Easy => {
                let easy_words_len = ((70.0 / 100.0) * (len as f32)) as u16;
                let medium_words_len = len - easy_words_len;
                //vec
                let easy_vec = self
                    .add_words(easy_words_len, EASY_FILE_WORDLIST)
                    .expect("Error occured while generating word");
                let medium_vec = self
                    .add_words(medium_words_len, MEDIUM_FILE_WORDLIST)
                    .expect("Error occured while generating words");
                let mut rand = thread_rng();
                let mut sentence = String::new();
                let mut total_vec = Vec::new();
                total_vec.extend(easy_vec);
                total_vec.extend(medium_vec);
                while total_vec.len() != 0 {
                    let total_vec_len = total_vec.len() as i32;
                    let random_number = thread_rng().gen_range(0..=total_vec_len - 1);
                    let word = total_vec[random_number as usize].clone();
                    total_vec.remove(random_number as usize);
                    sentence.push_str(&word);
                    sentence.push_str(" ");
                }
                self.sentence = sentence.trim().to_string();
                self.length = len
            }
            Medium => {
                let medium_words_len = ((70.0 / 100.0) * (len as f32)) as u16;
                let easy_words_len = len - medium_words_len;
                //vec
                let medium_vec = self
                    .add_words(medium_words_len, MEDIUM_FILE_WORDLIST)
                    .expect("Error occured while generating words");
                let easy_vec = self
                    .add_words(easy_words_len, EASY_FILE_WORDLIST)
                    .expect("Error occured while generating word");
                let mut rand = thread_rng();
                let mut sentence = String::new();
                let mut total_vec = Vec::new();
                total_vec.extend(medium_vec);
                total_vec.extend(easy_vec);
                while total_vec.len() != 0 {
                    let total_vec_len = total_vec.len() as i32;
                    let random_number = thread_rng().gen_range(0..=total_vec_len - 1);
                    let word = total_vec[random_number as usize].clone();
                    total_vec.remove(random_number as usize);
                    sentence.push_str(&word);
                    sentence.push_str(" ");
                }
                self.sentence = sentence.trim().to_string();
                self.length = len
            }
            Hard => {
                let hard_words_len = ((70.0 / 100.0) * (len as f32)) as u16;
                let medium_words_len = len - hard_words_len;
                //vec
                let hard_vec = self
                    .add_words(hard_words_len, HARD_WORD_WORDLIST)
                    .expect("Error occured while generating word");
                let medium_vec = self
                    .add_words(medium_words_len, MEDIUM_FILE_WORDLIST)
                    .expect("Error occured while generating words");
                let mut rand = thread_rng();
                let mut sentence = String::new();
                let mut total_vec = Vec::new();
                total_vec.extend(medium_vec);
                total_vec.extend(hard_vec);
                while total_vec.len() != 0 {
                    let total_vec_len = total_vec.len() as i32;
                    let random_number = thread_rng().gen_range(0..=total_vec_len - 1);
                    let word = total_vec[random_number as usize].clone();
                    total_vec.remove(random_number as usize);
                    sentence.push_str(&word);
                    sentence.push_str(" ");
                }
                self.sentence = sentence.trim().to_string();
                self.length = len
            }
        }
    }
    fn add_words(
        &self,
        ran_len: u16,
        path: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        //Read file
        // let file = OpenOptions::new().read(true).open(path)?;
        let mut file_string = path;
        // let mut buffer = BufReader::new(&file);
        // buffer.read_to_string(&mut file_string);

        let mut return_vec: Vec<String> = Vec::new();
        let contained_number: HashSet<usize> = HashSet::new(); // word dont repeat
                                                               // Random thread_rng()
        let mut rand = thread_rng();
        let file_string_vec: Vec<_> = file_string.lines().collect();
        let file_string_vec_len = file_string_vec.len();

        loop {
            if return_vec.len() == ran_len as usize {
                break;
            }
            let random_number = thread_rng().gen_range(0..=file_string_vec_len - 1);
            if !contained_number.contains(&random_number) {
                return_vec.push(file_string_vec[random_number].to_string())
            }
        }

        Ok(return_vec)
    }

    pub fn average_len(&mut self) {
        let sentence_break: Vec<_> = self.sentence.split_whitespace().collect();
        let mut total_length = 0;
        sentence_break.iter().for_each(|word| {
            total_length += word.len();
        });
        self.average_len = total_length / sentence_break.len();
    }
}
