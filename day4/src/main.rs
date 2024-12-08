use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut grid = Vec::new();
    let fp = File::open("./input.txt").unwrap();
    //let fp = File::open("./small_input.txt").unwrap();
    //let fp = File::open("./smaller_input.txt").unwrap();
    for line in BufReader::new(fp).lines() {
        let line = line.unwrap();
        grid.push(line.chars().collect::<Vec<_>>());
    }
    dbg!(part2(grid));
}

fn part2(grid: Vec<Vec<char>>) -> usize {
    let mut count = 0;
    for x in 1..grid.len() - 1 {
        for y in 1..grid[0].len() - 1 {
            if grid[x][y] == 'A'
                && x_match(grid[x - 1][y - 1], grid[x][y], grid[x + 1][y + 1])
                && x_match(grid[x - 1][y + 1], grid[x][y], grid[x + 1][y - 1])
            {
                count += 1;
            }
        }
    }
    count
}

fn x_match(x: char, y: char, z: char) -> bool {
    matches!([x, y, z], ['M', 'A', 'S'] | ['S', 'A', 'M'])
}

fn part1(grid: Vec<Vec<char>>) -> usize {
    let mut count = 0;
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if y < grid[0].len() - 3
                && chars_match(grid[x][y], grid[x][y + 1], grid[x][y + 2], grid[x][y + 3])
            {
                count += 1;
            }
            if x < grid.len() - 3
                && chars_match(grid[x][y], grid[x + 1][y], grid[x + 2][y], grid[x + 3][y])
            {
                count += 1;
            }
            if y < grid[0].len() - 3
                && x < grid.len() - 3
                && chars_match(
                    grid[x][y],
                    grid[x + 1][y + 1],
                    grid[x + 2][y + 2],
                    grid[x + 3][y + 3],
                )
            {
                count += 1;
            }
            if y > 2
                && x < grid.len() - 3
                && chars_match(
                    grid[x][y],
                    grid[x + 1][y - 1],
                    grid[x + 2][y - 2],
                    grid[x + 3][y - 3],
                )
            {
                count += 1;
            }
        }
    }
    count
}

fn chars_match(x: char, y: char, z: char, w: char) -> bool {
    matches!([x, y, z, w], ['X', 'M', 'A', 'S'] | ['S', 'A', 'M', 'X'])
}
