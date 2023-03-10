use crate::assets::*;
use crate::engine::{rendering::Render, animation::{AnimationSource, Animation}};


use macroquad::{
    color::colors::WHITE,
    texture::{draw_texture, Texture2D},
    prelude::{Rect, DrawTextureParams, Vec2},
};
use hecs::World;
use std::collections::HashMap;


fn build_door_animation() -> (AnimationSource, Animation) {
    let mut anim_src = AnimationSource::new(TextureId::Door as u32);
    let frame_size = Vec2::new(40.0, 40.0);
    for i in 0..4 {
        anim_src.set_frame(
            i,
            DrawTextureParams {
                dest_size: None,
                source: Some(
                    Rect {
                        x: i as f32 * frame_size.x,
                        y: 0.0,
                        w: frame_size.x, h: frame_size.y,
                    }
                ),
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        )
    }

    (anim_src, Animation::new(0.8, 4))
}

fn build_sidedoor_animation() -> (AnimationSource, Animation) {
    let mut anim_src = AnimationSource::new(TextureId::SideDoor as u32);
    let frame_size = Vec2::new(40.0, 59.0);
    for i in 0..4 {
        anim_src.set_frame(
            i,
            DrawTextureParams {
                dest_size: None,
                source: Some(
                    Rect {
                        x: i as f32 * frame_size.x,
                        y: 0.0,
                        w: frame_size.x,
                        h: frame_size.y,
                    }
                ),
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        )
    }

    (anim_src, Animation::new(0.8, 4))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TileType {
    Ground = 0,
    Wall = 1,
    SideWall = 2,
    GatewayWall = 3,
    Door = 4,
    SideDoor = 5,
    Shadow = 6,
}

impl TileType {
    pub fn new_tile_entity(&self, pos: (f32, f32)) -> (TileType, Render, (f32, f32)) {
        match self {
            Self::Ground => {
                return (self.clone(), Render::Texture(TextureId::Ground as u32), pos);
            },
            Self::Wall => {
                return (self.clone(), Render::Texture(TextureId::Wall as u32), pos);
            },
            Self::SideWall => {
                return (self.clone(), Render::Texture(TextureId::SideWall as u32), pos);
            },
            Self::GatewayWall => {
                return (self.clone(), Render::Texture(TextureId::GatewayWall as u32), pos);
            },
            Self::Door => {
                let (anim_src, anim) = build_door_animation();
                return (self.clone(), Render::Animation(anim, anim_src), pos);
            },
            Self::SideDoor => {
                let (anim_src, anim) = build_sidedoor_animation();
                return (self.clone(), Render::Animation(anim, anim_src), pos);
            },
            Self::Shadow => {
                todo!("not implemented");
            },
        }
    }

    pub fn size(&self) -> (f32, f32) {
        match self {
            Self::Ground => (40.0, 40.0),
            Self::Wall => (20.0, 40.0),
            Self::SideWall => (5.0, 5.0),
            Self::GatewayWall => (5.0, 5.0),
            Self::Door => (40.0, 40.0),
            Self::SideDoor => (5.0, 5.0),
            _ => todo!("implement size for the rest of enum variants"),
        }
    }
}


