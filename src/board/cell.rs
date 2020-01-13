#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CellState {
    EMPTY,
    BLACK,
    WHITE,
}

#[derive(Copy, Clone)]
pub struct Cell {
    pub state: CellState,
}

impl Cell {
    pub fn new (state: CellState) -> Cell {
        Cell {
            state: state
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_cell () {
        let cell = Cell::new(CellState::EMPTY);
        assert_eq!(cell.state, CellState::EMPTY);
    }
}