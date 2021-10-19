//! Defines types for working with the coordinate system

use crate::board::BOARD_SIZE;

/// One of the four diagonal directions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    NE,
    SE,
    SW,
    NW,
}

impl Direction {
    pub const ALL: &'static [Self] = &[Self::NE, Self::SE, Self::SW, Self::NW];
    pub const NORTH: &'static [Self] = &[Self::NE, Self::NW];
    pub const SOUTH: &'static [Self] = &[Self::SE, Self::SW];
}

/// A row and column position on the board
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub row: usize,
    pub col: usize,
}

impl Pos {
    /// Shift this position the given number of times in the given direction
    ///
    /// Returns `None` if the position would go out of bounds
    pub fn shift(self, shifts: isize, direction: Direction) -> Option<Self> {
        let row = self.row as isize;
        let col = self.col as isize;

        fn clamp(value: isize) -> Option<usize> {
            if value >= 0 && value < BOARD_SIZE as isize {
                Some(value as usize)
            } else {
                None
            }
        }

        Some(match direction {
            Direction::NE => Pos {
                row: clamp(row + shifts)?,
                col: clamp(col + shifts)?,
            },
            Direction::SE => Pos {
                row: clamp(row - shifts)?,
                col: clamp(col + shifts)?,
            },
            Direction::SW => Pos {
                row: clamp(row - shifts)?,
                col: clamp(col - shifts)?,
            },
            Direction::NW => Pos {
                row: clamp(row + shifts)?,
                col: clamp(col - shifts)?,
            },
        })
    }
}
