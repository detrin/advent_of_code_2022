use std::{io::{self, BufRead}, hash::Hash, vec};
use std::collections::HashMap;
use std::hash::Hasher;
use std::collections::VecDeque;

#[derive(Eq, PartialEq, Hash, Debug)]
struct Block {
    x: i32,
    y: i32,
    z: i32,
}

impl Block {
    fn new(x: i32, y: i32, z: i32) -> Block {
        Block {
            x,
            y,
            z,
        }
    }

    fn get_sides(&self) -> Vec<Side> {
        vec![
            Side::new(self.x, self.y, self.z, 0),
            Side::new(self.x, self.y, self.z, 1),
            Side::new(self.x, self.y, self.z, 2),
            Side::new(self.x, self.y, self.z, 3),
            Side::new(self.x, self.y, self.z, 4),
            Side::new(self.x, self.y, self.z, 5),
        ]
    }
}


// direction: 0 - x, 1 - y, 2 - z, 3 - -x, 4 - -y, 5 - -z
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
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

    fn is_free(&self, space: &Space) -> bool {
        ! match self.direction {
            0 => space.is_block(self.x + 1, self.y, self.z),
            1 => space.is_block(self.x, self.y + 1, self.z),
            2 => space.is_block(self.x, self.y, self.z + 1),
            3 => space.is_block(self.x - 1, self.y, self.z),
            4 => space.is_block(self.x, self.y - 1, self.z),
            5 => space.is_block(self.x, self.y, self.z - 1),
            _ => panic!("Unknown direction"),
        }
    }

    fn is_adjacent(&self, side: &Side) -> bool {
        match self.direction {
            0 => self.x == side.x - 1 && self.y == side.y && self.z == side.z && side.direction == 3,
            1 => self.x == side.x && self.y == side.y - 1 && self.z == side.z && side.direction == 4,
            2 => self.x == side.x && self.y == side.y && self.z == side.z - 1 && side.direction == 5,
            3 => self.x == side.x + 1 && self.y == side.y && self.z == side.z && side.direction == 0,
            4 => self.x == side.x && self.y == side.y + 1 && self.z == side.z && side.direction == 1,
            5 => self.x == side.x && self.y == side.y && self.z == side.z + 1 && side.direction == 2,
            _ => panic!("Unknown direction"),
        }
    }

    fn get_next_block(&self) -> Block {
        match self.direction {
            0 => Block {
                x: self.x + 1,
                y: self.y,
                z: self.z,
            },
            1 => Block {
                x: self.x,
                y: self.y + 1,
                z: self.z,
            },
            2 => Block {
                x: self.x,
                y: self.y,
                z: self.z + 1,
            },
            3 => Block {
                x: self.x - 1,
                y: self.y,
                z: self.z,
            },
            4 => Block {
                x: self.x,
                y: self.y - 1,
                z: self.z,
            },
            5 => Block {
                x: self.x,
                y: self.y,
                z: self.z - 1,
            },
            _ => panic!("Unknown direction"),
        }
    }

    fn get_adjacent_side(&self) -> Side {
        match self.direction {
            0 => Side {
                x: self.x + 1,
                y: self.y,
                z: self.z,
                direction: 3,
            },
            1 => Side {
                x: self.x,
                y: self.y + 1,
                z: self.z,
                direction: 4,
            },
            2 => Side {
                x: self.x,
                y: self.y,
                z: self.z + 1,
                direction: 5,
            },
            3 => Side {
                x: self.x - 1,
                y: self.y,
                z: self.z,
                direction: 0,
            },
            4 => Side {
                x: self.x,
                y: self.y - 1,
                z: self.z,
                direction: 1,
            },
            5 => Side {
                x: self.x,
                y: self.y,
                z: self.z - 1,
                direction: 2,
            },
            _ => panic!("Unknown direction"),
        }
    }

    fn get_neighbor_sides(&self, space: &Space) -> Vec<Side> {
        let direction_dim = self.direction % 3;
        let direction_way = self.direction / 3;
        let mut sides = Vec::new();
        // if direction_dim == 0 so it is x then we need to use (x, y, z+1), (x, y, z-1), (x, y+1, z), (x, y-1, z)
        // the direction stays the same 
        let iterate_diff = vec![(1, 0, 0), (0, 1, 0), (0, 0, 1), (-1, 0, 0), (0, -1, 0), (0, 0, -1)];
        for i in 0..6 {
            if i % 3 == direction_dim {
                continue;
            }
            let (x, y, z) = iterate_diff[i as usize];
            // println!("direction_dim: {}, direction_way: {}, i: {}, direction: {}", direction_dim, direction_way, i, self.direction);
            // println!("x: {}, y: {}, z: {}", x, y, z);
            let side = Side {
                x: self.x + x,
                y: self.y + y,
                z: self.z + z,
                direction: self.direction,
            };
            if side.is_free(space) {
                sides.push(side);
            }
        }

        // println!("side: {:?}", self);
        // println!("next block: {:?}", self.get_next_block());
        let next_block = self.get_next_block();
        let next_block_sides = next_block.get_sides();
        // use is_adjacent
        for side in next_block_sides {
            let adjacent_side = side.get_adjacent_side();
            if side.direction != self.direction && side.direction != self.direction + 3 && adjacent_side.is_free(space) {
                sides.push(adjacent_side);
                // sides.push(side);
            }
        }

        let block = Block {
            x: self.x,
            y: self.y,
            z: self.z,
        };
        let block_sides = block.get_sides();
        for side in block_sides {
            if side.direction != self.direction && side.direction != self.direction + 3 && side.is_free(space) {
                let block = side.get_next_block();
                let side_block_orthogonal = Side {
                    x: block.x,
                    y: block.y,
                    z: block.z,
                    direction: side.direction,
                };
                if side_block_orthogonal.is_free(space) {
                    sides.push(side);
                }
            }
        }
        // print all sides
        // for side in &sides {
        //     println!("neighbor side: {:?}", side);
        // }

        sides
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
                if side.is_free(&self) {
                    inactive_sides += 1;
                }
            }
            inactive += inactive_sides;
        }
        inactive
    }

}

fn BFS_sides(space: &Space) {
    let mut visited = HashMap::new();
    let mut side_run_number = HashMap::new();
    let mut available_sides = HashMap::new();
    let mut runs = 0;
    for (block, _) in space.blocks.iter() {
        for direction in 0..6 {
            let side = Side::new(block.x, block.y, block.z, direction);
            if side.is_free(&space) && !visited.contains_key(&side) {
                available_sides.insert(side, true);
                visited.insert(side, false);
            }
        }
    }

    for (side, _) in &available_sides {
        if !visited[side] {
            let mut queue = VecDeque::new();
            queue.push_back(Box::new(*side));
            while let Some(side) = queue.pop_front() {
                visited.insert(*side, true);
                side_run_number.insert(*side, runs);
                let neighbors = side.get_neighbor_sides(space);
                for neighbor in neighbors {
                    // println!("neighbor: {:?}", neighbor);
                    // println!("is available: {}", available_sides.contains_key(&neighbor));
                    // if available_sides.contains_key(&neighbor) {
                    //     println!("is visited: {}", visited[&neighbor]);
                    // }
                    if available_sides.contains_key(&neighbor) && !visited[&neighbor] {
                        queue.push_back(Box::new(neighbor));
                    }
                }
            }
            runs += 1;
        }
    }

    let mut run_count = HashMap::new();
    for (_, run_number) in side_run_number {
        if !run_count.contains_key(&run_number) {
            run_count.insert(run_number, 1);
        } else {
            run_count.insert(run_number, run_count[&run_number] + 1);
        }
    }

    for (run_number, count) in run_count {
        println!("Run number: {}, count: {}", run_number, count);
    }
}


pub fn task18_part1_v1() {
    let stdin = io::stdin();
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

pub fn task18_part2_v1() {
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
    BFS_sides(&space);

}