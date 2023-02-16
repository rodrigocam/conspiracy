mod asset;
mod editor;
mod tile;

use asset::Assets;
use editor::Editor;
use macroquad::prelude::*;

#[macroquad::main("Conspiracy")]
async fn main() {
    let assets = Assets::load().await;
    let mut editor = Editor::new();

    loop {
        clear_background(BLACK);
        editor.update();
        editor.draw(&assets);
        next_frame().await
    }
}
