mod assets;
// mod editor;
// mod tile;
mod animation;
mod rendering;

use assets::*;
// use editor::Editor;
use hecs::*;
use macroquad::prelude::*;
use rendering::system_render_textures;
use animation::*;

#[macroquad::main("Conspiracy")]
async fn main() {
    let assets = Assets::load().await;
    let mut world = World::new();
    // let mut editor = Editor::new();

    build_door_anim(&mut world);
    play_animations(&world);
    loop {
        clear_background(BLACK);
        // editor.update();
        // editor.draw(&assets);
        update_animations(&mut world, get_frame_time());
        draw_animations(&world, &assets);
        system_render_textures(&mut world, &assets);
        next_frame().await
    }
}
