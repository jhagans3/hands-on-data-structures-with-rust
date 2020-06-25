pub struct Strength {
    pub strength: i16,
    pub health: i16,
}

#[derive(PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct Direction {
    pub velocity_x: i32,
    pub velocity_y: i32,
}
