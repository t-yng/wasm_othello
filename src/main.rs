mod board;

fn main() {
    let b = board::Board::new();
    println!("{:?}", &b.cells[10].state);
}
