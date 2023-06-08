use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};
use std::cmp::{min, max};
use std::fmt;
use indicatif::ProgressBar;
use std::ptr::hash;

// make grid and save into hasmap only points that are set
// add boundaries as lines
struct Grid {
    points: HashMap<(i32, i32), bool>,
    boundaries: Vec<(i32, i32, i32, i32)>,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            points: HashMap::new(),
            boundaries: Vec::new(),
        }
    }

    fn set(&mut self, x: i32, y: i32) {
        self.points.insert((x, y), true);
    }

    fn get(&self, x: i32, y: i32) -> bool {
        for boundary in self.boundaries.iter() {
            if x >= boundary.0 && x <= boundary.2 && y >= boundary.1 && y <= boundary.3 {
                return true;
            }
        }
        match self.points.get(&(x, y)) {
            Some(_) => true,
            None => false,
        }
    }

    fn remove(&mut self, x: i32, y: i32) {
        self.points.remove(&(x, y));
    }

    fn get_max_height(&self) -> i32 {
        let mut max_height = 0;
        for point in self.points.keys() {
            if point.1 > max_height {
                max_height = point.1;
            }
        }
        max_height
    }

    fn print(&self) {
        let mut min_x = 0;
        let mut min_y = 0;
        let mut max_x = 0;
        let mut max_y = 0;
        for point in self.points.keys() {
            if point.1 < min_y {
                min_y = point.1;
            }
            if point.1 > max_y {
                max_y = point.1;
            }
        }
        min_x = 0;
        max_x = 8;
        max_y += 5;
        min_y -= 0;
        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                if self.get(x, y) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    // autoclean and keep only top N rows
    fn autoclean(&mut self, n: i32) {
        // get top height
        let mut max_height = 0;
        for point in self.points.keys() {
            if point.1 > max_height {
                max_height = point.1;
            }
        }
        // remove all points that are below max_height - n
        let min_height = max_height - n;
        let mut to_remove = Vec::new();
        for point in self.points.keys() {
            if point.1 < min_height {
                to_remove.push(*point);
            }
        }
        for point in to_remove.iter() {
            self.points.remove(point);
        }
    }

    // get height for each column
    fn get_heights(&self) -> Vec<i32> {
        let mut heights = Vec::new();
        // iterate over all points and find max height for each column
        for point in self.points.keys() {
            let x = point.0;
            let y = point.1;
            if x >= heights.len() as i32 {
                heights.push(y);
            } else {
                if y > heights[x as usize] {
                    heights[x as usize] = y;
                }
            }
        }
        heights.remove(0);
        heights
    }

    fn get_heghts_relative(&self) -> Vec<i32> {
        let heights = self.get_heights();
        let mut relative_heights = Vec::new();
        let mut min_height = 1000000;
        for height in heights.iter() {
            if *height < min_height {
                min_height = *height;
            }
        }
        for height in heights.iter() {
            relative_heights.push(height - min_height);
        }
        relative_heights
    }
}

// make piece that has set of devined points
struct Piece {
    points: HashSet<(i32, i32)>,
}

impl Piece {
    fn new() -> Piece {
        Piece {
            points: HashSet::new(),
        }
    }

    fn add(&mut self, x: i32, y: i32) {
        self.points.insert((x, y));
    }

    fn get(&self, x: i32, y: i32) -> bool {
        match self.points.get(&(x, y)) {
            Some(_) => true,
            None => false
        }
    }

    fn move_piece(&mut self, x: i32, y: i32) {
        let mut new_points = HashSet::new();
        for point in self.points.iter() {
            new_points.insert((point.0 + x, point.1 + y));
        }
        self.points = new_points;
    }

    fn create_piece(piece_type: i32) -> Piece {
        let mut piece = Piece::new();
        match piece_type {
            1 => {
                piece.add(0, 0);
                piece.add(1, 0);
                piece.add(2, 0);
                piece.add(3, 0);
            }
            2 => {
                piece.add(1, 0);
                piece.add(0, 1);
                piece.add(1, 1);
                piece.add(2, 1);
                piece.add(1, 2);
            }
            3 => {
                piece.add(0, 0);
                piece.add(1, 0);
                piece.add(2,0);
                piece.add(2, 2);
                piece.add(2, 1);
                
            }
            4 => {
                piece.add(0, 0);
                piece.add(0, 1);
                piece.add(0, 2);
                piece.add(0, 3);
            }
            5 => {
                piece.add(0, 0);
                piece.add(0, 1);
                piece.add(1, 0);
                piece.add(1, 1);
            }
            _ => panic!("Wrong piece type"),
        }
        piece
    }

    fn print(&self) {
        let mut min_x = 0;
        let mut min_y = 0;
        let mut max_x = 0;
        let mut max_y = 0;
        for point in self.points.iter() {
            if point.0 < min_x {
                min_x = point.0;
            }
            if point.0 > max_x {
                max_x = point.0;
            }
            if point.1 < min_y {
                min_y = point.1;
            }
            if point.1 > max_y {
                max_y = point.1;
            }
        }
        for y in (min_y..max_y + 1).rev() {
            for x in min_x..max_x + 1 {
                if self.get(x, y) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}

// test if piece can be placed on grid
fn test_piece(grid: &Grid, piece: &Piece) -> bool {
    for point in piece.points.iter() {
        if grid.get(point.0, point.1) {
            return false;
        }
    }
    true
}

// place piece on grid
fn place_piece(grid: &mut Grid, piece: &Piece) {
    for point in piece.points.iter() {
        grid.set(point.0, point.1);
    }
}

fn remove_piece(grid: &mut Grid, piece: &Piece) {
    for point in piece.points.iter() {
        grid.remove(point.0, point.1);
    }
}

// create enum for directions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Direction::Right => write!(f, "Right"),
            Direction::Left => write!(f, "Left"),
            Direction::Up => write!(f, "Up"),
            Direction::Down => write!(f, "Down"),
        }
    }
}

impl Direction {
    fn get_all() -> Vec<Direction> {
        vec![Direction::Right, Direction::Left, Direction::Up, Direction::Down]
    }

    fn to_numeric(&self) -> i32 {
        match *self {
            Direction::Right => 0,
            Direction::Left => 1,
            Direction::Up => 2,
            Direction::Down => 3,
        }
    }
}

fn move_to_direction(grid: &Grid, piece: &mut Piece, direction: Direction) -> bool {
    let mut moved = true;
    if direction == Direction::Left || direction == Direction::Right {
        let mut dx = 0;
        if direction == Direction::Left {
            dx = -1;
        } else {
            dx = 1;
        } 
        piece.move_piece(dx, 0);
        // check if piece can be placed
        if !test_piece(&grid, &piece) {
            piece.move_piece(-dx, 0);
            moved = false;
        } 
    } else {
        let mut dy = 0;
        if direction == Direction::Down {
            dy = -1;
        } else {
            dy = 1;
        } 
        piece.move_piece(0, dy);
        // check if piece can be placed
        if !test_piece(&grid, &piece) {
            piece.move_piece(0, -dy);
            moved = false;
        } 
    }
    moved
}


pub fn task17_part1_v1() {
    let stdin = io::stdin();
    // parse input
    // >>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
    // into vector of directions < - left and > - right
    // parse it with match
    let directions = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Wrong direction"),
        })
        .collect::<Vec<Direction>>();

    println!("{:?}", directions);
    for pi in 1..5 {
        let piece = Piece::create_piece(pi);
        piece.print();
        println!("----------------");
    }

    let mut grid = Grid::new();
    // add boundary
    grid.boundaries.push((0, 0, 8, 0));
    grid.boundaries.push((0, 0, 0, 10000));
    grid.boundaries.push((8, 0, 8, 10000));

    let mut piece_type = 0;
    let mut di = 0;
    let mut placed_cnt = 0;
    while placed_cnt < 2022 {
        let mut piece = Piece::create_piece(piece_type + 1);
        let starting_height = max(grid.get_max_height(), 0) + 4;
        piece.move_piece(3, starting_height);
        // place_piece(&mut grid, &piece);
        // grid.print();

        // let mut piece_move = 0;

        // place_piece(&mut grid, &piece);
        // println!("----------------");
        // println!("di: {}, piece_move: {}", di, piece_move);
        // println!("piece_type: {}", piece_type);
        
        // grid.print();
        // remove_piece(&mut grid, &piece);

        let mut moved_down = true;

        while moved_down {
            let direction = directions[di];
            
            // piece_move += 1;
            move_to_direction(&grid, &mut piece, direction);
            // place_piece(&mut grid, &piece);
            // grid.print();
            // test if it can be moved down 
            // place_piece(&mut grid, &piece);
            // println!("----------------");
            // println!("di: {}, piece_move: {}", di, piece_move);
            // println!("direction: {}", direction);
            // grid.print();
            // remove_piece(&mut grid, &piece);

            moved_down = move_to_direction(&grid, &mut piece, Direction::Down);
            
            di = (di + 1) % directions.len();
            
        }
        place_piece(&mut grid, &piece);
        piece_type = (piece_type + 1) % 5;
        placed_cnt += 1;
        // println!("+----------------+");
        //grid.print();
        // print max height
        
    }
    
    println!("max height: {}", grid.get_max_height());

}


pub fn task17_part2_v1() {
    let stdin = io::stdin();
    // parse input
    // >>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
    // into vector of directions < - left and > - right
    // parse it with match
    let directions = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Wrong direction"),
        })
        .collect::<Vec<Direction>>();

    println!("{:?}", directions);
    for pi in 1..5 {
        let piece = Piece::create_piece(pi);
        piece.print();
        println!("----------------");
    }

    let mut grid = Grid::new();
    // add boundary
    grid.boundaries.push((0, 0, 8, 0));
    grid.boundaries.push((0, 0, 0, 10000));
    grid.boundaries.push((8, 0, 8, 10000));

    let mut cycle_found = false;

    let mut piece_type = 0;
    let mut di = 0;
    let mut placed_cnt: u64 = 0;
    let bar = ProgressBar::new(1000000000000);
    while placed_cnt < 1000000000000 {
        let mut piece = Piece::create_piece(piece_type + 1);
        let starting_height = max(grid.get_max_height(), 0) + 4;
        piece.move_piece(3, starting_height);
        // place_piece(&mut grid, &piece);
        // grid.print();

        // let mut piece_move = 0;

        // place_piece(&mut grid, &piece);
        // println!("----------------");
        // println!("di: {}, piece_move: {}", di, piece_move);
        // println!("piece_type: {}", piece_type);
        
        // grid.print();
        // remove_piece(&mut grid, &piece);

        let mut moved_down = true;

        while moved_down {
            let direction = directions[di];
            
            // piece_move += 1;
            move_to_direction(&grid, &mut piece, direction);
            // place_piece(&mut grid, &piece);
            // grid.print();
            // test if it can be moved down 
            // place_piece(&mut grid, &piece);
            // println!("----------------");
            // println!("di: {}, piece_move: {}", di, piece_move);
            // println!("direction: {}", direction);
            // grid.print();
            // remove_piece(&mut grid, &piece);

            moved_down = move_to_direction(&grid, &mut piece, Direction::Down);
            
            di = (di + 1) % directions.len();
            
        }
        place_piece(&mut grid, &piece);
        piece_type = (piece_type + 1) % 5;
        placed_cnt += 1;
        // println!("+----------------+");
        //grid.print();
        // print max height
        // grid.autoclean(10000);
        bar.inc(1);
        let mut grid_states = HashMap::new();
        if !cycle_found {
            let numeric_dir = directions[di].to_numeric();
            let state_vec = grid.get_heghts_relative();
            // whole state is 1D vector numeric_dir, piece_type and state_vec
            let mut state = vec![numeric_dir, piece_type];
            state.extend(state_vec);
            println!("state: {:?}", state);
            if grid_states.contains_key(&state) {
                cycle_found = true;
                let cycle_start = grid_states.get(&state).unwrap();
                let cycle_len = placed_cnt - cycle_start;
                let remaining = 1000000000000 - placed_cnt;
                let cycle_remaining = remaining % cycle_len;
                placed_cnt += remaining - cycle_remaining;
                println!("cycle found: {} {}", cycle_start, cycle_len);
            } else {
                grid_states.insert(state, placed_cnt);
            }

        }
        
    }
    
    println!("max height: {}", grid.get_max_height());

}