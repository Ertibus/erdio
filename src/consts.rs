pub const MAP_SIZE_I: usize = 24;
pub const MAP_SIZE_J: usize = 32;

pub mod assets {
    pub const FLOOR: &str = "models/floorFull.glb#Scene0";
    pub const WALL: &str = "models/wall.glb#Scene0";
    pub const DOOR: &str = "models/wallDoorway.glb#Scene0";
    pub const ALIEN: &str = "models/alien.glb#Scene0";
    pub const ASTRONAUTS: [&str; 2] = [
        "models/astronautA.glb#Scene0",
        "models/astronautB.glb#Scene0"
    ];
}
