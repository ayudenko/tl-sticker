extern crate termion;

use std::fmt;
use std::io::{stdout, Write};
use termion::raw::IntoRawMode;

pub struct Stickers(pub Vec<Sticker>);

impl Stickers {
    pub fn init() -> Self {
        Self(Vec::new())
    }

    pub fn clear(&mut self) {
        *self = Stickers::init();
    }

    pub fn push(&mut self, sticker: Sticker) {
        self.0.push(sticker);
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl fmt::Display for Stickers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut stdout = stdout().into_raw_mode().unwrap();
        let mut line_number = 1;
        self.0.iter().fold(Ok(()), |result, sticker| {
            result.and_then(|_| {
                write!(stdout, r#"{}"#, termion::cursor::Goto(1, line_number)).unwrap();
                line_number += 1;
                writeln!(f, "Sticker: {}", sticker)
            })
        })
    }
}

#[derive(Debug)]
pub struct Sticker {
    title: String,
    items: Vec<StickerItem>,
}

impl Sticker {
    pub fn new(title: String) -> Self {
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
