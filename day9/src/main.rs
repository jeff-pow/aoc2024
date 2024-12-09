use std::{fs::read_to_string, ops::Range, time::Instant};

fn main() {
    let input = read_to_string("example.txt").unwrap();
    let input = read_to_string("input.txt").unwrap();
    let str = parse_input(input);

    let t = Instant::now();
    println!("{}", part2(str));
    dbg!(t.elapsed());
}

fn find_free_chunk(fs: &[Option<u64>], required_size: usize) -> Option<usize> {
    fs.windows(required_size)
        .position(|w| w.iter().all(|x| x.is_none()))
}

fn find_required_chunk(fs: &[Option<u64>], num_to_find: u64) -> Range<usize> {
    let start = fs.iter().position(|&s| s == Some(num_to_find)).unwrap();
    start
        ..start
            + fs[start..]
                .iter()
                .take_while(|&&s| s == Some(num_to_find))
                .count()
}

fn part2(mut fs: Vec<Option<u64>>) -> u64 {
    let mut num_to_find = fs.iter().filter_map(|&x| x).max().unwrap();
    let mut chunk = find_required_chunk(&fs, num_to_find);
    loop {
        if let Some(free) = find_free_chunk(&fs[..chunk.start], chunk.len()) {
            for offset in 0..chunk.len() {
                fs.swap(free + offset, chunk.start + offset);
            }
        }
        if num_to_find == 0 {
            break;
        }
        num_to_find -= 1;
        chunk = find_required_chunk(&fs, num_to_find);
    }

    fs.into_iter()
        .enumerate()
        .filter_map(|(idx, c)| c.map(|x| x * idx as u64))
        .sum::<u64>()
}

fn find_free(fs: &[Option<u64>], start: usize) -> usize {
    fs.iter()
        .enumerate()
        .skip(start)
        .find(|c| c.1.is_none())
        .map(|c| c.0)
        .unwrap()
}

fn find_allocated(fs: &[Option<u64>]) -> usize {
    fs.len() - 1 - fs.iter().rev().position(|c| c.is_some()).unwrap()
}

#[allow(dead_code)]
fn part1(mut fs: Vec<Option<u64>>) -> u64 {
    let mut free = find_free(&fs, 0);
    let mut alloc = find_allocated(&fs);
    while free < alloc {
        fs.swap(free, alloc);
        free = find_free(&fs, free + 1);
        alloc = find_allocated(&fs);
    }

    fs.into_iter()
        .enumerate()
        .filter_map(|(idx, c)| c.map(|x| x * idx as u64))
        .sum::<u64>()
}

fn parse_input(input: String) -> Vec<Option<u64>> {
    let mut vec = vec![];
    let mut file = true;
    let mut file_num = 0;

    for num in input.trim().chars() {
        if file {
            vec.extend(std::iter::repeat(Some(file_num)).take(num.to_digit(10).unwrap() as usize));
            file_num += 1;
        } else {
            vec.extend(std::iter::repeat(None).take(num.to_digit(10).unwrap() as usize));
        }
        file = !file;
    }

    vec
}
