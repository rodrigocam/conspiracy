use hecs::*;
use macroquad::prelude::*;
use crate::{engine::rendering::{Layer, Render}, tile::TileType};
use std::collections::HashMap;

const GROUND: KeyCode = KeyCode::Q;
const WALL: KeyCode = KeyCode::W;
const DOOR: KeyCode = KeyCode::E;

const UNDO: KeyCode = KeyCode::U;
const SAVE: KeyCode = KeyCode::S;
const LOAD: KeyCode = KeyCode::L;
const MULTI: KeyCode = KeyCode::Z;

const MAP_PATH: &str = "stage_1.txt";


#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    Insert(TileType, (u32, u32)),
}


#[derive(Debug)]
pub struct Editor {
    pub selected_tile: TileType,
    pub cur_tile: Option<Entity>,
    pub tile_map: HashMap<(u32, u32), (TileType, Entity)>,
    commands: Vec<Command>,
    cur_layer: Layer,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            selected_tile: TileType::Ground,
            cur_tile: None,
            tile_map: HashMap::new(),
            commands: Vec::new(),
            cur_layer: 0,
        }
    }

    pub fn start(&mut self, world: &mut World) {
        let render = self.selected_tile.get_render();
        self.cur_tile = Some(world.spawn(
            (render, mouse_position(), 100 as Layer)
        ));
    }

    pub fn update(&mut self, world: &mut World) {
        self.sync_cur_tile_position(world);

        if is_key_pressed(GROUND) {
            self.update_selected_tile(world, TileType::Ground);
        }

        if is_key_pressed(WALL) {
            match self.selected_tile {
                TileType::Wall => self.update_selected_tile(world, TileType::SideWall),
                TileType::SideWall => self.update_selected_tile(world, TileType::GatewayWall),
                _ => self.update_selected_tile(world, TileType::Wall),
            }
        }

        if is_key_pressed(DOOR) {
            match self.selected_tile {
                TileType::Door => self.update_selected_tile(world, TileType::SideDoor),
                _ => self.update_selected_tile(world, TileType::Door),
            }
        }

        if is_key_pressed(UNDO) {
            self.undo_command(world);
        }

        if is_key_pressed(SAVE) {
            self.save_map();
        }
        
        if is_key_pressed(LOAD) {
            println!("loading");
            self.load_map(world);
        }

        if is_mouse_button_pressed(MouseButton::Left) || is_key_down(MULTI) {
            self.insert_selected_tile(world);
        }
    }

    fn undo_command(&mut self, world: &mut World) {
        if let Some(last_command) = self.commands.pop() {
            println!("undoing {:?}", last_command);
            match last_command {
                Command::Insert(_, pos) => {
                    if let Some(removed) = self.tile_map.remove(&pos) {
                        world.despawn(removed.1).unwrap();
                    }
                },
            }
        }
    }

    fn update_selected_tile(&mut self, world: &mut World, tile: TileType) {
        self.selected_tile = tile;
        let mut query = world.query_one::<&mut Render>(self.cur_tile.unwrap()).unwrap();
        let render = query.get().unwrap();
        *render = self.selected_tile.get_render();
    }

    fn sync_cur_tile_position(&mut self, world: &mut World) {
        let mut query = world.query_one::<&mut (f32, f32)>(self.cur_tile.unwrap()).unwrap();
        let pos = query.get().unwrap();
        let tile_pos = self.tile_position();
        *pos = (tile_pos.0 as f32, tile_pos.1 as f32);
    }

    fn insert_selected_tile(&mut self, world: &mut World) {
        self.insert_tile(world, self.selected_tile, self.tile_position());
    }

    fn tile_position(&self) -> (u32, u32) {
        let mouse_pos = mouse_position();
        (
            (((mouse_pos.0 / self.selected_tile.size().0).floor())  * self.selected_tile.size().0) as u32,
            (((mouse_pos.1 / self.selected_tile.size().1).floor()) * self.selected_tile.size().1) as u32 ,
        )
    }

    fn save_map(&self) {
        println!("saving map");
        let mut buffer = String::new();
        for command in self.commands.iter() {
            match command {
                Command::Insert(tile, pos) => {
                    buffer.push_str(&format!("{}:{}x{} ", *tile as u32, pos.0, pos.1));
                }
            }
        }
        std::fs::write(MAP_PATH, &buffer).unwrap();
    }

    fn load_map(&mut self, world: &mut World) {
        println!("loading map");
        let content = std::fs::read_to_string(MAP_PATH).unwrap();
        for s in content.trim_end().split(' ') {
            let mut entry = s.split(':');
            let tile = TileType::from_u32(entry.next().unwrap().parse::<u32>().unwrap());
            let mut pos = entry.next().unwrap().split("x").map(|s| s.parse::<u32>().unwrap());
            let pos = (pos.next().unwrap(), pos.next().unwrap());

            self.insert_tile(world, tile, pos);
        }
    }

    fn insert_tile(&mut self, world: &mut World, tile: TileType, pos: (u32, u32)) {
        let command = Command::Insert(tile, pos);
        if self.commands.last().is_some() && self.commands.last().unwrap() == &command {
            return
        }
        println!("command {:?}", command);
        self.commands.push(command);
        let entity = world.spawn((tile.get_render(), (pos.0 as f32, pos.1 as f32), 0 as Layer));
        self.tile_map.insert(pos, (tile, entity));
    }
}
