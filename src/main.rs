use tl_sticker::sticker::Stickers;
use tl_sticker::{App, Mode};

fn main() {
    let mut app = App::new(Mode::Command, Vec::<char>::new(), Stickers::init());
    app.run();
}
