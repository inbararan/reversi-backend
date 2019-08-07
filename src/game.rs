use super::board::{Board, Tile, Color};
use super::position::{Position, Direction};

enum Change {
    Color(Position, Color),
    Unset(Position),
    Player
}

pub struct Game {
    board: Board,
    current_player: Color,
    changes: Vec<Change>
}

pub struct ChangeSummary {
    pub tiles: std::collections::HashMap<Position, Tile>,
    pub player: Color
}

pub type Result = std::result::Result<(()), String>;

impl Game {
    pub fn new() -> Game {
        Game{ board: Board::new(10, 10), current_player: Color::Black, changes: vec![] }
    }

    pub fn flush_changes(&mut self) -> ChangeSummary {
        let mut tiles: std::collections::HashMap<Position, Tile> = std::collections::HashMap::new();
        for change in self.changes.iter() {
            match change {
                Change::Color(pos, col) => {
                    self.board.set(pos, col);
                    tiles.insert(*pos, Tile(Some(*col)));
                },
                Change::Unset(pos) => {
                    self.board.unset(pos);
                    tiles.insert(*pos, Tile(None));
                },
                Change::Player => {
                    self.current_player = self.current_player.opposite();
                }
            }
        }
        self.discard_changes();

        ChangeSummary {
            tiles: tiles,
            player: self.current_player
        }
    }

    pub fn discard_changes(&mut self) {
        self.changes = vec![];
    }

    pub fn prepare_board(&mut self) -> Result {
        for pos in self.board.iter_all_positions() {
            self.changes.push(Change::Unset(pos))
        }
        self.changes.push(Change::Color(Position{x: 4, y: 4}, Color::White));
        self.changes.push(Change::Color(Position{x: 4, y: 5}, Color::Black));
        self.changes.push(Change::Color(Position{x: 5, y: 4}, Color::Black));
        self.changes.push(Change::Color(Position{x: 5, y: 5}, Color::White));

        Ok(())
    }

    pub fn flip_vector(&self, position: &Position, direction: &Direction) -> Option<Vec<Position>> {
        let mut current = position.advance(direction, &self.board.size);
        let mut flip_vector: Vec<Position> = Vec::new();
        loop {
            match current {
                Some(pos) => {
                    match self.board.get(&pos).0 {
                        Some(color) => {
                            if color == self.current_player {
                                return if flip_vector.is_empty() { None } else { Some(flip_vector) }
                            } else {
                                flip_vector.push(pos);
                                println!("flipping: {},{}", pos.x, pos.y);
                                current = pos.advance(direction, &self.board.size);
                            }
                        },
                        None => {
                            return None;
                        }
                    }
                },
                None => {
                    return None;
                }
            }
        }
    }

    pub fn do_turn(&mut self, position: Position) -> Result {
        if self.board.taken(&position) { return Err(String::from("Position already taken")); }
        
        let flip_positions = Direction::iter_all()
                                       .filter_map(|direction|
                                           self.flip_vector(&position, direction)
                                       )
                                       .flatten()
                                       .collect::<Vec<Position>>();
        if flip_positions.is_empty() { return Err(String::from("You must flip at least one tile")); }
        
        for pos in flip_positions.into_iter() {
            self.changes.push(Change::Color(pos, self.current_player.clone()));
        }
        self.changes.push(Change::Color(position, self.current_player.clone()));
        self.changes.push(Change::Player);

        Ok(())
    }

    pub fn cancel(&mut self) -> Result {
        Err(String::from("Unimplemented"))
    }
}