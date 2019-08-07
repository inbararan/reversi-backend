use super::position::{Position, Size};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Color {
    White, Black
}

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Tile(pub Option<Color>);

pub struct Board {
    tiles: Vec<Tile>,
    pub size: Size
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        Board {
            tiles: vec![Tile(None); width * height],
            size: Size{ width: width, height: height }
        }
    }

    fn at(&self, pos: &Position) -> &Tile {
        &self.tiles[pos.y * self.size.width + pos.x]
    }

    fn at_mut(&mut self, pos: &Position) -> &mut Tile {
        &mut self.tiles[pos.y * self.size.width + pos.x]
    }

    pub fn get(&self, pos: &Position) -> Tile {
        self.at(pos).clone()
    }

    pub fn set(&mut self, pos: &Position, color: &Color) {
        *self.at_mut(pos) = Tile(Some(color.clone()));
    }

    pub fn unset(&mut self, pos: &Position) {
        *self.at_mut(pos) = Tile(None);
    }

    pub fn iter_all_positions(&self) -> impl Iterator<Item=Position> {
        let size = self.size;
        (0..size.width*size.height).map(move |idx| Position{x: idx % size.width, y: idx / size.width})
    }

    pub fn taken(&self, pos: &Position) -> bool {
        self.at(pos).0.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::super::position::Position;
    use super::Board;

    #[test]
    fn iter_all_positions_test() {
        let board = Board::new(3, 4);
        let mut iter = board.iter_all_positions();
        assert_eq!(iter.next(), Some(Position{x: 0, y: 0}));
        assert_eq!(iter.next(), Some(Position{x: 1, y: 0}));
        assert_eq!(iter.next(), Some(Position{x: 2, y: 0}));
        assert_eq!(iter.next(), Some(Position{x: 0, y: 1}));
        assert_eq!(iter.next(), Some(Position{x: 1, y: 1}));
        assert_eq!(iter.next(), Some(Position{x: 2, y: 1}));
        assert_eq!(iter.next(), Some(Position{x: 0, y: 2}));
        assert_eq!(iter.next(), Some(Position{x: 1, y: 2}));
        assert_eq!(iter.next(), Some(Position{x: 2, y: 2}));
        assert_eq!(iter.next(), Some(Position{x: 0, y: 3}));
        assert_eq!(iter.next(), Some(Position{x: 1, y: 3}));
        assert_eq!(iter.next(), Some(Position{x: 2, y: 3}));
        assert_eq!(iter.next(), None);
    }
}
