/// The size of all the rows and columns on the board
pub const BOARD_SIZE: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Piece {
    X,
    O,
}

/// An occupied tile of the board
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tile {
    /// The piece residing on this tile
    pub piece: Piece,
    /// true if the piece on this tile has been promoted to a king
    pub is_king: bool,
}

impl Tile {
    const fn new(piece: Piece) -> Self {
        Self {
            piece,
            is_king: false,
        }
    }
}

/// The tiles of a checkers board
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Board([[Option<Tile>; BOARD_SIZE]; BOARD_SIZE]);

const P_X: Option<Tile> = Some(Tile::new(Piece::X));
const P_O: Option<Tile> = Some(Tile::new(Piece::O));

/// The default setup of the checkers board
///
/// Note that the board is vertically reversed from how it is rendered because of how indexing works
pub(crate) const DEFAULT_SETUP: Board = Board([
    [P_X,  None, P_X,  None, P_X,  None, P_X,  None],
    [None, P_X,  None, P_X,  None, P_X,  None, P_X ],
    [P_X,  None, P_X,  None, P_X,  None, P_X,  None],
    [None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None],
    [None, P_O,  None, P_O,  None, P_O,  None, P_O ],
    [P_O,  None, P_O,  None, P_O,  None, P_O,  None],
    [None, P_O,  None, P_O,  None, P_O,  None, P_O ],
]);
