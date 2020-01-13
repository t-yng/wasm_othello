mod cell;
mod simulator;

use cell::Cell;
use cell::CellState;
use simulator::flip_stones;

pub struct Board {
    pub cells: Vec<Cell>,
}

impl Board {
    pub fn new () -> Board {
        Board {
            cells: Board::initialize_cells()
        }
    }

    fn initialize_cells () -> Vec<Cell> {
        let mut cells = vec![Cell::new(CellState::EMPTY); 64];
        cells[27] = Cell::new(CellState::BLACK);
        cells[36] = Cell::new(CellState::BLACK);
        cells[28] = Cell::new(CellState::WHITE);
        cells[35] = Cell::new(CellState::WHITE);
        cells
    }

    pub fn put_stone (&mut self, position: usize, stone: CellState) {
        self.cells = flip_stones(&self.cells, position, stone);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_board_instance () {
        let board = Board::new();
        assert_eq!(board.cells[0].state, CellState::EMPTY);
        assert_eq!(board.cells[27].state, CellState::BLACK);
        assert_eq!(board.cells[36].state, CellState::BLACK);
        assert_eq!(board.cells[28].state, CellState::WHITE);
        assert_eq!(board.cells[35].state, CellState::WHITE);
    }
}