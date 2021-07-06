extern crate rustymaze;

use rustymaze::maze::{Grid};

fn main() {
    let mut grid = Grid::new(5, 3);

    println!("{:?}", grid);
}