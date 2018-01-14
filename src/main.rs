extern crate tcod;

use std::cmp;
use tcod::console::*;
use tcod::colors;

const SCREEN_WIDTH : i32 = 80;
const SCREEN_HEIGHT : i32 = 50;
const FPS_LIMIT : i32 = 20;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;
const COLOR_DARK_WALL: colors::Color = colors::Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: colors::Color = colors::Color { r: 50, g: 50, b: 150 };

type Map = Vec<Vec<Tile>>;

#[derive(Clone, Copy, Debug)]
struct Tile {
    blocked : bool,
    block_sight : bool,
}

impl Tile {
    pub fn ground() -> Self {
        Tile { blocked: false, block_sight: false }
    }

    pub fn wall() -> Self {
        Tile { blocked: true, block_sight: true }
    }
}

#[derive(Clone, Copy, Debug)]
struct Rect {
    x1 : i32,
    y1 : i32,
    x2 : i32,
    y2 : i32,
}

impl Rect {
    pub fn new(x : i32, y : i32, w : i32, h : i32) -> Self {
        Rect { x1: x, y1: y, x2: x + w, y2: y + h }
    }
}

#[derive(Debug)]
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

    pub fn move_by(&mut self, dx : i32, dy : i32, map : &Map) {
        if !map[(self.x + dx) as usize][(self.y + dy) as usize].blocked {
            self.x += dx;
            self.y += dy;
        }
    }

    pub fn draw(&self, con : &mut Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }

    pub fn clear(&self, con: &mut Console) {
        con.put_char(self.x, self.y, ' ', BackgroundFlag::None);
    }
}

fn make_map() -> Map {
    let mut map = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    let room1 = Rect::new(20, 15, 10, 15);
    let room2 = Rect::new(50, 15, 10, 15);
    create_room(room1, &mut map);
    create_room(room2, &mut map);
    create_h_tunnel(25, 55, 23, &mut map);

    map
}
fn create_room(room : Rect, map : &mut Map) {
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize] = Tile::ground();
        }
    }
}

fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
        map[x as usize][y as usize] = Tile::ground();
    }
}

fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
    for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
        map[x as usize][y as usize] = Tile::ground();
    }
}

fn render_all(root: &mut Root, con: &mut Offscreen, entities: &[Entity], map: &Map) {
    for entity in entities {
        entity.draw(con);
    }

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let wall = map[x as usize][y as usize].block_sight;
            if wall {
                con.set_char_background(x, y, COLOR_DARK_WALL, BackgroundFlag::Set);
            } else {
                con.set_char_background(x, y, COLOR_DARK_GROUND, BackgroundFlag::Set);
            }
        }
    }

    blit(con, (0, 0), (MAP_WIDTH, MAP_HEIGHT), root, (0, 0), 1.0, 1.0);
    root.flush();

    for entity in entities {
        entity.clear(con)
    }
}

fn handle_keys(root : &mut Root, player : &mut Entity, map : &Map) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = root.wait_for_keypress(true);
    match key {
        Key { code: Up, .. } => player.move_by(0, -1, map),
        Key { code: Down, .. } => player.move_by(0, 1, map),
        Key { code: Left, .. } => player.move_by(-1, 0, map),
        Key { code: Right, .. } => player.move_by(1, 0, map),
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
    let mut con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);

    tcod::system::set_fps(FPS_LIMIT);

    let mut map = make_map();
    let player = Entity::new(25, 23, '@', colors::WHITE);
    let npc = Entity::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, 'O', colors::YELLOW);
    let mut entities = [player, npc];

    while !root.window_closed() {
        for entity in &entities {
            entity.draw(&mut con)
        }
        
        render_all(&mut root, &mut con, &entities, &map);

        let exit = handle_keys(&mut root, &mut entities[0], &map);
        if exit { break }

    }
}
