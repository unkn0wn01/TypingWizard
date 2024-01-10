use crate::config::COLOR_BACKGROUND;
use crate::config::COLOR_CORRECT_LETTERS;
use crate::config::COLOR_DEFAULT_LETTERS;
use crate::config::COLOR_INCORRECT_LETTERS;
use crate::config::RESULT_DEFAULT_COLOR;
use crate::App;
use crate::TypingSpeed;
use crate::Valid;
use crate::Window;
use ratatui::Frame;

use ratatui::prelude::*;
use ratatui::text::Line;
use ratatui::widgets::Paragraph;
use ratatui::widgets::*;
pub fn ui<'a>(f: &mut Frame, app: &mut App, wpm: &mut TypingSpeed) {
    match app.window.clone() {
        Some(window) => match window {
            Window::Main => {
                let chunks = Layout::new(
                    Direction::Vertical,
                    [Constraint::Max(15), Constraint::Min(5), Constraint::Max(15)],
                )
                .split(f.size());

                let n1 = Paragraph::new("").bg(COLOR_BACKGROUND);
                let n2 = Paragraph::new("").bg(COLOR_BACKGROUND);
                f.render_widget(n1, chunks[0]);
                f.render_widget(n2, chunks[2]);

                match app.valid.clone() {
                    Some(value) => match value {
                        Valid::Correct => {
                            let element = app.correct.pop_back();
                            let element = match element {
                                Some(e) => e,
                                None => String::from(""),
                            };
                            app.inner_content.insert(
                                0,
                                Span::styled(
                                    element.to_string(),
                                    Style::new()
                                        .fg(COLOR_CORRECT_LETTERS)
                                        .bold()
                                        .underlined()
                                        .not_dim(),
                                ),
                            )
                        }
                        Valid::InCorrect => {
                            let element = app.incorrect.pop_back();
                            let element = match element {
                                Some(e) => e,
                                None => String::from(""),
                            };
                            app.inner_content.insert(
                                0,
                                Span::styled(
                                    element.to_string(),
                                    Style::new()
                                        .fg(COLOR_INCORRECT_LETTERS)
                                        .bold()
                                        .underlined()
                                        .not_dim(),
                                ),
                            )
                        }

                        Valid::Backspace => {
                            app.inner_content.remove(0);
                            let char = app.chardone.pop_back().unwrap();
                            app.input.insert(0, char);
                        }
                    },
                    None => (),
                }
                let mut appcontent = vec![
                    Span::styled(
                        "|",
                        Style::default()
                            .not_bold()
                            .dim()
                            .rapid_blink()
                            .bg(Color::DarkGray),
                    ),
                    Span::raw(app.input.clone()).fg(COLOR_DEFAULT_LETTERS),
                ];

                for element in app.inner_content.iter() {
                    appcontent.insert(0, element.to_owned())
                }
                let text = vec![Line::from(appcontent)];
                let p = Paragraph::new(text)
                    .bg(COLOR_BACKGROUND)
                    .bold()
                    .alignment(Alignment::Center)
                    .wrap(Wrap { trim: true });
                f.render_widget(p, chunks[1]);
            }
            Window::ResultTyp => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(vec![
                        Constraint::Max(15),
                        Constraint::Min(5),
                        Constraint::Max(15),
                    ])
                    .split(f.size());
                let n1 = Paragraph::new("").bg(COLOR_BACKGROUND);
                let n3 = Paragraph::new("").bg(COLOR_BACKGROUND);
                f.render_widget(n1, chunks[0]);
                f.render_widget(n3, chunks[2]);

                let lines = vec![
                    Line::from(vec![
                        Span::raw("Took "),
                        Span::styled(
                            wpm.return_total_time(),
                            Style::default().bold().fg(Color::Green),
                        ),
                        Span::raw(" secs to complete"),
                    ]),
                    Line::from(vec![
                        Span::raw("Accuracy is "),
                        Span::styled(
                            wpm.return_accuracy(),
                            Style::default().bold().fg(Color::Green),
                        ),
                        Span::raw(" %"),
                    ]),
                    Line::from(vec![
                        Span::raw("Speed :  "),
                        Span::styled(wpm.return_wpm(), Style::default().bold().fg(Color::Green))
                            .underlined(),
                        Span::raw(" (word per minute)"),
                    ]),
                ];
                let n2 = Paragraph::new(lines)
                    .bold()
                    .fg(RESULT_DEFAULT_COLOR)
                    .alignment(Alignment::Center)
                    .bg(COLOR_BACKGROUND)
                    .wrap(Wrap { trim: true });
                f.render_widget(n2, chunks[1])
            }
        },
        _ => (),
    }
}
