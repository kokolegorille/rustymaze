extern crate rustymaze;

use rustymaze::maze::{Grid, RecursiveBacktrack};

fn main() {
    let mut grid = Grid::new(20, 20);
    let mut generator = RecursiveBacktrack::new();

    generator.carve(&mut grid);

    println!("{}", grid.to_string());

    // println!("{:?}", grid.borders_h);
    // println!("-----------------------------------");
    // println!("{:?}", grid.borders_v);
}