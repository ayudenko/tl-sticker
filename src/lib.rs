use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub mod command;
pub mod sticker;

use command::Command;
use sticker::Stickers;

pub struct App {
    mode: Mode,
    buffer: Vec<char>,
    stickers: Stickers,
}

impl App {
    pub fn new(mode: Mode, buffer: Vec<char>, stickers: Stickers) -> Self {
        Self {
            mode,
            buffer,
            stickers,
        }
    }

    pub fn run(&mut self) {
        let stdin = stdin();

        let mut stdout = stdout().into_raw_mode().unwrap();

        Command::print_stickers_list(&self.stickers);
        stdout.flush().unwrap();

        for c in stdin.keys() {
            write!(
                stdout,
                r#"{}{}"#,
                termion::cursor::Goto(1, 1),
                termion::clear::All
            )
            .unwrap();

            match c.unwrap() {
                Key::Ctrl('q') => break,
                Key::Char(char) => match self.mode {
                    Mode::Command => match char {
                        'q' => break,
                        'l' => Command::print_stickers_list(&self.stickers),
                        'n' => Command::new_sticker_name_request(&mut self.mode),
                        _ => (),
                    },
                    Mode::WaitingForStickerName => {
                        match char {
                            '\n' => Command::finish_input(
                                &mut self.buffer,
                                &mut self.mode,
                                &mut self.stickers,
                            ),
                            _ => {
                                self.buffer.push(char);
                                write!(
                                    stdout,
                                    r#"{}{}New Sticker name: {}"#,
                                    termion::cursor::Goto(1, 1),
                                    termion::clear::All,
                                    self.buffer.iter().collect::<String>(),
                                )
                                .unwrap();
                            }
                        };
                    }
                    Mode::None => (),
                },
                _ => (),
            }

            stdout.flush().unwrap();
        }
    }
}

pub enum Mode {
    Command,
    None,
    WaitingForStickerName,
}
