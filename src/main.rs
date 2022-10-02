extern crate termion;

use std::fmt;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    println!("Hello, Dude!");

    let stdin = stdin();

    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        r#"{}{}ctrl + q to exit"#,
        termion::cursor::Goto(1, 1),
        termion::clear::All
    )
    .unwrap();
    stdout.flush().unwrap();

    let mut mode = Mode::Command;
    let mut input_text_buffer: Vec<char> = Vec::new();
    let mut stickers: Stickers = Stickers::init();

    for c in stdin.keys() {
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();

        match c.unwrap() {
            Key::Ctrl('n') => {
                write!(
                    stdout,
                    "{}{}",
                    termion::cursor::Goto(1, 1),
                    termion::clear::All
                )
                .unwrap();
                write!(stdout, "Sticker name: ").unwrap();
                mode = Mode::WaitingForStickerName;
            }
            Key::Ctrl('q') => break,
            Key::Ctrl('l') => {
                write!(
                    stdout,
                    "{}{}",
                    termion::cursor::Goto(1, 1),
                    termion::clear::All
                )
                .unwrap();
                if stickers.len() > 0 {
                    write!(stdout, "{}", stickers).unwrap();
                } else {
                    write!(stdout, "No stickers found!").unwrap();
                }
            }
            Key::Char(char) => match mode {
                Mode::Command => match char {
                    'q' => break,
                    'l' => {
                        write!(
                            stdout,
                            "{}{}",
                            termion::cursor::Goto(1, 1),
                            termion::clear::All
                        )
                        .unwrap();
                        if stickers.len() > 0 {
                            write!(stdout, "{}", stickers).unwrap();
                        } else {
                            write!(stdout, "No stickers found!").unwrap();
                        }
                    }
                    'n' => {
                        write!(
                            stdout,
                            "{}{}",
                            termion::cursor::Goto(1, 1),
                            termion::clear::All
                        )
                        .unwrap();
                        write!(stdout, "Sticker name: ").unwrap();
                        mode = Mode::WaitingForStickerName;
                    }
                    _ => (),
                },
                Mode::WaitingForStickerName => {
                    match char {
                        '\n' => {
                            if !input_text_buffer.is_empty() {
                                stickers.push(Sticker::new(
                                    input_text_buffer.iter().collect::<String>(),
                                ));
                                input_text_buffer.clear();
                                mode = Mode::Command;
                            }
                        }
                        _ => {
                            input_text_buffer.push(char);
                            write!(
                                stdout,
                                "{}{}",
                                termion::cursor::Goto(1, 1),
                                termion::clear::All
                            )
                            .unwrap();
                            write!(
                                stdout,
                                "Sticker name: {}",
                                input_text_buffer.iter().collect::<String>()
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

enum Mode {
    Command,
    None,
    WaitingForStickerName,
}

#[derive(Debug)]
struct Sticker {
    title: String,
    items: Vec<StickerItem>,
}

impl Sticker {
    fn new(title: String) -> Self {
        Self {
            title,
            items: Vec::<StickerItem>::new(),
        }
    }
}

impl fmt::Display for Sticker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.title)
    }
}

#[derive(Debug)]
struct StickerItem {
    text: String,
}

struct Stickers(pub Vec<Sticker>);

impl Stickers {
    fn init() -> Self {
        Self(Vec::new())
    }

    fn clear(&mut self) {
        *self = Stickers::init();
    }

    fn push(&mut self, sticker: Sticker) {
        self.0.push(sticker);
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl fmt::Display for Stickers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut stdout = stdout().into_raw_mode().unwrap();
        let mut line_number = 1;
        self.0.iter().fold(Ok(()), |result, sticker| {
            result.and_then(|_| {
                write!(stdout, "{}", termion::cursor::Goto(1, line_number)).unwrap();
                line_number += 1;
                writeln!(f, "Sticker: {}", sticker)
            })
        })
    }
}
