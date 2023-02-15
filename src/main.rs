mod tile;
mod asset;

use macroquad::prelude::*;
use asset::Assets;
use tile::{TileData, Tiles};


#[macroquad::main("Conspiracy")]
async fn main() {
    let assets = Assets::load().await;
    let tiles = Tiles::load(&assets);
    let mut tile_set: Vec<(Texture2D, (f32, f32))> = Vec::new();

    loop {
        clear_background(BLACK);
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            let tile_pos = ((mouse_pos.0/40.0).floor() * 40.0, (mouse_pos.1/40.0).floor() * 40.0);
            tile_set.push((assets.textures.ground, tile_pos));
        }

        for (t, p) in &tile_set {
            draw_texture(*t, p.0, p.1, WHITE);
        }
        // draw_texture(assets.textures.ground, 40.0, 0.0, WHITE);
        next_frame().await
    }
}
