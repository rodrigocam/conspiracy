mod assets;
mod editor;
mod tile;
mod engine;

use editor::Editor;
use hecs::*;
use macroquad::prelude::*;

#[macroquad::main("Conspiracy")]
async fn main() {
    let assets = engine::Assets::load(&[
        assets::TextureId::Ground.entry(),
        assets::TextureId::Wall.entry(),
        assets::TextureId::SideWall.entry(),
        assets::TextureId::GatewayWall.entry(),
        assets::TextureId::Door.entry(),
        assets::TextureId::SideDoor.entry(),
        assets::TextureId::Shadow.entry(),
    ]).await;

    let mut world = World::new();
    let mut editor = Editor::new();

    editor.start(&mut world);

    loop {
        clear_background(BLACK);

        editor.update(&mut world);

        engine::animation::system_update_animation(&mut world);
        engine::rendering::system_render(&world, &assets);


        // editor.update();
        // editor.draw(&assets);
        // editor::system_update(&mut editor, &mut world);
        // editor::system_handle_mouse_click(&mut editor, &mut world);
        // update_animations(&mut world, get_frame_time());
        // rendering::system_draw_tiles(&mut world, &assets);
        // draw_animations(&world, &assets);
        // system_render_textures(&mut world, &assets);
        next_frame().await
    }
}
