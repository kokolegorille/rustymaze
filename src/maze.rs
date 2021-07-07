use std::collections::HashMap;
use std::cmp;

use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Coordinate(pub usize, pub usize);

impl Coordinate {
    fn neighbors(&self) -> Vec<Coordinate> {
        let mut vec = Vec::new();

        let Coordinate(x, y) = *self;

        if x > 0 { vec.push(Coordinate(x - 1, y)); }
        if y > 0 { vec.push(Coordinate(x, y - 1)); }
        vec.push(Coordinate(x + 1, y));
        vec.push(Coordinate(x, y + 1));
        vec
    }
}

#[derive(Debug, PartialEq)]
pub struct Cell {
    coordinate: Coordinate,
    n: Coordinate,
    e: Coordinate,
    s: Coordinate,
    w: Coordinate,
}

impl Cell {
    pub fn new(coordinate: Coordinate) -> Self {
        let Coordinate(x, y) = coordinate;

        Self { 
            coordinate, 
            n: coordinate, 
            e: Coordinate(x + 1, y), 
            s: Coordinate(x, y + 1), 
            w: coordinate,
        }
    }
}

#[derive(Debug)]
pub enum Border {
    Wall,
    Passage,
}

#[derive(Debug)]
pub enum State {
    Valid,
    Uninitialized,
    InvalidWidthHeight,
    NoPaths,
}

#[derive(Debug)]
pub enum HorizontalVertical {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
pub struct Grid {
    width: usize,
    height: usize,
    cells: HashMap<Coordinate, Cell>,
    borders_h: HashMap<Coordinate, Border>,
    borders_v: HashMap<Coordinate, Border>,
    paths: HashMap<Coordinate, Cell>,
    state: State,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let mut cells = HashMap::new();
        let mut borders_h = HashMap::new();
        let mut borders_v = HashMap::new();

        for x in 0..width {
            for y in 0..height {
                let coordinate = Coordinate(x, y);
                cells.insert(coordinate, Cell::new(coordinate));
            }
        }

        for x in 0..(width - 1) {
            for y in 0..height {
                borders_h.insert(Coordinate(x, y), Border::Wall);
            }
        }

        for x in 0..width {
            for y in 0..(height - 1) {
                borders_v.insert(Coordinate(x, y), Border::Wall);
            }
        }

        Self {
            width,
            height,
            cells,
            borders_h,
            borders_v,
            paths: HashMap::new(),
            state: State::Uninitialized,
        }
    }

    pub fn open_passage(&mut self, current: Coordinate, last: Coordinate) {
        match self.maybe_horizontal_or_vertical(current, last) {
            Some(HorizontalVertical::Horizontal) => {
                self.borders_h.insert(Coordinate(last.0, cmp::max(current.1, last.1)), Border::Passage);
            },
            Some(HorizontalVertical::Vertical) => {
                self.borders_v.insert(Coordinate(cmp::max(current.0, last.0), last.1), Border::Passage);
            },
            None => (),
        }
    }

    fn maybe_horizontal_or_vertical(&self, current: Coordinate, last: Coordinate) -> Option<HorizontalVertical> {
        let abs_h: isize = (current.1 as isize - last.1 as isize).abs();
        let abs_v: isize = (current.0 as isize - last.0 as isize).abs();

        if current.0 == last.0 && abs_h == 1 {
            Some(HorizontalVertical::Horizontal)
        } else if current.1 == last.1 && abs_v == 1 {
            Some(HorizontalVertical::Vertical)
        } else {
            None
        }
    }

    fn neighbors(&self, coordinate: &Coordinate) -> Vec<Coordinate> {
        coordinate.neighbors()
            .into_iter()
            .filter(|c| self.is_on_grid(*c))
            .collect()
    }

    fn is_on_grid(&self, coordinate: Coordinate) -> bool {
        let Coordinate(x, y) = coordinate;
        x < self.width && y < self.height
    }
}

impl ToString for Grid {
    fn to_string(&self) -> String {
        let mut output = String::new();
        output.push('+');

        for _ in 0..self.width {
            output.push_str(&"---+");
        }
        output.push('\n');

        for y in 0..self.height {
            let mut top = String::from("|");
            let mut bottom = String::from("+");
            for x in 0..self.width {
                let body = "   ";

                let east_boundary = match self.borders_h.get(&Coordinate(x, y)) {
                    Some(Border::Passage) => " ",
                    Some(Border::Wall) => "|",
                    None => "|",
                }; 

                top.push_str(&body);
                top.push_str(&east_boundary);

                // let south_boundary = if true { "   " } else { "---"}; 

                let south_boundary = match self.borders_v.get(&Coordinate(x, y)) {
                    Some(Border::Passage) => "   ",
                    Some(Border::Wall) => "---",
                    None => "---",
                }; 

                bottom.push_str(&south_boundary);
                bottom.push('+');
            }
            output.push_str(&top[..]);
            output.push('\n');

            output.push_str(&bottom[..]);
            output.push('\n');
        }
        output
    }
}

pub struct RecursiveBacktrack {
    visited: HashMap<Coordinate, bool>
}

impl RecursiveBacktrack {
    pub fn new() -> Self {
        Self {
            visited: HashMap::new(),
        }
    }

    pub fn carve(&mut self, grid: &mut Grid) {
        let mut rng = thread_rng();

        let x: usize = rng.gen_range(0, grid.width);
        let y: usize = rng.gen_range(0, grid.height);

        let starting_coordinate = Coordinate(x, y);

        self.do_carve(grid, starting_coordinate, starting_coordinate)
    }

    fn do_carve(&mut self, grid: &mut Grid, current: Coordinate, last: Coordinate) {
        // println!("Carving... {:?} {:?}", current, last);

        match self.visited.get(&current) {
            Some(_) => (),
            None => {
                self.visited.insert(current, true);
                grid.open_passage(current, last);

                let mut neighbors = grid.neighbors(&current);
                neighbors.shuffle(&mut thread_rng());

                for neighbor in neighbors {
                    self.do_carve(grid, neighbor, current);
                }
            },
        }
    }
}