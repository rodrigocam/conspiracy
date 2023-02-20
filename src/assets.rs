use macroquad::texture::{load_texture, Texture2D};
use std::collections::HashMap;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum TextureId {
    Ground,
    Wall,
    SideWall,
    GatewayWall,
    Door,
    SideDoor,
    Shadow,
}

impl TextureId {
    pub fn path(&self) -> &str {
        match self {
            Self::Ground => "assets/textures/tiles/ground.png",
            Self::Wall => "assets/textures/tiles/wall.png",
            Self::SideWall => "assets/textures/tiles/side_wall.png",
            Self::GatewayWall => "assets/textures/tiles/gateway_wall.png",
            Self::Door => "assets/textures/tiles/door.png",
            Self::SideDoor => "assets/textures/tiles/side_door.png",
            Self::Shadow => "assets/textures/tiles/shadow.png",
        }
    }
}

pub struct Assets {
    textures: HashMap<TextureId, Texture2D>,
}

impl Assets {
    pub async fn load() -> Assets {
        let path = TextureId::Wall.path();
        println!("asdasd {}", path);
        Self {
            textures: HashMap::from([
                (
                    TextureId::Ground,
                    load_texture(TextureId::Ground.path()).await.unwrap(),
                ),
                (
                    TextureId::Wall,
                    load_texture(TextureId::Wall.path()).await.unwrap(),
                ),
                (
                    TextureId::SideWall,
                    load_texture(TextureId::SideWall.path()).await.unwrap(),),
                (
                    TextureId::GatewayWall,
                    load_texture(TextureId::GatewayWall.path()).await.unwrap(),
                ),
                (
                    TextureId::Door,
                    load_texture(TextureId::Door.path()).await.unwrap(),
                ),
                (
                    TextureId::SideDoor,
                    load_texture(TextureId::SideDoor.path()).await.unwrap(),
                ),
                (
                    TextureId::Shadow,
                    load_texture(TextureId::Shadow.path()).await.unwrap(),
                ),
            ]),
        }
    }

    pub fn get_texture(&self, tex_id: &TextureId) -> Option<&Texture2D> {
        self.textures.get(tex_id)
    }
}
