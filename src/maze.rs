use std::{fs::File, io::{BufRead, BufReader}};

pub type Maze = Vec<Vec<char>>;

pub fn load_maze(filename: &str) -> Maze {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

pub fn get_cell(maze: &Maze, x: f32, y: f32, block_size: usize) -> Option<char> {
    let i = x as usize / block_size;
    let j = y as usize / block_size;

    maze.get(j).and_then(|row| row.get(i)).copied()
}
