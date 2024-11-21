use crate::ui::ui;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{prelude::Backend, Terminal};

use core::fmt;
use std::collections::HashMap;

pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

pub enum CurrentEditing {
    Key,
    Value,
}

pub struct App {
    pub key_input: String,
    pub value_input: String,
    pub pairs: HashMap<String, String>,
    pub curr_scrn: CurrentScreen,
    pub curr_edtng: Option<CurrentEditing>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            curr_scrn: CurrentScreen::Main,
            curr_edtng: None,
        }
    }
}

impl fmt::Display for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = serde_json::to_string(&self.pairs).map_err(|_| fmt::Error)?;
        write!(f, "{}", output)
    }
}

impl App {
    fn reset_input(&mut self) {
        self.key_input = String::new();
        self.value_input = String::new();
        self.curr_edtng = None;
    }

    pub fn save_key_value(&mut self) {
        self.pairs
            .insert(self.key_input.clone(), self.value_input.clone());

        self.reset_input();
    }

    /// Else condition basically `starts` editing
    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.curr_edtng {
            match edit_mode {
                CurrentEditing::Key => self.curr_edtng = Some(CurrentEditing::Value),
                CurrentEditing::Value => self.curr_edtng = Some(CurrentEditing::Key),
            }
        } else {
            self.curr_edtng = Some(CurrentEditing::Key)
        }
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> std::io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;
        if let Event::Key(key) = event::read()? {
            dbg!(key.code);
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.curr_scrn {
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('q') => return Ok(true),
                    KeyCode::Char('n') => return Ok(false),
                    _ => {}
                },
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('e') => {
                        app.curr_scrn = CurrentScreen::Editing;
                        app.curr_edtng = Some(CurrentEditing::Key);
                    }
                    KeyCode::Char('q') => {
                        app.curr_scrn = CurrentScreen::Exiting;
                    }
                    _ => {}
                },
                CurrentScreen::Editing => match key.kind {
                    KeyEventKind::Press => match key.code {
                        KeyCode::Enter => {
                            if let Some(editing) = &app.curr_edtng {
                                match editing {
                                    CurrentEditing::Key => {
                                        app.curr_edtng = Some(CurrentEditing::Value)
                                    }
                                    CurrentEditing::Value => {
                                        app.save_key_value();
                                        app.curr_scrn = CurrentScreen::Main;
                                    }
                                }
                            }
                        }
                        KeyCode::Backspace => {
                            if let Some(editing) = &app.curr_edtng {
                                match editing {
                                    CurrentEditing::Key => app.key_input.pop(),
                                    CurrentEditing::Value => app.value_input.pop(),
                                };
                            }
                        }
                        KeyCode::Esc => {
                            app.curr_scrn = CurrentScreen::Main;
                            app.curr_edtng = None;
                        }
                        KeyCode::Tab => app.toggle_editing(),
                        KeyCode::Char(x) => {
                            if let Some(editing) = &app.curr_edtng {
                                match editing {
                                    CurrentEditing::Key => app.key_input.push(x),
                                    CurrentEditing::Value => app.value_input.push(x),
                                };
                            }
                        }
                        _ => (),
                    },
                    _ => (),
                },
            }
        }
    }
}
