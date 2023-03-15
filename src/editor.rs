use hecs::*;
use macroquad::prelude::*;
use crate::{engine::rendering::{Layer, Render}, tile::TileType};
use std::collections::HashMap;

const GROUND: KeyCode = KeyCode::Q;
const WALL: KeyCode = KeyCode::W;
const DOOR: KeyCode = KeyCode::E;

const UNDO: KeyCode = KeyCode::U;


#[derive(Debug)]
pub enum Command {
    Insert(TileType, (u32, u32)),
    Undo,
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

        if is_mouse_button_pressed(MouseButton::Left) {
            self.insert_selected_tile(world);
        }
    }

    fn undo_command(&mut self, world: &mut World) {
        let last_command = self.commands.pop().unwrap();
        match last_command {
            Command::Insert(_, pos) => {
                let removed = self.tile_map.remove(&pos).unwrap();
                world.despawn(removed.1).unwrap();
            },
            _ => {}
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
        let tile_pos = self.tile_position();
        self.commands.push(Command::Insert(self.selected_tile, tile_pos));
        let entity = world.spawn((self.selected_tile.get_render(), (tile_pos.0 as f32, tile_pos.1 as f32), self.cur_layer));
        self.tile_map.insert(tile_pos, (self.selected_tile, entity));
    }

    fn tile_position(&self) -> (u32, u32) {
        let mouse_pos = mouse_position();
        (
            (((mouse_pos.0 / self.selected_tile.size().0).floor())  * self.selected_tile.size().0) as u32,
            (((mouse_pos.1 / self.selected_tile.size().1).floor()) * self.selected_tile.size().1) as u32 ,
        )
    }
}
