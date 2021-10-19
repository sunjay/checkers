mod coords;
mod board;

pub use board::*;
pub use coords::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Move {
    /// Move a single tile at the given position 1 step in the given direction to an empty tile
    Step {pos: Pos, dir: Direction},

    /// Perform a single jump from the given position in the given direction, capturing the opponent
    /// piece between the start and end tile, and landing on an empty tile
    Jump {pos: Pos, dir: Direction},

    /// Perform a series of jumps from the given position in the given directions, capturing
    /// opponent pieces on every jump, and landing on an empty tile
    Jumps {pos: Pos, dirs: Vec<Direction>},
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Checkers {
    board: Board,
    /// The piece that will move on the next turn
    current_piece: Piece,
}

impl Default for Checkers {
    fn default() -> Self {
        Self::new()
    }
}

impl Checkers {
    pub const fn new() -> Self {
        Self {
            board: DEFAULT_SETUP,
            current_piece: Piece::X,
        }
    }

    pub fn current_piece(&self) -> Piece {
        self.current_piece
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    /// Returns all the valid moves for the current piece
    ///
    /// Note that since jumps are forced, if any jump moves are available, only those will be returned
    pub fn valid_moves(&self) -> Vec<Move> {
        let valid = Vec::new();
        valid
    }
}
