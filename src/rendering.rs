use crate::{animation::*, tile::TileType, assets::{Assets, TextureId}};
use hecs::*;
use macroquad::prelude::{draw_texture, Vec2, WHITE};

pub struct Drawable;

pub fn system_render_textures(world: &mut World, assets: &Assets) {
    for (_, (tex_id, _, position)) in &mut world.query::<(&TextureId, &Drawable, &Vec2)>() {
        draw_texture(
            *assets.get_texture(tex_id).unwrap(),
            position.x,
            position.y,
            WHITE,
        );
    }
}

pub fn system_draw_tiles(world: &mut World, assets: &Assets) {
    for (_, (_, anim, anim_src, pos)) in &mut world.query::<(&TileType, &Animation, &AnimationSource, &(f32, f32))>() {
        anim.draw(Vec2::from(*pos), assets, anim_src);
    }
    
    for (_, (_, tex_id, position)) in &mut world.query::<(&TileType, &TextureId, &(f32, f32))>() {
        draw_texture(
            *assets.get_texture(tex_id).unwrap(),
            position.0,
            position.1,
            WHITE,
        );
    }
}
