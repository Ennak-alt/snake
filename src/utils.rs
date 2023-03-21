pub const WINDOW_SIZE: u8 = 160;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Point {
    pub x: u8,
    pub y: u8,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
