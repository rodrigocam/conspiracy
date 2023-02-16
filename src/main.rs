mod asset;
mod editor;
mod tile;

use asset::Assets;
use macroquad::prelude::*;
use editor::Editor;

#[macroquad::main("Conspiracy")]
async fn main() {
    let assets = Assets::load().await;
    let mut editor = Editor::new();

    loop {
        clear_background(BLACK);
        editor.update();
        editor.draw(&assets);
        // for (t, p) in &tile_set {
        //     draw_texture(*t, p.0, p.1, WHITE);
        // }
        next_frame().await
    }
}
