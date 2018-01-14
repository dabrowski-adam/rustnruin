extern crate tcod;

use tcod::console::*;
use tcod::colors;

const SCREEN_WIDTH : i32 = 80;
const SCREEN_HEIGHT : i32 = 50;
const FPS_LIMIT : i32 = 20;

fn main() {
    let mut root = Root::initializer()
        .title("Rust'n'Ruin")
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .init();

    tcod::system::set_fps(FPS_LIMIT);

    while !root.window_closed() {
        root.set_default_foreground(colors::WHITE);
        root.put_char(1, 1, '@', BackgroundFlag::None);
        root.flush();
        root.wait_for_keypress(true);
    }
}
