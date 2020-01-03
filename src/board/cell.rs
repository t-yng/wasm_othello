#[derive(Debug, Copy, Clone)]
pub enum CellState {
    EMPTY,
    BLACK,
    WHITE,
}

#[derive(Copy, Clone)]
pub struct Cell {
    pub state: CellState,
}