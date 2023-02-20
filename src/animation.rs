use crate::assets::{Assets, TextureId};
use macroquad::prelude::{Rect, draw_texture, draw_texture_ex, DrawTextureParams, Vec2, WHITE};
use hecs::World;
use std::collections::HashMap;

pub fn build_door_anim(world: &mut World) {
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

    world.spawn((anim_src, Animation::new(0.8, 4)));
}

pub fn update_animations(world: &World, time_elapsed: f32) {
    for (_, anim) in &mut world.query::<&mut Animation>() {
        anim.update(time_elapsed);
    }
}

pub fn play_animations(world: &World) {
    for (_, anim) in &mut world.query::<&mut Animation>() {
        anim.play();
    }
}

pub fn draw_animations(world: &World, assets: &Assets) {
    for (_, (anim, anim_src)) in &mut world.query::<(&Animation, &AnimationSource)>() {
        anim.draw(Vec2::new(200.0, 200.0), assets, anim_src);
    }
}

#[derive(Debug)]
pub struct Animation {
    duration: f32,
    frames: u32,
    time_elapsed: f32,
    is_playing: bool,
}

impl Animation {
    pub fn new(duration: f32, frames: u32) -> Self {
        Self {
            duration,
            frames,
            time_elapsed: 0.0,
            is_playing: false,
        }
    }

    pub fn update(&mut self, time_elapsed: f32) {
        if self.is_playing {
            self.time_elapsed += time_elapsed;
            if self.time_elapsed >= self.duration {
                self.time_elapsed -= self.duration;
            }
        }
    }

    pub fn draw(&self, position: Vec2, assets: &Assets, anim_src: &AnimationSource) {
        if let Some(frame_params) = anim_src.get_frame(&self.current_frame()) {
            draw_texture_ex(
                *(assets.get_texture(&anim_src.texture_id).unwrap()),
                position.x,
                position.y,
                WHITE,
                frame_params.clone(),
            )
        }
    }

    pub fn play(&mut self) {
        self.is_playing = true;
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
    }

    pub fn reset(&mut self) {
        self.time_elapsed = 0.0;
    }

    fn current_frame(&self) -> u32 {
        let frame_duration = self.duration / self.frames as f32;
        (self.time_elapsed / frame_duration).floor() as u32
    }
}

#[derive(Debug)]
pub struct AnimationSource {
    pub texture_id: TextureId,
    frames: HashMap<u32, DrawTextureParams>,
}

impl AnimationSource {
    pub fn new(texture_id: TextureId) -> Self {
        Self {
            texture_id,
            frames: HashMap::new(),
        }
    }

    pub fn set_frame(&mut self, id: u32, params: DrawTextureParams) {
        self.frames.insert(id, params);
    }

    pub fn get_frame(&self, id: &u32) -> Option<&DrawTextureParams> {
        self.frames.get(id)
    }
}
