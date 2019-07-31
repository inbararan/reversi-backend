#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Position {
    pub x: usize, pub y: usize
}

pub enum Color {
    White, Black
}

pub struct Cell(pub Option<Color>);