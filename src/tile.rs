use crate::asset::Assets;
use macroquad::{color::colors::WHITE, texture::{draw_texture, Texture2D}};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum TileType {
    Ground = 0,
    Wall = 1,
    SideWall = 2,
    GatewayWall = 3,
    Shadow = 4,
}

impl TileType {
    pub fn texture(&self, assets: &Assets) -> Texture2D {
        match self {
            Self::Ground => assets.textures.ground,
            Self::Wall => assets.textures.wall,
            Self::SideWall => assets.textures.side_wall,
            Self::GatewayWall => assets.textures.gateway_wall,
            // Tile::Door => TileData::new(assets.textures.door),
            // Tile::SideDoor => TileData::new(assets.textures.side_door),
            Self::Shadow => assets.textures.shadow,
        }
    }

    pub fn size(&self) -> (f32, f32) {
        match self {
            Self::Ground => (40.0, 40.0),
            Self::Wall =>(20.0, 65.0),
            _ => todo!("implement size for the rest of enum variants")
        }
    }

    pub fn tile_pos(&self, pos: (f32, f32)) -> (usize, usize) {
        (
            ((pos.0 / self.size().0).floor()) as usize,
            ((pos.1 / self.size().1).floor()) as usize,
        )
    }

}

pub type Layer = HashMap<(usize, usize), TileType>;

#[derive(Debug)]
pub struct GridPositionOutOfBounds;

pub struct TileMap {
    width: usize,
    height: usize,
    layers: [Layer; 2],
}

impl TileMap {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            layers: [HashMap::new(), HashMap::new()],
        }
    }

    pub fn get_tile(&self, layer: usize, x: usize, y: usize) -> Option<&TileType> {
        if x > self.width as usize || y > self.height as usize {
            return None;
        }

        self.layers[layer].get(&(x, y))
    }

    pub fn new_tile(
        &mut self,
        layer: usize,
        pos: (f32, f32),
        tile_type: TileType,
    ) -> Result<(), GridPositionOutOfBounds> {
        let tile_pos = tile_type.tile_pos(pos);
        if tile_pos.0 > self.width || tile_pos.1 > self.height {
            return Err(GridPositionOutOfBounds);
        }

        self.layers[layer].insert(tile_pos, tile_type);
        Ok(())
    }

    // pub fn remove_tile(&mut self, layer: usize, pos: (f32, f32) {
    //     self.layers[layer].remove((y * x + x) as usize);
    // }

    pub fn draw(&self, assets: &Assets) {
        self.draw_layer(0, assets);
        self.draw_layer(1, assets);
    }

    fn draw_layer(&self, layer: usize, assets: &Assets) {
        for (pos, tile_type) in self.layers[layer].iter() {
            draw_texture(tile_type.texture(assets), pos.0 as f32 * tile_type.size().0, pos.1 as f32 * tile_type.size().1, WHITE);
        }
    }
}
