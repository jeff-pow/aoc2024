use itertools::Itertools;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

#[derive(Clone, Copy)]
struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

fn main() {
    let mut vec = Vec::new();
    //for line in BufReader::new(File::open("example.txt").unwrap()).lines() {
    for line in BufReader::new(File::open("input.txt").unwrap()).lines() {
        let line = line.unwrap();
        let (p, v) = line.split_once(' ').unwrap();
        let p = p.strip_prefix("p=").unwrap();
        let (p_x, p_y) = p.split_once(',').unwrap();
        let v = v.strip_prefix("v=").unwrap();
        let (v_x, v_y) = v.split_once(',').unwrap();
        vec.push(Robot {
            pos: (p_x.parse().unwrap(), p_y.parse().unwrap()),
            vel: (v_x.parse().unwrap(), v_y.parse().unwrap()),
        });
    }

    let t = Instant::now();
    println!("{}", part2(vec));
    dbg!(t.elapsed());
}

fn num_neighbors(vec: &[Robot], seen: &mut HashSet<(i32, i32)>, x: i32, y: i32) -> usize {
    if seen.contains(&(x, y)) {
        return 0;
    }

    if !vec.iter().any(|r| r.pos == (x, y)) {
        return 0;
    }

    seen.insert((x, y));
    let mut count = 1;

    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            count += num_neighbors(vec, seen, x + dx, y + dy);
        }
    }

    count
}

fn part2(mut vec: Vec<Robot>) -> i32 {
    for i in 0.. {
        for robot in &mut vec {
            robot.pos.0 = (robot.pos.0 + robot.vel.0).rem_euclid(BOX_X);
            robot.pos.1 = (robot.pos.1 + robot.vel.1).rem_euclid(BOX_Y);
        }
        let mut seen = HashSet::new();
        for x in 0..BOX_X {
            for y in 0..BOX_Y {
                if num_neighbors(&vec, &mut seen, x, y) > 64 {
                    draw(&vec, true);
                    return i + 1;
                }
            }
        }
        println!("{i}");
    }
    unreachable!()
}

const BOX_X: i32 = 101;
const BOX_Y: i32 = 103;

fn part1(mut vec: Vec<Robot>) -> i32 {
    let mut quadrants = [0; 4];
    for robot in &mut vec {
        robot.pos.0 = (robot.pos.0 + robot.vel.0 * 100).rem_euclid(BOX_X);
        robot.pos.1 = (robot.pos.1 + robot.vel.1 * 100).rem_euclid(BOX_Y);
    }
    draw(&vec, false);

    for robot in vec {
        if robot.pos.0 < BOX_X / 2 {
            if robot.pos.1 < BOX_Y / 2 {
                quadrants[1] += 1;
            } else if robot.pos.1 > BOX_Y / 2 {
                quadrants[2] += 1;
            }
        } else if robot.pos.0 > BOX_X / 2 {
            if robot.pos.1 < BOX_Y / 2 {
                quadrants[0] += 1;
            } else if robot.pos.1 > BOX_Y / 2 {
                quadrants[3] += 1;
            }
        }
    }
    dbg!(quadrants);

    quadrants.into_iter().product::<i32>()
}

fn draw(vec: &[Robot], draw_quadrants: bool) {
    for y in 0..BOX_Y {
        for x in 0..BOX_X {
            if !draw_quadrants && (x == BOX_X / 2 || y == BOX_Y / 2) {
                print!(" ");
                continue;
            }
            let num = vec.iter().filter(|r| r.pos.0 == x && r.pos.1 == y).count();
            if num > 0 {
                print!("{num}");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
