use std::{fs::File, io::read_to_string};

const DEBUG: bool = false;

fn main() {
    //let str = read_to_string(File::open("small_example.txt").unwrap()).unwrap();
    //let str = read_to_string(File::open("larger_example.txt").unwrap()).unwrap();
    //let str = read_to_string(File::open("p2_example.txt").unwrap()).unwrap();
    let str = read_to_string(File::open("input.txt").unwrap()).unwrap();
    let (initial_config, movements) = str.split_once("\n\n").unwrap();
    let mut grid = Vec::new();

    for line in initial_config.lines() {
        grid.push(line.chars().collect::<Vec<_>>());
    }
    let movements = movements
        .chars()
        .filter(|&c| !c.is_whitespace())
        .collect::<String>();

    println!("{}", part2(grid, &movements));
}

fn part2(mut grid: Vec<Vec<char>>, movements: &str) -> usize {
    grid.iter_mut().for_each(|vec| {
        let mut new_vec = vec![];
        for &c in vec.iter() {
            match c {
                '#' => new_vec.extend_from_slice(&['#'; 2]),
                'O' => new_vec.extend_from_slice(&['[', ']']),
                '.' => new_vec.extend_from_slice(&['.'; 2]),
                '@' => new_vec.extend_from_slice(&['@', '.']),
                _ => unreachable!(),
            }
        }
        *vec = new_vec;
    });
    let mut robot_x = usize::MAX;
    let mut robot_y = usize::MAX;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '@' {
                robot_x = x;
                robot_y = y;
            }
        }
    }

    for movement in movements.chars() {
        let offset = offset(movement);
        let new_x = robot_x.checked_add_signed(offset.0).unwrap();
        let new_y = robot_y.checked_add_signed(offset.1).unwrap();

        if grid[new_y][new_x] == '.' {
            grid[new_y][new_x] = '@';
            grid[robot_y][robot_x] = '.';
            robot_x = new_x;
            robot_y = new_y;
        } else if matches!(grid[new_y][new_x], '[' | ']')
            && is_valid_move(&grid, new_x, new_y, offset)
        {
            move_boxes(&mut grid, new_x, new_y, offset);
            assert_eq!('.', grid[new_y][new_x]);
            grid[new_y][new_x] = '@';
            grid[robot_y][robot_x] = '.';
            robot_x = new_x;
            robot_y = new_y;
        }
        if DEBUG {
            println!("\nMove {movement}:");
            draw(&grid);
        }
    }

    let mut score = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] != '[' {
                continue;
            }
            score += 100 * y + x;
        }
    }
    score
}

fn is_valid_move(grid: &[Vec<char>], x: usize, y: usize, offset: (isize, isize)) -> bool {
    if grid[y][x] == '#' {
        return false;
    }
    let (new_x, new_y) = (
        x.checked_add_signed(offset.0).unwrap(),
        y.checked_add_signed(offset.1).unwrap(),
    );
    if grid[y][x] == '.' {
        true
    } else if offset.0 != 0 {
        is_valid_move(grid, new_x, new_y, offset)
    } else {
        assert_ne!(offset.1, 0);
        assert_eq!(offset.0, 0);
        let (new_partner_x, new_partner_y) = match grid[y][x] {
            '[' => ((x + 1), y.checked_add_signed(offset.1).unwrap()),
            ']' => ((x - 1), y.checked_add_signed(offset.1).unwrap()),
            _ => unreachable!(),
        };
        is_valid_move(grid, new_x, new_y, offset)
            && is_valid_move(grid, new_partner_x, new_partner_y, offset)
    }
}

fn move_boxes(grid: &mut [Vec<char>], x: usize, y: usize, offset: (isize, isize)) {
    if grid[y][x] == '#' {
        panic!()
    }
    let (new_x, new_y) = (
        x.checked_add_signed(offset.0).unwrap(),
        y.checked_add_signed(offset.1).unwrap(),
    );
    if grid[y][x] == '.' {
        return;
    } else if offset.0 != 0 {
        move_boxes(grid, new_x, new_y, offset);
        let temp = grid[y][x];
        grid[y][x] = grid[new_y][new_x];
        grid[new_y][new_x] = temp;
    } else {
        assert_ne!(offset.1, 0);
        assert_eq!(offset.0, 0);
        let (partner_x, partner_y) = match grid[y][x] {
            '[' => (x + 1, y),
            ']' => (x - 1, y),
            _ => unreachable!(),
        };
        let (new_partner_x, new_partner_y) = match grid[y][x] {
            '[' => ((x + 1), y.checked_add_signed(offset.1).unwrap()),
            ']' => ((x - 1), y.checked_add_signed(offset.1).unwrap()),
            _ => unreachable!(),
        };

        move_boxes(grid, new_x, new_y, offset);
        move_boxes(grid, new_partner_x, new_partner_y, offset);

        let temp = grid[y][x];
        grid[y][x] = grid[new_y][new_x];
        grid[new_y][new_x] = temp;

        let temp = grid[partner_y][partner_x];
        grid[partner_y][partner_x] = grid[new_partner_y][new_partner_x];
        grid[new_partner_y][new_partner_x] = temp;
    }
}

fn part1(mut grid: Vec<Vec<char>>, movements: &str) -> usize {
    let mut robot_x = usize::MAX;
    let mut robot_y = usize::MAX;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '@' {
                robot_x = x;
                robot_y = y;
            }
        }
    }

    for movement in movements.chars() {
        let offset = offset(movement);
        let new_x = robot_x.checked_add_signed(offset.0).unwrap();
        let new_y = robot_y.checked_add_signed(offset.1).unwrap();

        if grid[new_y][new_x] == '.' {
            grid[new_y][new_x] = '@';
            grid[robot_y][robot_x] = '.';
            robot_x = new_x;
            robot_y = new_y;
        } else if grid[new_y][new_x] == 'O' && push_boxes(&mut grid, new_x, new_y, offset) {
            assert_eq!('.', grid[new_y][new_x]);
            grid[new_y][new_x] = '@';
            grid[robot_y][robot_x] = '.';
            robot_x = new_x;
            robot_y = new_y;
        }
        if DEBUG {
            println!("\nMove {movement}:");
            draw(&grid);
        }
    }

    let mut score = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] != 'O' {
                continue;
            }
            score += 100 * y + x;
        }
    }
    score
}

fn push_boxes(grid: &mut [Vec<char>], x: usize, y: usize, offset: (isize, isize)) -> bool {
    if grid[y][x] == '#' {
        return false;
    }
    let (new_x, new_y) = (
        x.checked_add_signed(offset.0).unwrap(),
        y.checked_add_signed(offset.1).unwrap(),
    );
    if grid[new_y][new_x] == '.' || push_boxes(grid, new_x, new_y, offset) {
        let temp = grid[y][x];
        grid[y][x] = grid[new_y][new_x];
        grid[new_y][new_x] = temp;
        return true;
    }

    false
}

fn offset(movement: char) -> (isize, isize) {
    match movement {
        '^' => (0, -1),
        '>' => (1, 0),
        '<' => (-1, 0),
        'v' => (0, 1),
        _ => unreachable!("char found: {movement}"),
    }
}

fn draw(grid: &[Vec<char>]) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            print!("{}", grid[y][x]);
        }
        println!();
    }
}
