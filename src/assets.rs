
#[derive(Debug, Clone, Copy)]
pub enum TextureId {
    Ground = 0,
    Wall = 1,
    SideWall = 2,
    GatewayWall = 3,
    Door = 4,
    SideDoor = 5,
    Shadow = 6,
}

impl TextureId {
    pub fn entry(&self) -> (u32, &str) {
        match self {
            Self::Ground => (*self as u32, "assets/textures/tiles/ground.png"),
            Self::Wall => (*self as u32, "assets/textures/tiles/wall.png"),
            Self::SideWall => (*self as u32, "assets/textures/tiles/side_wall.png"),
            Self::GatewayWall => (*self as u32, "assets/textures/tiles/gateway_wall.png"),
            Self::Door => (*self as u32, "assets/textures/tiles/door.png"),
            Self::SideDoor => (*self as u32, "assets/textures/tiles/side_door.png"),
            Self::Shadow => (*self as u32, "assets/textures/tiles/shadow.png"),
        }
    }
}
