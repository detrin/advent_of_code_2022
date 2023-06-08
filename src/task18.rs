use std::{io::{self, BufRead}, hash::Hash};
use std::collections::HashMap;
use std::hash::Hasher;

// 2,2,2
// 1,2,2
// 3,2,2
// 2,1,2
// 2,3,2
// 2,2,1
// 2,2,3
// 2,2,4
// 2,2,6
// 1,2,5
// 3,2,5
// 2,1,5
// 2,3,5
// hashmap of (x, y, z)

#[derive(Eq, PartialEq, Hash)]
struct Block {
    x: i32,
    y: i32,
    z: i32,
}

// direction: 0 - x, 1 - y, 2 - z, 3 - -x, 4 - -y, 5 - -z
#[derive(Eq, PartialEq, Hash)]
struct Side {
    x: i32,
    y: i32,
    z: i32,
    direction: i32,
}

impl Side {
    fn new(x: i32, y: i32, z: i32, direction: i32) -> Side {
        Side {
            x,
            y,
            z,
            direction,
        }
    }

    fn is_active(&self, space: &Space) -> bool {
        match self.direction {
            0 => space.is_block(self.x + 1, self.y, self.z),
            1 => space.is_block(self.x, self.y + 1, self.z),
            2 => space.is_block(self.x, self.y, self.z + 1),
            3 => space.is_block(self.x - 1, self.y, self.z),
            4 => space.is_block(self.x, self.y - 1, self.z),
            5 => space.is_block(self.x, self.y, self.z - 1),
            _ => panic!("Unknown direction"),
        }
    }

    fn is_adjascent(&self, other: &Side) -> bool {
        // check is sides of block shar eone side
        if self.x == other.x && self.y == other.y && self.z == other.z {
            // directions can't be opposite
            if self.direction + other.direction == 5 {
                return false;
            }
            // directions can't be the same
            if self.direction == other.direction {
                return false;
            }
            return true;
        }

        // check is sides of block are on the same line and z
        if self.x == other.x && self.y == other.y  {
            if self.direction == 0 && other.direction == 3 {
                return true;
            }
            if self.direction == 3 && other.direction == 0 {
                return true;
            }
            if self.direction == 1 && other.direction == 4 {
                return true;
            }
            if self.direction == 4 && other.direction == 1 {
                return true;
            }
            if self.direction == 2 && other.direction == 5 {
                return true;
            }
            if self.direction == 5 && other.direction == 2 {
                return true;
            }
        }
    }
}

struct Space {
    blocks: HashMap<Block, bool>,
}

impl Space {
    fn new() -> Space {
        Space {
            blocks: HashMap::new(),
        }
    }

    fn add_block(&mut self, x: i32, y: i32, z: i32) {
        self.blocks.insert(Block { x, y, z }, true);
    }

    fn remove_block(&mut self, x: i32, y: i32, z: i32) {
        self.blocks.remove(&Block { x, y, z });
    }

    fn is_block(&self, x: i32, y: i32, z: i32) -> bool {
        self.blocks.contains_key(&Block { x, y, z })
    }

    fn count_active(&self) -> usize {
        self.blocks.len()
    }

    // write function that will found how many sides are without any adjacent block
    fn count_free_sides(&self) -> usize {
        let mut inactive = 0;
        for (block, _) in self.blocks.iter() {
            let mut inactive_sides = 0;
            for direction in 0..6 {
                let side = Side::new(block.x, block.y, block.z, direction);
                if !side.is_active(&self) {
                    inactive_sides += 1;
                }
            }
            inactive += inactive_sides;
        }
        inactive
    }

    // BFS over sides and get all inactive sides in vectors that are adjacent
    fn count_free_sides_v2(&self) -> usize {
        let mut inactive = 0;
        let mut visited = HashMap::new();
        for (block, _) in self.blocks.iter() {
            let mut inactive_sides = 0;
            for direction in 0..6 {
                let side = Side::new(block.x, block.y, block.z, direction);
                if !side.is_active(&self) {
                    inactive_sides += 1;
                }
            }
            inactive += inactive_sides;
        }
        inactive
    }

}

// iterate over all sides and check if there is a block

pub fn task18_part1_v1() {
    let stdin = io::stdin();
    // parse "2,3,5"
    let lines = stdin.lock().lines().map(|line| {
        let line = line.unwrap();
        let mut split = line.split(",");
        let x = split.next().unwrap().parse::<i32>().unwrap();
        let y = split.next().unwrap().parse::<i32>().unwrap();
        let z = split.next().unwrap().parse::<i32>().unwrap();
        (x, y, z)
    }).collect::<Vec<_>>();

    let mut space = Space::new();
    for (x, y, z) in lines {
        space.add_block(x, y, z);
    }

    let mut active = space.count_active();
    let mut free_sides = space.count_free_sides();
    println!("active: {}, inactive: {}", active, free_sides);
    
}