use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let fp = File::open("./example.txt").unwrap();
    let fp = File::open("input.txt").unwrap();
    let grid = BufReader::new(fp)
        .lines()
        .map_while(Result::ok)
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("{}", part1(&grid));
}

fn part2(grid: &[Vec<char>]) -> usize {
    let mut total = 0;
    let mut seen = HashSet::new();
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            let mut fences = 0;
            let mut area = 0;
            part2_helper(grid, (0, 0), x, y, &mut seen, &mut fences, &mut area);
            total += fences * area;
        }
    }
    total
}

fn part2_helper(
    grid: &[Vec<char>],
    prev_offset: (isize, isize),
    x: usize,
    y: usize,
    seen: &mut HashSet<(usize, usize)>,
    fences: &mut usize,
    area: &mut usize,
) {
    if seen.contains(&(x, y)) {
        return;
    }
    seen.insert((x, y));
    *area += 1;
    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;
        if !(0..grid.len() as isize).contains(&new_x)
            || !(0..grid[0].len() as isize).contains(&new_y)
        {
            *fences += 1;
            continue;
        }
        if grid[new_x as usize][new_y as usize] != grid[x][y] {
            *fences += 1;
        } else {
            part2_helper(
                grid,
                (dx, dy),
                new_x as usize,
                new_y as usize,
                seen,
                fences,
                area,
            );
        }
    }
}

fn fences(grid: &[Vec<char>], x: usize, y: usize) -> usize {
    let mut fences = 0;
    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;
        if !(0..grid.len() as isize).contains(&new_x)
            || !(0..grid[0].len() as isize).contains(&new_y)
        {
            fences += 1;
            continue;
        }
        if grid[new_x as usize][new_y as usize] != grid[x][y] {
            fences += 1;
        }
    }
    fences
}

fn part1(grid: &[Vec<char>]) -> usize {
    let mut total = 0;
    let mut seen = HashSet::new();
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            let mut fences = 0;
            let mut area = 0;
            part1_helper(grid, x, y, &mut seen, &mut fences, &mut area);
            total += fences * area;
        }
    }
    total
}

fn part1_helper(
    grid: &[Vec<char>],
    x: usize,
    y: usize,
    seen: &mut HashSet<(usize, usize)>,
    fences: &mut usize,
    area: &mut usize,
) {
    if seen.contains(&(x, y)) {
        return;
    }
    seen.insert((x, y));
    *area += 1;
    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;
        if !(0..grid.len() as isize).contains(&new_x)
            || !(0..grid[0].len() as isize).contains(&new_y)
        {
            *fences += 1;
            continue;
        }
        if grid[new_x as usize][new_y as usize] != grid[x][y] {
            *fences += 1;
        } else {
            part1_helper(grid, new_x as usize, new_y as usize, seen, fences, area);
        }
    }
}
