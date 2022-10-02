use std::io::{stdout, Write};
use termion::raw::IntoRawMode;

use crate::sticker::{Sticker, Stickers};
use crate::Mode;

pub struct Command;

impl Command {
    pub fn print_stickers_list(stickers: &Stickers) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        if !stickers.is_empty() {
            write!(
                stdout,
                r#"{}{}{}"#,
                termion::cursor::Goto(1, 1),
                termion::clear::All,
                stickers
            )
            .unwrap();
        } else {
            write!(
                stdout,
                r#"{}{}No stickers found!"#,
                termion::cursor::Goto(1, 1),
                termion::clear::All
            )
            .unwrap();
        }
    }

    pub fn new_sticker_name_request(mode: &mut Mode) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(
            stdout,
            r#"{}{}New Sticker name: "#,
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();
        *mode = Mode::WaitingForStickerName;
    }

    pub fn finish_input(buffer: &mut Vec<char>, mode: &mut Mode, stickers: &mut Stickers) {
        if !buffer.is_empty() {
            stickers.push(Sticker::new(buffer.iter().collect::<String>()));
            buffer.clear();
            *mode = Mode::Command;
        }
    }
}
