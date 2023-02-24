use crate::tile::TileType;
use macroquad::input::{is_key_pressed, is_mouse_button_pressed, mouse_position, KeyCode, MouseButton};
use hecs::World;
use std::collections::HashMap;

const INSERT: KeyCode = KeyCode::I;
const DELETE: KeyCode = KeyCode::D;
const SAVE: KeyCode = KeyCode::S;

const GROUND: KeyCode = KeyCode::Q;
const WALL: KeyCode = KeyCode::W;
const DOOR: KeyCode = KeyCode::E;

const LAYER1: KeyCode = KeyCode::Key1;
const LAYER2: KeyCode = KeyCode::Key2;

pub enum EditorMode {
    Insert,
    Delete,
}

pub struct Editor {
    pub mode: EditorMode,
    pub cur_tile_type: TileType,
    pub cur_layer: usize,
    pub tile_map: [HashMap<(u32, u32), TileType>; 3],
}

impl Editor {
    pub fn new() -> Self {
        Self {
            mode: EditorMode::Insert,
            cur_tile_type: TileType::Ground,
            cur_layer: 0,
            tile_map: [HashMap::new(), HashMap::new(), HashMap::new()],
        }
    }
}

pub fn system_update(editor: &mut Editor, world: &mut World) {
    if is_key_pressed(INSERT) {
        editor.mode = EditorMode::Insert;
    } else if is_key_pressed(DELETE) {
        editor.mode = EditorMode::Delete
    }

    if is_key_pressed(GROUND) {
        println!("ground tile selected");
        editor.cur_tile_type = TileType::Ground;
    } else if is_key_pressed(WALL) {
        match editor.cur_tile_type {
            TileType::Wall => {
                println!("side wall tile selected");
                editor.cur_tile_type = TileType::SideWall;
            }
            TileType::SideWall => {
                println!("gateway wall tile selected");
                editor.cur_tile_type = TileType::GatewayWall;
            }
            _ => {
                println!("wall tile selected");
                editor.cur_tile_type = TileType::Wall;
            }
        }
    }

    if is_key_pressed(DOOR) {
        match editor.cur_tile_type {
            TileType::Door => {
                println!("side door tile selected");
                editor.cur_tile_type = TileType::SideDoor;
            }
            _ => {
                println!("door tile selected");
                editor.cur_tile_type = TileType::Door;
            }
        }
    }

    if is_key_pressed(LAYER1) {
        println!("changed to layer 1");
        editor.cur_layer = 0;
    } else if is_key_pressed(LAYER2) {
        println!("changed to layer 2");
        editor.cur_layer = 1;
    }

    if is_key_pressed(SAVE) {
        todo!("save the tile map");
    }
}


pub fn system_handle_mouse_click(editor: &mut Editor, world: &mut World) {
    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_pos = mouse_position();

        match editor.mode {
            EditorMode::Insert => {
                let tile_pos = (
                    (((mouse_pos.0 / editor.cur_tile_type.size().0).floor())  * editor.cur_tile_type.size().0),
                    ((mouse_pos.1 / editor.cur_tile_type.size().1).floor()) * editor.cur_tile_type.size().1 ,
                );
                editor.tile_map[editor.cur_layer]
                    .insert((tile_pos.0 as u32, tile_pos.1 as u32), editor.cur_tile_type.clone());
                editor.cur_tile_type.spawn_tile(world, (tile_pos.0, tile_pos.1))
            }
            EditorMode::Delete => {
                // self.tile_map
                //     .remove_tile(self.cur_layer, tile_pos.0 as u32, tile_pos.1 as u32);
            }
        }
    }
}
