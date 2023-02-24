use crate::{animation::*, assets::*};


use macroquad::{
    color::colors::WHITE,
    texture::{draw_texture, Texture2D},
    prelude::{Rect, DrawTextureParams, Vec2},
};
use hecs::World;
use std::collections::HashMap;


fn build_door_animation() -> (AnimationSource, Animation) {
    let mut anim_src = AnimationSource::new(TextureId::Door);
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

fn build_sidedoor_animation() -> (AnimationSource, Animation) {
    let mut anim_src = AnimationSource::new(TextureId::SideDoor);
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
    pub fn spawn_tile(&self, world: &mut World, pos: (f32, f32)) {
        match self {
            Self::Ground => {
                world.spawn((self.clone(), TextureId::Ground, pos));
            },
            Self::Wall => {
                world.spawn((self.clone(), TextureId::Wall, pos));
            },
            Self::SideWall => {
                world.spawn((self.clone(), TextureId::SideWall, pos));
            },
            Self::GatewayWall => {
                world.spawn((self.clone(), TextureId::GatewayWall, pos));
            },
            Self::Door => {
                let (anim_src, anim) = build_door_animation();
                world.spawn((self.clone(), anim_src, anim, pos));
            },
            Self::SideDoor => {
                let (anim_src, anim) = build_sidedoor_animation();
                world.spawn((self.clone(), anim_src, anim, pos));
            },
            Self::Shadow => {},
        }
    }

    pub fn size(&self) -> (f32, f32) {
        match self {
            Self::Ground => (40.0, 40.0),
            Self::Wall => (20.0, 65.0),
            Self::SideWall => (5.0, 5.0),
            Self::GatewayWall => (5.0, 5.0),
            Self::Door => (40.0, 40.0),
            Self::SideDoor => (5.0, 59.0),
            _ => todo!("implement size for the rest of enum variants"),
        }
    }
}


