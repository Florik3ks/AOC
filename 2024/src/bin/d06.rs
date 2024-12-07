use std::time::Instant;
use std::vec;
use std::{fs::File, io::Read};

fn main() {
    // get day number from file
    let day = file!().split('/').last().unwrap()[1..3].to_owned();

    // read input
    let path = format!("src/input/input{day}.txt");
    println!("{}", path);
    let mut file = File::open(path).expect("File not found");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("cannot read file");

    // measure time needed to run both parts
    let mut now: Instant;
    let mut elapsed: std::time::Duration;

    now = Instant::now();
    let p1result = p1(&input);
    elapsed = now.elapsed();

    println!("Part 1 solution in: {:.2?}", elapsed);
    println!("{}", p1result);

    now = Instant::now();
    let p2result = p2(&input);
    elapsed = now.elapsed();

    println!("Part 2 solution in: {:.2?}", elapsed);
    println!("{}", p2result);
}

pub fn p1(input: &str) -> i32 {
    let lines = get_path(input);

    return lines.len() as i32;
}

fn get_path(input: &str) -> Vec<(usize, usize)> {
    let mut lines: Vec<Vec<char>> = input
        .lines()
        .map(|f| {
            f.to_string()
                .as_bytes()
                .to_owned()
                .iter()
                .map(|f| *f as char)
                .collect()
        })
        .collect();

    // get guard pos
    let mut guardpos: (usize, usize) = (0, 0);
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if lines[y][x] == '^' {
                guardpos = (x, y);
                lines[y][x] = 'X'
            }
        }
    }

    let mut dir_index = 0;
    let dirs = [
        (0, -1), // up
        (1, 0),  // right
        (0, 1),  // down
        (-1, 0), // left
    ];
    let mut condition = true;
    while condition {
        let nx: usize = match guardpos.0.checked_add_signed(dirs[dir_index].0) {
            Some(v) => v,
            None => {
                condition = false;
                continue;
            }
        };
        let ny: usize = match guardpos.1.checked_add_signed(dirs[dir_index].1) {
            Some(v) => v,
            None => {
                condition = false;
                continue;
            }
        };
        if ny >= lines.len() || nx >= lines[ny].len() {
            condition = false;
            continue;
        }
        if lines[ny][nx] == '#' {
            dir_index += 1;
            dir_index %= dirs.len();
            continue;
        }
        guardpos = (nx, ny);
        lines[ny][nx] = 'X';
    }

    let mut path: Vec<(usize, usize)> = vec![];
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if lines[y][x] == 'X' {
                path.push((x, y));
            }
        }
    }
    return path;
}

pub fn p2(input: &str) -> i32 {
    let path = get_path(input);

    let mut lines: Vec<Vec<char>> = input
        .lines()
        .map(|f| {
            f.to_string()
                .as_bytes()
                .to_owned()
                .iter()
                .map(|f| *f as char)
                .collect()
        })
        .collect();

    let dirs: [(isize, isize); 4] = [
        (0, -1), // up
        (1, 0),  // right
        (0, 1),  // down
        (-1, 0), // left
    ];

    let mut guardpos: (usize, usize) = (0, 0);
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if lines[y][x] == '^' {
                guardpos = (x, y);
                lines[y][x] = 'X'
            }
        }
    }

    let mut r = 0;
    let mut why: Vec<(usize, usize)> = vec![];
    for (x, y) in path {
        if x == guardpos.0 && y == guardpos.1 {
            continue;
        }
        lines[y][x] = '#';
        if place_wall_creates_loop(&mut lines, (guardpos.0, guardpos.1), dirs, 0) {
            r += 1;
            why.push((x, y));
        }
        lines[y][x] = '.';
    }

    return r;
}

fn place_wall_creates_loop(
    lines: &mut Vec<Vec<char>>,
    (cx, cy): (usize, usize),
    dirs: [(isize, isize); 4],
    dir_index: usize,
) -> bool {
    let mut new_dir_index = dir_index;
    let mut nx = cx;
    let mut ny = cy;
    let mut visited: Vec<(usize, usize, usize)> = vec![];
    loop {
        let nx2 = match nx.checked_add_signed(dirs[new_dir_index].0) {
            Some(v) => v,
            None => {
                return false;
            }
        };
        let ny2 = match ny.checked_add_signed(dirs[new_dir_index].1) {
            Some(v) => v,
            None => {
                return false;
            }
        };
        if ny2 >= lines.len() || nx2 >= lines[ny].len() {
            return false;
        }

        if lines[ny2][nx2] == '#' || lines[ny2][nx2] == '0' {
            if visited.contains(&(nx, ny, new_dir_index)) {
                return true;
            }
            visited.push((nx, ny, new_dir_index));
            new_dir_index += 1;
            new_dir_index %= dirs.len();
            continue;
        }
        nx = nx2;
        ny = ny2;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = 41;
        let example = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(p1(example), expected);
    }

    #[test]
    fn test_part2() {
        let expected = 6;
        let example = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(p2(example), expected);
    }

    #[test]
    fn mt1() {
        let example = r"....#.....
.........#
....#.....
..#.....#.
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(p2(example), 2);
    }
}
