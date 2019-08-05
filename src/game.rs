use super::board::{Board, Position, Cell, Color};

enum Change {
    Color(Position, Color),
    Unset(Position)
    /*Flip(Position)*/
}

pub struct Game {
    board: Board,
    changes: Vec<Change>
}

pub struct ChangeSummary(pub std::collections::HashMap<Position, Cell>);

pub type Result = std::result::Result<(()), String>;

impl Game {
    pub fn new() -> Game {
        Game{ board: Board::new(10, 10), changes: vec![] }
    }

    pub fn flush_changes(&mut self) -> ChangeSummary {
        let mut summary = ChangeSummary(std::collections::HashMap::new());
        for change in self.changes.iter() {
            match change {
                Change::Color(pos, col) => {
                    self.board.set(pos, col);
                    summary.0.insert(*pos, Cell(Some(*col)));
                },
                Change::Unset(pos) => {
                    self.board.unset(pos);
                    summary.0.insert(*pos, Cell(None));
                }
            }
        }
        self.discard_changes();

        summary
    }

    pub fn discard_changes(&mut self) {
        self.changes = vec![];
    }

    pub fn prepare_board(&mut self) -> Result {
        for pos in self.board.iter_all_positions() {
            self.changes.push(Change::Unset(pos))
        }
        self.changes.push(Change::Color(Position{x: 5, y: 5}, Color::White));
        self.changes.push(Change::Color(Position{x: 5, y: 6}, Color::Black));
        self.changes.push(Change::Color(Position{x: 6, y: 5}, Color::Black));
        self.changes.push(Change::Color(Position{x: 6, y: 6}, Color::White));

        Ok(())
    }

    fn legal_position(&self, position: &Position) -> bool {
        if self.board.taken(position) { return false; }

        true
    }

    pub fn do_turn(&mut self, position: Position) -> Result {
        Err(String::from("Unimplemented"))
    }

    pub fn cancel(&mut self) -> Result {
        Err(String::from("Unimplemented"))
    }
}