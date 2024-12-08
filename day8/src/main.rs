use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut grid = Vec::new();
    let fp = File::open("./input.txt").unwrap();
    //let fp = File::open("./small_input.txt").unwrap();
    for line in BufReader::new(fp).lines() {
        let line = line.unwrap();
        grid.push(line.chars().collect::<Vec<_>>());
    }
    println!("{}", part2(grid));
}

fn part2(grid: Vec<Vec<char>>) -> usize {
    let mut map = HashMap::new();
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            let char = grid[x][y];
            map.entry(char).or_insert(vec![]).push((x, y));
        }
    }

    let mut new_grid = grid.clone();
    new_grid.iter_mut().flatten().for_each(|e| *e = '.');

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] == '.' {
                continue;
            }
            for &(other_x, other_y) in map.get(&grid[x][y]).unwrap() {
                let delta_x = other_x as i32 - x as i32;
                let delta_y = other_y as i32 - y as i32;

                // Just... try a lot lmao
                for i in -100..100 {
                    let new_x = i * delta_x + other_x as i32;
                    let new_y = i * delta_y + other_y as i32;

                    if !(0..grid.len() as i32).contains(&new_x)
                        || !(0..grid[0].len() as i32).contains(&new_y)
                    {
                        continue;
                    }

                    new_grid[new_x as usize][new_y as usize] = '#';
                }
            }
        }
    }
    new_grid.into_iter().flatten().filter(|c| *c == '#').count()
}

fn part1(grid: Vec<Vec<char>>) -> usize {
    let mut map = HashMap::new();
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            let char = grid[x][y];
            map.entry(char).or_insert(vec![]).push((x, y));
        }
    }

    let mut new_grid = grid.clone();
    new_grid.iter_mut().flatten().for_each(|e| *e = '.');

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] == '.' {
                continue;
            }
            for &(other_x, other_y) in map.get(&grid[x][y]).unwrap() {
                let delta_x = other_x as i32 - x as i32;
                let delta_y = other_y as i32 - y as i32;

                let new_x = delta_x + other_x as i32;
                let new_y = delta_y + other_y as i32;

                if !(0..grid.len() as i32).contains(&new_x)
                    || !(0..grid[0].len() as i32).contains(&new_y)
                {
                    continue;
                }

                if grid[new_x as usize][new_y as usize] == grid[x][y] {
                    continue;
                }

                new_grid[new_x as usize][new_y as usize] = '#';
            }
        }
    }

    new_grid.into_iter().flatten().filter(|c| *c == '#').count()
}
