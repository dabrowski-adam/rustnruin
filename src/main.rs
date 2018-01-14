extern crate tcod;

use tcod::console::*;
use tcod::colors;

const SCREEN_WIDTH : i32 = 80;
const SCREEN_HEIGHT : i32 = 50;
const FPS_LIMIT : i32 = 20;

struct Entity {
    x : i32,
    y : i32,
    char : char,
    color : colors::Color,
}

impl Entity {
    pub fn new(x : i32, y : i32, char : char, color : colors::Color) -> Self {
        Entity {
            x: x,
            y: y,
            char: char,
            color: color,
        }
    }

    pub fn move_by(&mut self, dx : i32, dy : i32) {
        self.x += dx;
        self.y += dy;
    }

    pub fn draw(&self, con : &mut Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }

    pub fn clear(&self, con: &mut Console) {
        con.put_char(self.x, self.y, ' ', BackgroundFlag::None);
    }
}

fn handle_keys(root : &mut Root, player : &mut Entity) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = root.wait_for_keypress(true);
    match key {
        Key { code: Up, .. } => player.move_by(0, -1),
        Key { code: Down, .. } => player.move_by(0, 1),
        Key { code: Left, .. } => player.move_by(-1, 0),
        Key { code: Right, .. } => player.move_by(1, 0),
        Key { code: Enter, alt: true, .. } => {
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
        },
        Key { code: Escape, .. } => return true,
        _ => {},
    }

    false
}

fn main() {
    let mut root = Root::initializer()
        .title("Rust'n'Ruin")
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .init();
    let mut con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    tcod::system::set_fps(FPS_LIMIT);

    

    let player = Entity::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', colors::WHITE);
    let npc = Entity::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, '@', colors::YELLOW);
    let mut entities = [player, npc];
    while !root.window_closed() {
        for entity in &entities {
            entity.draw(&mut con)
        }
        
        blit(&con, (0, 0), (SCREEN_WIDTH, SCREEN_HEIGHT), &mut root, (0, 0), 1.0, 1.0);
        root.flush();

        for entity in &entities {
            entity.clear(&mut con)
        }

        let exit = handle_keys(&mut root, &mut entities[0]);
        if exit { break }

    }
}
