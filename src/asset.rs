use macroquad::texture::{load_texture, Texture2D};

pub struct Assets {
    pub textures: Textures,
}

impl Assets {
    pub async fn load() -> Self {
        let ground = load_texture("assets/textures/tiles/ground.png")
            .await
            .unwrap();
        let wall = load_texture("assets/textures/tiles/wall.png")
            .await
            .unwrap();
        let side_wall = load_texture("assets/textures/tiles/side_wall.png")
            .await
            .unwrap();
        let gateway_wall = load_texture("assets/textures/tiles/gateway_wall.png")
            .await
            .unwrap();
        // let door = load_texture("textures/tiles/door.png").await.unwrap();
        // let side_door = load_texture("textures/tiles/side_door.png").await.unwrap();
        let shadow = load_texture("assets/textures/tiles/shadow.png")
            .await
            .unwrap();

        Self {
            textures: Textures {
                ground,
                wall,
                side_wall,
                gateway_wall,
                // door,
                // side_door,
                shadow,
            },
        }
    }
}

pub struct Textures {
    pub ground: Texture2D,
    pub wall: Texture2D,
    pub side_wall: Texture2D,
    pub gateway_wall: Texture2D,
    // pub door: Texture2D,
    // pub side_door: Texture2D,
    pub shadow: Texture2D,
}
