use std::io::stdout;

use crossterm::event::*;
use crossterm::terminal::LeaveAlternateScreen;
use crossterm::{event, terminal::disable_raw_mode};
use std::collections::VecDeque;

use app::{App, TypingSpeed};
use crossterm::{
    terminal::{enable_raw_mode, EnterAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use wordform::{Mode, WordSentence};
mod app;
mod config;
mod ui;
mod wordform;

use app::Valid;
use app::Window;
use ui::ui;
type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let mut word = WordSentence::new();
    word.mode = Mode::Easy;
    word.generate_sentence(10);
    word.average_len();

    let mut wpm = TypingSpeed::new();
    wpm.set_avg_len(word.average_len);

    let mut backend = Terminal::new(CrosstermBackend::new(stdout()))?;
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut app = App::new(word.sentence);
    let len = app.input.len();
    let mut typing_completed = false;
    let mut typing_started = true;

    loop {
        backend.draw(|f| ui(f, &mut app, &mut wpm))?;

        match app.window.clone() {
            Some(window) => match window {
                Window::Main => {
                    if app.inner_content.len() == len - 1 {
                        app.window = Some(Window::ResultTyp);
                        typing_completed = true;
                    }
                    if let Event::Key(key) = event::read()? {
                        if typing_started {
                            wpm.start_time();
                            typing_started = false;
                        }

                        if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL
                        {
                            break;
                        }
                        if key.code == KeyCode::Backspace {
                            if app.inner_content.len() != 0 {
                                app.valid = Some(Valid::Backspace);
                                continue;
                            }
                        }
                        let mut char_string: VecDeque<_> = app.input.chars().collect();
                        let char = char_string.pop_front();
                        let char = match char {
                            Some(c) => c,
                            None => panic!(""),
                        };
                        app.chardone.push_back(char);
                        if key.code == KeyCode::Char(char) {
                            app.input.remove(0);
                            app.correct.push_back(char.to_string());
                            app.valid = Some(Valid::Correct);
                            wpm.inc_correct_word();
                        } else {
                            char_string.push_front(char);
                            app.input.remove(0);
                            app.incorrect.push_back(char.to_string());
                            app.valid = Some(Valid::InCorrect);
                            wpm.inc_incorrect_word();
                        }
                    }
                }
                Window::ResultTyp => {
                    if let Event::Key(key) = event::read()? {
                        if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL
                        {
                            break;
                        }

                        // if key.code == KeyCode::Char('r') {
                        //     if let Some(_) = app.window.take() {
                        //         app.window = Some(Window::Main)
                        //     }
                        // }
                    }
                }
            },
            _ => (),
        }
        if typing_completed {
            wpm.end_time(); // Capture the end time when typing is done

            // Calculate the time difference between start and end time
            wpm.total_time();

            // Calculate WPM and accuracy based on correct words and total time taken
            wpm.cal_wpm();
            wpm.cal_accuracy();
            typing_completed = false;
        }
    }
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
