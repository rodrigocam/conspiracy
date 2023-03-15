use crate::tile::TileType;
use macroquad::input::{is_key_pressed, is_key_down, is_mouse_button_pressed, mouse_position, KeyCode, MouseButton};
use hecs::World;
use std::collections::HashMap;
use serde::{ser::SerializeMap, Serialize, Deserialize, Serializer};
use std::fmt::{Display, Formatter};


const INSERT: KeyCode = KeyCode::I;
const DELETE: KeyCode = KeyCode::D;
const SAVE: KeyCode = KeyCode::S;

const GROUND: KeyCode = KeyCode::Q;
const WALL: KeyCode = KeyCode::W;
const DOOR: KeyCode = KeyCode::E;

const LAYER1: KeyCode = KeyCode::Key1;
const LAYER2: KeyCode = KeyCode::Key2;

#[derive(Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct TileIndex((u32, u32));

impl Display for TileIndex {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[derive(Deserialize, Debug)]
pub struct TileMap(HashMap<TileIndex, TileType>);

impl Serialize for TileMap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for (k, v) in &self.0 {
            map.serialize_entry(&k.to_string(), &v)?;
        }
        map.end()
    }
}


#[derive(Serialize, Deserialize)]
pub struct CurTile;


#[derive(Serialize, Deserialize)]
pub enum EditorMode {
    Insert,
    Delete,
}

#[derive(Serialize, Deserialize)]
pub struct Editor {
    pub mode: EditorMode,
    pub cur_tile_type: TileType,
    pub cur_layer: usize,
    pub tile_map: [TileMap; 3],
}

impl Editor {
    pub fn new() -> Self {
        Self {
            mode: EditorMode::Insert,
            cur_tile_type: TileType::Ground,
            cur_layer: 0,
            tile_map: [TileMap(HashMap::new()), TileMap(HashMap::new()), TileMap(HashMap::new())],
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
            let serialized = serde_json::to_string(self).unwrap();
            std::fs::write("map.txt", serialized).unwrap();
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
                    self.tile_map[self.cur_layer].0
                        .insert(TileIndex((tile_pos.0 as u32, tile_pos.1 as u32)), self.cur_tile_type.clone());
                    let (a,b,c) =self.cur_tile_type.new_tile_entity(tile_pos);
                    world.spawn((a,b,c, self.cur_layer as crate::engine::rendering::Layer));
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
        world.spawn((a,b,c, CurTile, 100 as crate::engine::rendering::Layer));
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


