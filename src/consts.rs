pub const MAP_SIZE_I: usize = 12;
pub const MAP_SIZE_J: usize = 16;

pub mod fonts {
    pub const MAIN_FONT: &str = "fonts/SourceCodePro-Medium.ttf";
}

pub mod assets {
    pub const BONUS: &str = "models/bonus.glb#Scene0";
    pub const FLOOR: &str = "models/floorFull.glb#Scene0";
    pub const WALL: &str = "models/wall.glb#Scene0";
    pub const DOOR: &str = "models/wallDoorway.glb#Scene0";
    pub const ALIEN: &str = "models/alien.glb#Scene0";
    pub const ASTRONAUTS: [&str; 2] = [
        "models/astronautA.glb#Scene0",
        "models/astronautB.glb#Scene0"
    ];
}
