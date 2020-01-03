mod cell;

use cell::Cell;
use cell::CellState;

pub struct Board {
    pub cells: [Cell; 32],
}

impl Board {
    pub fn new () -> Board {
        Board {
            cells: [Cell {state: CellState::EMPTY}; 32],
        }
    }
}