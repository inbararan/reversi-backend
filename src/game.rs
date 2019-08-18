use super::board::{Board, Tile, Color};
use super::position::Position;
use std::collections::HashMap;

/*
This struct is used in a doubled role. It represents a "hard" set of changes, non-regarding the previous game state
The two roles are:
1. As a return value to send the front-end - which doesn't know anything about the values.
2. As a saved information in order to recreate previous states - as it's created anyway, and makes revoke_changes somewhat easier to implement
*/
pub struct ChangeSet {
    pub tiles: HashMap<Position, Tile>,
    pub player: Color
}

struct ChangeLog {
    pending_tile_changes: Vec<Position>,            /* List of pending tiles to change */
    pending_player_change: bool,               
    history: Vec<ChangeSet>                         /* A stack where each item is a list of changes, reversing one set of changes */
}

impl ChangeLog {
    pub fn new() -> ChangeLog {
        ChangeLog {
            pending_tile_changes: Vec::new(),
            pending_player_change: false,
            history: Vec::new()
        }
    }

    pub fn push_tile_change(&mut self, position: Position) {
        self.pending_tile_changes.push(position);
    }
    pub fn push_player_change(&mut self) {
        self.pending_player_change = !self.pending_player_change;
    }
    
    pub fn discard_changes(&mut self) {
        self.pending_tile_changes.clear();
        self.pending_player_change = false;
    }
}

pub type Result = std::result::Result<ChangeSet, String>;

pub struct Game {
    board: Board,
    current_player: Color,
    change_log: ChangeLog
}

impl Game {
    pub fn new() -> Game {
        let mut board = Board::new(10, 10);
        board.set(&Position{x: 4, y: 4}, &Color::White);
        board.set(&Position{x: 4, y: 5}, &Color::Black);
        board.set(&Position{x: 5, y: 4}, &Color::Black);
        board.set(&Position{x: 5, y: 5}, &Color::White);

        Game{ board: board, current_player: Color::Black, change_log: ChangeLog::new() }
    }

    fn error(&mut self, message: &'static str) -> Result {
        self.change_log.discard_changes();
        Err(String::from(message))
    }

    fn flush_changes(&mut self) -> ChangeSet {
        let mut tiles: HashMap<Position, Tile> = HashMap::new();
        let mut history_tiles: HashMap<Position, Tile> = HashMap::new();
        let history_player = self.current_player;

        for pos in self.change_log.pending_tile_changes.iter() {
            history_tiles.insert(*pos, self.board.get(pos));
            self.board.set(pos, &self.current_player);
            tiles.insert(*pos, self.board.get(pos));
        }
        if self.change_log.pending_player_change { self.current_player = self.current_player.opposite() }

        self.change_log.history.push(ChangeSet { tiles: history_tiles, player: history_player });
        
        self.change_log.discard_changes();

        ChangeSet { tiles: tiles, player: self.current_player }
    }

    fn revoke_changes(&mut self) -> Option<ChangeSet> {
        let change_set = self.change_log.history.pop()?;
        
        for (pos, tile) in change_set.tiles.iter() {
            match tile.0 {
                Some(color) => self.board.set(pos, &color),
                None => self.board.unset(pos)
            }
        }
        self.current_player = change_set.player;

        Some(change_set)
    }

    pub fn summary(&self) -> ChangeSet {
        ChangeSet{
            tiles: self.board.iter_all_positions()
                       .map(|pos| (pos, self.board.get(&pos)))
                       .collect(),
            player: self.current_player
        }
    }

    pub fn do_turn(&mut self, position: Position) -> Result {
        if self.board.taken(&position) { return self.error("Position already taken"); }
        
        let flip_positions = self.board.calculate_flip_positions(&position, &self.current_player);
        if flip_positions.is_empty() { return self.error("You must flip at least one tile"); }
        
        for pos in flip_positions.into_iter() {
            self.change_log.push_tile_change(pos);
        }
        self.change_log.push_tile_change(position);
        self.change_log.push_player_change();

        Ok(self.flush_changes())
    }

    pub fn cancel(&mut self) -> Result {
        match self.revoke_changes() {
            Some(change_set) => Ok(change_set),
            None => self.error("No more moves to cancel")
        }
    }
}