use arrayvec::ArrayVec;
use super::cell::Cell;
use super::cell::CellState;

struct Direction;

impl Direction {
    const LEFT_UP: i8 = -9;
    const UP: i8 = -8;
    const RIGHT_UP: i8 = -7;
    const LEFT: i8 = -1;
    const RIGHT: i8 = 1;
    const LEFT_DOWN: i8 = 7;
    const DOWN: i8 = 8;
    const RIGHT_DOWN: i8 = 9;

    pub fn directions () -> Vec<i8> {
        vec![
            Direction::LEFT_UP,
            Direction::UP,
            Direction::RIGHT_UP,
            Direction::LEFT,
            Direction::RIGHT,
            Direction::LEFT_DOWN,
            Direction::DOWN,
            Direction::RIGHT_DOWN
        ]
    }
}

pub fn get_available_positions (cells: [Cell; 64], stone: CellState) -> Vec<usize> {
    let empty_positions: Vec<usize> = cells.iter()
        .enumerate()
        .filter_map(
            |(index, &cell)| match cell.state {
                CellState::EMPTY => Some(index),
                _ => None
            }
        )
        .collect();

    empty_positions.into_iter()
        .filter(
            |&position| can_put_stone(cells, position, stone)
        )
        .collect()
}

pub fn flip_stones (cells: [Cell; 64], position: usize, stone: CellState) -> [Cell; 64] {
    let mut clone_cells = cells.clone();

    let flipped_positions = get_flipped_positions(clone_cells, position, stone);
    for &flipped_position in flipped_positions.iter() {
        clone_cells[flipped_position] = Cell::new(stone);
    }

    clone_cells
}

fn can_put_stone (cells: [Cell; 64], position: usize, stone: CellState) -> bool {
    let cell = cells[position];

    if cell.state != CellState::EMPTY {
        return false;
    }

    match count_flipped_stone(cells, position, stone) {
        0 => false,
        _ => true
    }
}

fn get_flipped_positions (cells: [Cell; 64], position: usize, stone: CellState) -> Vec<usize> {
    Direction::directions()
        .iter()
        .flat_map(
            |&d| get_flipped_direction_positions(cells, position, d, stone, vec![])
        )
        .collect()
}

fn count_flipped_stone (cells: [Cell; 64], position: usize, stone: CellState) -> usize {
    let positions = get_flipped_positions(cells, position, stone);
    positions.len()
}

fn get_flipped_direction_positions (cells: [Cell; 64], position: usize, direction: i8, stone: CellState, mut positions: Vec<usize>) -> Vec<usize> {
    if is_out_of_board(position, direction) {
        return vec![];
    }

    let next_position = match (position as i8) + direction {
        p if p >= 0 => p as usize,
        _ => return vec![],
    };
    let cell = cells[next_position];

    if cell.state == CellState::EMPTY {
        return vec![];
    }

    match cell.state == stone {
        true => positions,
        false => {
            positions.push(next_position);
            get_flipped_direction_positions(cells, next_position, direction, stone, positions)
        }
    }
}

fn is_left_end (position: usize) -> bool {
    position % 8 == 0
}

fn is_right_end (position: usize) -> bool {
    position % 8 == 7
}

fn is_out_of_board (position: usize, direction: i8) -> bool {
    if is_left_end(position) {
        match direction {
            Direction::LEFT | Direction::LEFT_UP | Direction::LEFT_DOWN => return true,
            _ => ()
        }
    }

    if is_right_end(position) {
        match direction {
            Direction::RIGHT | Direction::RIGHT_UP | Direction::RIGHT_DOWN => return true,
            _ => ()
        }
    }

    let next_position = (position as i8) + direction;
    match next_position {
        0..=63 => false,
        _ => true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Initialized Cells
    // let cells = vec![
    //     [0, 0, 0, 0, 0, 0, 0, 0],
    //     [0, 0, 0, 0, 0, 0, 0, 0],
    //     [0, 0, 0, 0, 0, 0, 0, 0],
    //     [0, 0, 0, 1, 2, 0, 0, 0],
    //     [0, 0, 0, 2, 1, 0, 0, 0],
    //     [0, 0, 0, 0, 0, 0, 0, 0],
    //     [0, 0, 0, 0, 0, 0, 0, 0],
    //     [0, 0, 0, 0, 0, 0, 0, 0],
    // ];

    fn create_cells (cells: Vec<[usize; 8]>) -> [Cell; 64] {
        cells.iter()
             .flatten()
             .map(
                 |&v| match v {
                    0 => Cell::new(CellState::EMPTY),
                    1 => Cell::new(CellState::WHITE),
                    2 => Cell::new(CellState::BLACK),
                    _ => panic!("盤面に不正な値が指定されました。0, 1, 2 で値をしてください。")
                }
            )
            .collect::<ArrayVec<[Cell; 64]>>()
            .into_inner()
            .unwrap_or([Cell::new(CellState::EMPTY); 64])
    }

    #[test]
    fn is_out_of_board_test () {
        // Boardの外側の判定
        assert_eq!(is_out_of_board(8, Direction::LEFT_UP), true);
        assert_eq!(is_out_of_board(8, Direction::LEFT), true);
        assert_eq!(is_out_of_board(8, Direction::LEFT_DOWN), true);
        assert_eq!(is_out_of_board(15, Direction::RIGHT_UP), true);
        assert_eq!(is_out_of_board(15, Direction::RIGHT), true);
        assert_eq!(is_out_of_board(15, Direction::RIGHT_DOWN), true);
        assert_eq!(is_out_of_board(1, Direction::UP), true);
        assert_eq!(is_out_of_board(62, Direction::DOWN), true);

        // Boardの内側の判定
        assert_eq!(is_out_of_board(55, Direction::LEFT_UP), false);
        assert_eq!(is_out_of_board(55, Direction::LEFT), false);
        assert_eq!(is_out_of_board(55, Direction::LEFT_DOWN), false);
        assert_eq!(is_out_of_board(8, Direction::RIGHT_UP), false);
        assert_eq!(is_out_of_board(8, Direction::RIGHT), false);
        assert_eq!(is_out_of_board(8, Direction::RIGHT_DOWN), false);
        assert_eq!(is_out_of_board(62, Direction::UP), false);
        assert_eq!(is_out_of_board(1, Direction::DOWN), false);

        // 境界値のテスト
        assert_eq!(is_out_of_board(62, Direction::RIGHT), false);
        assert_eq!(is_out_of_board(1, Direction::LEFT), false);
    }

    #[test]
    fn get_flipped_direction_positions_test () {
        let cells = create_cells(vec![
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 0, 0, 0],
            [0, 0, 0, 1, 1, 0, 0, 0],
            [0, 0, 0, 2, 2, 2, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
        ]);

        assert_eq!(
            get_flipped_direction_positions(cells, 42, Direction::RIGHT_UP, CellState::WHITE, vec![]),
            vec![35]
        );
    }

    #[test]
    fn get_flipped_positions_test () {
        let cells = create_cells(vec![
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 0, 0, 0],
            [0, 0, 0, 1, 1, 0, 0, 0],
            [0, 0, 0, 2, 2, 2, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
        ]);

        assert_eq!(
            get_flipped_positions(cells, 42, CellState::WHITE),
            vec![35]
        );
    }

    #[test]
    fn count_flipped_stone_test () {
        let cells = create_cells(vec![
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 0, 0, 0],
            [0, 0, 0, 1, 1, 0, 0, 0],
            [0, 0, 0, 2, 2, 2, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
        ]);

        assert_eq!(
            count_flipped_stone(cells, 42, CellState::WHITE),
            1
        );
    }

    #[test]
    fn can_put_stone_test () {
        let cells = create_cells(vec![
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 0, 0, 0],
            [0, 0, 0, 1, 1, 0, 0, 0],
            [0, 0, 0, 2, 2, 2, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
        ]);

        assert_eq!(
            can_put_stone(cells, 42, CellState::WHITE),
            true
        );
    }

    #[test]
    fn get_available_positions_test () {
        let cells = create_cells(vec![
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 0, 0, 0],
            [0, 0, 0, 1, 1, 0, 0, 0],
            [0, 0, 0, 2, 2, 2, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
        ]);

        let expected = vec![42, 43, 44, 45, 46];
        assert_eq!(get_available_positions(cells, CellState::WHITE), expected);
    }
}