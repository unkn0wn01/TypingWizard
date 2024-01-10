#![allow(unused)]

use ratatui::prelude::*;
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

#[derive(Clone)]
pub enum Valid {
    InCorrect,
    Correct,
    Backspace,
}

#[derive(Clone)]
pub enum Window {
    Main,
    ResultTyp,
}

pub struct App<'a> {
    pub valid: Option<Valid>,
    pub input: String,
    pub incorrect: VecDeque<String>,
    pub correct: VecDeque<String>,
    pub inner_content: Vec<Span<'a>>,
    pub window: Option<Window>,
    pub chardone: VecDeque<char>,
}

impl<'a> App<'a> {
    pub fn new(input: String) -> Self {
        App {
            input,
            valid: None,
            incorrect: VecDeque::new(),
            correct: VecDeque::new(),
            inner_content: Vec::new(),
            window: Some(Window::Main),
            chardone: VecDeque::new(),
        }
    }
}
pub struct TypingSpeed {
    correct_words: f64,
    incorrect_word: f64,
    start_time: Option<Instant>,
    end_time: Option<Instant>,
    total_time: f64,
    accuracy: f64,
    wpm: f64,
    average_len: usize,
}
impl TypingSpeed {
    pub fn new() -> Self {
        TypingSpeed {
            correct_words: 0.0,
            incorrect_word: 0.0,
            start_time: None,
            end_time: None,
            total_time: 0.0,
            accuracy: 0.0,
            wpm: 0.0,
            average_len: 0,
        }
    }
    pub fn start_time(&mut self) {
        self.start_time = Some(Instant::now());
    }
    pub fn end_time(&mut self) {
        self.end_time = Some(Instant::now());
    }
    pub fn total_time(&mut self) {
        let total_time = (self.end_time.unwrap() - self.start_time.unwrap()).as_secs_f64() / 60.0;
        self.total_time = total_time;
    }
    pub fn inc_correct_word(&mut self) {
        self.correct_words = self.correct_words + 1.0;
    }
    pub fn inc_incorrect_word(&mut self) {
        self.incorrect_word = self.incorrect_word + 1.0;
    }
    pub fn cal_wpm(&mut self) {
        self.wpm = (((self.correct_words) / { 5.0 }) / self.total_time).round();
    }
    pub fn cal_accuracy(&mut self) {
        let total_words = self.correct_words + self.incorrect_word;
        let accuracy = (self.correct_words / total_words) * 100.0;
        self.accuracy = accuracy.round();
    }
    pub fn return_accuracy(&self) -> String {
        return self.accuracy.to_string();
    }
    pub fn return_wpm(&self) -> String {
        return self.wpm.to_string();
    }
    pub fn set_avg_len(&mut self, len: usize) {
        self.average_len = len;
    }
    pub fn return_total_time(&self) -> String {
        let total_time =
            ((self.end_time.unwrap() - self.start_time.unwrap()).as_secs_f64()).round();
        total_time.to_string()
    }
}
