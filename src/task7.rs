use std::io::{self, BufRead};

struct Directory {
    id: usize,
    name: String,
    files: Vec<File>,
    directories: Vec<Directory>,
}

struct File {
    name: String,
    size: usize,
}

impl Directory {
    fn new(name: String, id: usize) -> Directory {
        Directory {
            id,
            name,
            files: Vec::new(),
            directories: Vec::new(),
        }
    }

    fn add_file(&mut self, name: String, size: usize) {
        self.files.push(File { name, size });
    }

    fn add_directory(&mut self, directory: Directory) {
        self.directories.push(directory);
    }

    fn total_size(&self) -> usize {
        let file_sizes: usize = self.files.iter().map(|file| file.size).sum();
        let subdirectory_sizes: usize = self.directories.iter().map(|directory| directory.total_size()).sum();
        file_sizes + subdirectory_sizes
    }

    fn total_size_except(&self, except: usize) -> usize {
        let file_sizes: usize = self.files.iter().map(|file| file.size).sum();
        let subdirectory_sizes: usize = self.directories.iter().map(|directory| {
            if directory.id == except {
                0
            } else {
                directory.total_size_except(except)
            }
        }).sum();
        file_sizes + subdirectory_sizes
    }

    fn get_directory_mut(&mut self, name: &str) -> Option<&mut Directory> {
        // return &mut
        self.directories.iter_mut().find(|directory| directory.name == name)
    }

    fn get_directory_recursive(&mut self, name: &[String]) -> Option<&mut Directory> {
        if name.is_empty() {
            Some(self)
        } else {
            let directory_name = &name[0];
            let directory = self.get_directory_mut(directory_name);
            match directory {
                Some(directory) => directory.get_directory_recursive(&name[1..]),
                None => None,
            }
        }
    }

    fn print_tree(&self, depth: usize) {
        let indent = " ".repeat(depth * 2);
        println!("{}{}", indent, self.name);
        for file in &self.files {
            println!("{}{} {}", indent, file.name, file.size);
        }
        for directory in &self.directories {
            directory.print_tree(depth + 1);
        }
    }

    fn get_id_vec(&self) -> Vec<usize> {
        let mut id_vec = Vec::new();
        id_vec.push(self.id);
        for directory in &self.directories {
            id_vec.extend(directory.get_id_vec());
        }
        id_vec
    }

    fn get_directory_by_id(&mut self, id: usize) -> Option<&mut Directory> {
        if self.id == id {
            Some(self)
        } else {
            for directory in &mut self.directories {
                let result = directory.get_directory_by_id(id);
                if result.is_some() {
                    return result;
                }
            }
            None
        }
    }
}

fn parse_input(input: &str) -> Directory {
    let mut root = Directory::new("/".to_owned(), 0);
    let mut current_directory = &mut root;
    let mut path_buffer = Vec::new();
    let mut id = 1;

    for line in input.lines() {
        let line = line.trim_start_matches("$ ");
        if line.starts_with("cd /") {
            continue;
        }
        let tokens: Vec<&str> = line.trim().split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }
        match tokens[0] {
            "dir" => {
                let directory_name = tokens[1].to_owned();
                let directory = Directory::new(directory_name, id);
                id += 1;
                current_directory.add_directory(directory);
            }
            "cd" => {
                match tokens[1] {
                    ".." => {
                        path_buffer.pop();
                        current_directory = root.get_directory_recursive(&path_buffer).unwrap();
                    }
                    _ => {
                        let dir_name = tokens[1].to_owned();
                        // root.print_tree(0);
                        path_buffer.push(dir_name);
                        current_directory = root.get_directory_recursive(&path_buffer).unwrap();
                    }
                }
            }
            "ls" => {
                // empty 
            }
            _ => {
                let file_name = tokens[1].to_owned();
                let file_size = tokens[0].parse().unwrap();
                current_directory.add_file(file_name, file_size);
            }
        }
    }

    root
}

fn find_directories_with_size(directory: &Directory, max_size: usize) -> Vec<&Directory> {
    let mut result = Vec::new();

    if directory.total_size() <= max_size {
        result.push(directory);
    }

    for subdirectory in &directory.directories {
        result.extend(find_directories_with_size(subdirectory, max_size));
    }

    result
}

pub fn task7_part1_v1() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
        .join("\n");

    let root_directory = parse_input(&input);

    let max_size = 100000;
    let directories = find_directories_with_size(&root_directory, max_size);

    let total_sizes: usize = directories.iter().map(|directory| directory.total_size()).sum();

    println!("{}", total_sizes);
}

pub fn task7_part2_v1() {
    let stdin = io::stdin();
    let input = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
        .join("\n");

    let mut root_directory = parse_input(&input);

    //root_directory.print_tree(0);
    let id_vec = root_directory.get_id_vec();
    let mut smallest_id = 0;
    let mut smallest_size = 0;

    for id in id_vec {
        let size_without_id = root_directory.total_size_except(id);
        if size_without_id <= 40000000 && size_without_id > smallest_size {
            smallest_id = id;
            smallest_size = size_without_id;
        }
    }
    let directory = root_directory.get_directory_by_id(smallest_id).unwrap();
    let directory_size = directory.total_size();
    print!("{} {}", directory_size, directory.name);
}
