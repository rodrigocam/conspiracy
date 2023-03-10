use crate::tile::TileType;
use macroquad::input::{is_key_pressed, is_key_down, is_mouse_button_pressed, mouse_position, KeyCode, MouseButton};
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


pub struct CurTile;

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

    pub fn start(&self, world: &mut World) {
        self.spawn_cur_tile(world);
    }

    pub fn update(&mut self, world: &mut World) {
        self.update_cur_tile_pos(world);
        if is_key_pressed(INSERT) {
            self.mode = EditorMode::Insert;
        } else if is_key_pressed(DELETE) {
            self.mode = EditorMode::Delete
        }

        if is_key_pressed(GROUND) {
            println!("ground tile selected");
            self.cur_tile_type = TileType::Ground;
            self.despawn_cur_tile(world);
            self.spawn_cur_tile(world);
        } else if is_key_pressed(WALL) {
            match self.cur_tile_type {
                TileType::Wall => {
                    println!("side wall tile selected");
                    self.cur_tile_type = TileType::SideWall;
                    self.despawn_cur_tile(world);
                    self.spawn_cur_tile(world);
                }
                TileType::SideWall => {
                    println!("gateway wall tile selected");
                    self.cur_tile_type = TileType::GatewayWall;
                    self.despawn_cur_tile(world);
                    self.spawn_cur_tile(world);
                }
                _ => {
                    println!("wall tile selected");
                    self.cur_tile_type = TileType::Wall;
                    self.despawn_cur_tile(world);
                    self.spawn_cur_tile(world);
                }
            }
        }

        if is_key_pressed(DOOR) {
            match self.cur_tile_type {
                TileType::Door => {
                    println!("side door tile selected");
                    self.cur_tile_type = TileType::SideDoor;

                    self.despawn_cur_tile(world);
                    self.spawn_cur_tile(world);
                }
                _ => {
                    println!("door tile selected");
                    self.cur_tile_type = TileType::Door;
                    self.despawn_cur_tile(world);
                    self.spawn_cur_tile(world);
                }
            }
        }

        if is_key_pressed(LAYER1) {
            println!("changed to layer 1");
            self.cur_layer = 0;
        } else if is_key_pressed(LAYER2) {
            println!("changed to layer 2");
            self.cur_layer = 1;
        }

        self.handle_mouse_click(world);

        if is_key_pressed(SAVE) {
            todo!("save the tile map");
        }
    }

    fn handle_mouse_click(&mut self, world: &mut World) {
        // println!("is key down {}", is_key_down(KeyCode::Z));
        if is_mouse_button_pressed(MouseButton::Left) || is_key_down(KeyCode::Z) {
            let mouse_pos = mouse_position();

            match self.mode {
                EditorMode::Insert => {
                    println!("inserting tile");
                    let tile_pos = (
                        (((mouse_pos.0 / self.cur_tile_type.size().0).floor())  * self.cur_tile_type.size().0),
                        ((mouse_pos.1 / self.cur_tile_type.size().1).floor()) * self.cur_tile_type.size().1 ,
                    );
                    self.tile_map[self.cur_layer]
                        .insert((tile_pos.0 as u32, tile_pos.1 as u32), self.cur_tile_type.clone());
                    world.spawn(self.cur_tile_type.new_tile_entity(tile_pos));
                }
                EditorMode::Delete => {
                    // self.tile_map
                    //     .remove_tile(self.cur_layer, tile_pos.0 as u32, tile_pos.1 as u32);
                }
            }
        }
    }

    fn despawn_cur_tile(&self, world: &mut World) {
        let mut to_remove = Vec::new();

        for (e, _) in &mut world.query::<(&TileType, &CurTile)>() {
            to_remove.push(e);
        }
        for e in to_remove {
            world.despawn(e).unwrap();
        }
    }

    fn spawn_cur_tile(&self, world: &mut World) {
        let (a,b,c) = self.cur_tile_type.new_tile_entity(mouse_position());
        world.spawn((a,b,c, CurTile));
    }

    fn update_cur_tile_pos(&self, world: &mut World) {
        for (_, (_, _, pos)) in &mut world.query::<(&TileType, &CurTile, &mut (f32, f32))>() {
            let mouse_pos = mouse_position();
            let tile_pos = (
                (((mouse_pos.0 / self.cur_tile_type.size().0).floor())  * self.cur_tile_type.size().0),
                ((mouse_pos.1 / self.cur_tile_type.size().1).floor()) * self.cur_tile_type.size().1 ,
            );
            *pos = tile_pos;
        }
    }
}


