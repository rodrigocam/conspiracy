use crate::{asset::Assets, tile::{TileType, TileMap}};
use macroquad::input::{
    is_key_pressed, is_mouse_button_pressed, mouse_position, KeyCode, MouseButton,
};

const INSERT: KeyCode = KeyCode::I;
const DELETE: KeyCode = KeyCode::D;
const SAVE: KeyCode = KeyCode::S;

const GROUND: KeyCode = KeyCode::G;
const WALL: KeyCode = KeyCode::W;

const LAYER1: KeyCode = KeyCode::Key1;
const LAYER2: KeyCode = KeyCode::Key2;

pub enum EditorMode {
    Insert,
    Delete,
}

pub struct Editor {
    mode: EditorMode,
    cur_tile_type: TileType,
    cur_layer: usize,
    tile_map: TileMap,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            mode: EditorMode::Insert,
            cur_tile_type: TileType::Ground,
            cur_layer: 0,
            tile_map: TileMap::new(20, 15),
        }
    }

    pub fn update(&mut self) {
        if is_key_pressed(INSERT) {
            self.mode = EditorMode::Insert;
        } else if is_key_pressed(DELETE) {
            self.mode = EditorMode::Delete
        }

        if is_key_pressed(GROUND) {
            self.cur_tile_type = TileType::Ground;
        } else if is_key_pressed(WALL) {
            self.cur_tile_type = TileType::Wall;
        }

        if is_key_pressed(LAYER1) {
            self.cur_layer = 0;
        } else if is_key_pressed(LAYER2) {
            self.cur_layer = 1;
        }

        self.handle_mouse_click();

        if is_key_pressed(SAVE) {
            todo!("save the tile map");
        }
    }

    pub fn draw(&self, assets: &Assets) {
        self.tile_map.draw(assets);
    }

    fn handle_mouse_click(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();

            match self.mode {
                EditorMode::Insert => {
                    self.tile_map
                        .new_tile(
                            self.cur_layer,
                            mouse_pos,
                            self.cur_tile_type.clone(),
                        )
                        .unwrap();
                }
                EditorMode::Delete => {
                    // self.tile_map
                    //     .remove_tile(self.cur_layer, tile_pos.0 as u32, tile_pos.1 as u32);
                }
            }
        }
    }
}
