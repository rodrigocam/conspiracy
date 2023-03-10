use macroquad::prelude::*;
use hecs::World;
use std::collections::HashMap;


pub type TextureId = u32;

pub struct Assets {
    textures: HashMap<TextureId, Texture2D>,
}

impl Assets {
    pub async fn load(textures: &[(TextureId, &str)]) -> Assets {
        let mut loaded_textures = HashMap::new(); 
        for (tex_id, path) in textures {
            loaded_textures.insert(tex_id.clone(), load_texture(path).await.unwrap());
        }

        Self {
            textures: loaded_textures,
        }
    }

    pub fn get_texture(&self, tex_id: &TextureId) -> Option<&Texture2D> {
        self.textures.get(tex_id)
    }
}

pub struct Update {
    pub delta_time: f32,
}

pub fn system_update(world: &mut World) {
    for (_, update) in &mut world.query::<&mut Update>() {
        update.delta_time = get_frame_time();
    }
}

pub mod animation {
    use macroquad::prelude::*;
    use hecs::World;
    use std::collections::HashMap;
    use crate::engine::{Assets, TextureId};


    #[derive(Debug)]
    pub struct Animation {
        duration: f32,
        frames: u32,
        time_elapsed: f32,
        is_playing: bool,
        run_forever: bool,
    }

    impl Animation {
        pub fn new(duration: f32, frames: u32) -> Self {
            Self {
                duration,
                frames,
                time_elapsed: 0.0,
                is_playing: false,
                run_forever: false,
            }
        }

        pub fn update(&mut self, time_elapsed: f32) {
            if self.is_playing {
                self.time_elapsed += time_elapsed;
                if self.time_elapsed >= self.duration {
                    if self.run_forever {
                        self.time_elapsed -= self.duration;
                    } else {
                        self.stop();
                    }
                }
            }
        }

        pub fn draw(&self, position: (f32, f32), assets: &Assets, anim_src: &AnimationSource) {
            if let Some(frame_params) = anim_src.get_frame(&self.current_frame()) {
                draw_texture_ex(
                    *(assets.get_texture(&anim_src.texture_id).unwrap()),
                    position.0,
                    position.1,
                    WHITE,
                    frame_params.clone(),
                )
            }
        }

        pub fn play(&mut self) {
            self.is_playing = true;
        }

        pub fn toggle_run_forever(&mut self) {
            self.run_forever = !self.run_forever;
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

    pub fn system_update_animation(world: &mut World) {
        for (_, anim) in &mut world.query::<&mut Animation>() {
            anim.update(get_frame_time());
        }
    }
}

pub mod rendering {
    use macroquad::prelude::*;
    use hecs::World;
    use crate::engine::{TextureId, Assets};
    use crate::engine::animation::{Animation, AnimationSource};

    pub enum Render {
        Texture(TextureId),
        Animation(Animation, AnimationSource),
    }

    pub fn system_render(world: &World, assets: &Assets) {
        let all_renders = &mut world.query::<(&Render, &(f32, f32))>();
        for (_, (render, pos)) in all_renders {
            match &render {
                Render::Texture(tex_id) => draw_texture(
                    *assets.get_texture(tex_id).unwrap(),
                    pos.0,
                    pos.1,
                    WHITE,
                ),
                Render::Animation(anim, anim_src) => anim.draw(
                    *pos,
                    assets,
                    anim_src
                ),
            }
        }
    }
}

