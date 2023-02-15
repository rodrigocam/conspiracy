use macroquad::texture::Texture2D;
use crate::asset::Assets;


pub enum Tile {
    Ground = 0,
    Wall = 1,
    SideWall = 2,
    GatewayWall = 3,
    // Door = 4,
    // SideDoor = 5,
    Shadow = 6,
}

impl Tile {

    pub fn new(&self, assets: &Assets) -> TileData {
        match self {
            Tile::Ground => TileData::new(assets.textures.ground),
            Tile::Wall => TileData::new(assets.textures.wall),
            Tile::SideWall => TileData::new(assets.textures.side_wall),
            Tile::GatewayWall => TileData::new(assets.textures.gateway_wall),
            // Tile::Door => TileData::new(assets.textures.door),
            // Tile::SideDoor => TileData::new(assets.textures.side_door),
            Tile::Shadow => TileData::new(assets.textures.shadow),
        }
    }
}

pub struct TileData {
    texture: Texture2D,
    // collider
}

impl TileData {
    pub fn new(texture: Texture2D) -> Self {
        Self { texture }
    }
}

pub struct Tiles {
    pub ground: TileData,
    pub wall: TileData,
    pub side_wall: TileData,
    pub gateway_wall: TileData,
    // pub door: TileData,
    // pub side_door: TileData,
    pub shadow: TileData,
}

impl Tiles {
    pub fn load(assets: &Assets) -> Self {
        Self {
            ground: Tile::Ground.new(assets),
            wall: Tile::Wall.new(assets),
            side_wall: Tile::SideWall.new(assets),
            gateway_wall: Tile::GatewayWall.new(assets),
            // door: Tile::Door.new(assets),
            // side_door: Tile::SideDoor.new(assets),
            shadow: Tile::Shadow.new(assets),
        }
    }
}
