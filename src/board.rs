#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Position {
    pub x: usize, pub y: usize
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Color {
    White, Black
}

#[derive(PartialEq, Debug, Clone)]
pub struct Cell(pub Option<Color>);

pub struct Board {
    cells: Vec<Cell>,
    width: usize,
    height: usize
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        Board {
            cells: vec![Cell(None); width * height],
            width: width,
            height: height
        }
    }

    fn at(&self, pos: &Position) -> &Cell {
        &self.cells[pos.y * self.width + pos.x]
    }

    fn at_mut(&mut self, pos: &Position) -> &mut Cell {
        &mut self.cells[pos.y * self.width + pos.x]
    }

    pub fn get(&self, pos: &Position) -> Cell {
        self.at(pos).clone()
    }

    pub fn set(&mut self, pos: &Position, color: &Color) {
        *self.at_mut(pos) = Cell(Some(color.clone()));
    }

    pub fn unset(&mut self, pos: &Position) {
        *self.at_mut(pos) = Cell(None);
    }

    pub fn iter_all_positions(&self) -> PositionIter {
        PositionIter{
            board: self,
            current: Some(Position{ x: 0, y: 0 }),
            advance: Box::new(|pos, board| {
                if pos.x + 1 < board.width {
                    Some(Position{x: pos.x + 1, y: pos.y})
                } else if pos.y + 1 < board.height {
                    Some(Position{x: 0, y: pos.y + 1})
                } else {
                    None
                }
            })
        }
    }

    pub fn as_hash_map(&self) -> std::collections::HashMap<Position, Cell> {
        self.iter_all_positions()
             .map(|pos| (pos, self.get(&pos)))
             .collect()
    }

    pub fn position_legality<'closure, 'board: 'closure>(&'board self) -> Box<dyn Fn(Position)->bool + 'closure>{
        /* Clarification: the "move" keyword moves the self argument, which is a reference in itself. The board itself isn't taken ownership on, of course*/
        Box::new(move |pos: Position| -> bool { pos.x < self.width && pos.y < self.height })
    }

    pub fn taken(&self, pos: &Position) -> bool {
        self.at(pos).0.is_some()
    }
}

pub struct PositionIter <'a>{
    board: &'a Board,
    current: Option<Position>,
    advance: Box<dyn Fn(&Position, &Board)->Option<Position>>
}

impl Iterator for PositionIter<'_> {
    type Item = Position;

    fn next(&mut self) -> Option<Position> {
        match &self.current {
            None => None,
            Some(current_pos) => {
                let next = current_pos.clone();
                self.current = (self.advance)(current_pos, self.board);
                Some(next)
            }
        }
    }
}

#[cfg(test)]
mod tests{
    use super::{Board, Position, Cell};

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

    #[test]
    fn as_hash_map_test() {
        let board = Board::new(3, 4);
        let mut desired: std::collections::HashMap<Position, Cell> = std::collections::HashMap::new();
        desired.insert(Position{x: 0, y: 0}, Cell(None));
        desired.insert(Position{x: 1, y: 0}, Cell(None));
        desired.insert(Position{x: 2, y: 0}, Cell(None));
        desired.insert(Position{x: 0, y: 1}, Cell(None));
        desired.insert(Position{x: 1, y: 1}, Cell(None));
        desired.insert(Position{x: 2, y: 1}, Cell(None));
        desired.insert(Position{x: 0, y: 2}, Cell(None));
        desired.insert(Position{x: 1, y: 2}, Cell(None));
        desired.insert(Position{x: 2, y: 2}, Cell(None));
        desired.insert(Position{x: 0, y: 3}, Cell(None));
        desired.insert(Position{x: 1, y: 3}, Cell(None));
        desired.insert(Position{x: 2, y: 3}, Cell(None));
        assert_eq!(board.as_hash_map(), desired);
    }
}
