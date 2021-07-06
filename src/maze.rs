use std::collections::HashMap;

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
pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,

    // borders_h: Vec<Border>,
    // borders_v: Vec<Border>,
    // paths: Vec<Cell>,

    // borders: HashMap<char, Vec<Border>>,
    borders_h: HashMap<Coordinate, Border>,
    borders_v: HashMap<Coordinate, Border>,

    paths: HashMap<Coordinate, Cell>,

    state: State,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let mut cells = vec![];

        // let mut borders_h = vec![];
        // let mut borders_v = vec![];

        // let mut borders = HashMap::new();
        // let mut borders_h = vec![];
        // let mut borders_v = vec![];

        let mut borders_h = HashMap::new();
        let mut borders_v = HashMap::new();

        for y in 0..height {
            for x in 0..width {
                cells.push(Cell::new(Coordinate(x, y)));

                // borders_h.push(Border::Wall);
                // borders_v.push(Border::Wall);
            }
        }

        for x in 0..(width - 1) {
            for y in 0..height {
                // borders.insert(Coordinate(x, y), Border::Wall);
                // borders.insert('h', Border::Wall);

                // borders_h.push(Border::Wall);

                borders_h.insert(Coordinate(x, y), Border::Wall);
            }
        }

        for x in 0..width {
            for y in 0..(height - 1) {
                // borders.insert('v', Border::Wall);

                // borders_v.push(Border::Wall);

                borders_v.insert(Coordinate(x, y), Border::Wall);
            }
        }

        // borders.insert('h', borders_h);
        // borders.insert('v', borders_v);

        Self {
            width,
            height,
            cells,
            borders_h,
            borders_v,
            // borders,
            // paths: vec![],
            paths: HashMap::new(),
            state: State::Uninitialized,
        }
    }

    pub fn open_passage(&mut self) {
            
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

    fn coordinate_to_index(&self, coordinate: Coordinate) -> usize {
        let Coordinate(x, y) = coordinate;
        x * self.width + y
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
        let starting_coordinate = Coordinate(3, 3);
        self.do_carve(grid, starting_coordinate, starting_coordinate)
    }

    fn do_carve(&mut self, grid: &mut Grid, current_coordinate: Coordinate, last_coordinate: Coordinate) {
        match self.visited.get(&current_coordinate) {
            Some(_) => (),
            None => {
                self.visited.insert(current_coordinate, true);
                grid.open_passage();
            },
        }
    }
}