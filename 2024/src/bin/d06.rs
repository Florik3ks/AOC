use std::process::exit;
use std::time::Instant;
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

    return lines
        .iter()
        .map(|f| f.iter().filter(|x| **x == 'X').count())
        .sum::<usize>() as i32;
}

pub fn p2(input: &str) -> i32 {
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

    let mut result = lines.clone();

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
    let dirs: [(isize, isize); 4] = [
        (0, -1), // up
        (1, 0),  // right
        (0, 1),  // down
        (-1, 0), // left
    ];
    let mut condition = true;
    while condition {
        let (cx, cy) = guardpos;
        let nx: usize = match cx.checked_add_signed(dirs[dir_index].0) {
            Some(v) => v,
            None => {
                condition = false;
                continue;
            }
        };
        let ny: usize = match cy.checked_add_signed(dirs[dir_index].1) {
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
            if result[cy][cx] != '0' {
                result[cy][cx] = '+';
            }
            lines[cy][cx] = '+';
            dir_index += 1;
            dir_index %= dirs.len();
            continue;
        }

        if result[ny][nx] != '0' {
            // search if the loop is possible
            if place_wall_creates_loop(&mut lines, (cx, cy), dirs, dir_index) {
                result[ny][nx] = '0';
            }
        }

        guardpos = (nx, ny);

        lines[ny][nx] = match dir_index {
            0 => '^',
            1 => '>',
            2 => 'v',
            3 => '<',
            _ => 'E', //rror
        };
        
        if result[ny][nx] != '0' {
            result[ny][nx] = match dir_index {
                0 => '|',
                1 => '-',
                2 => '|',
                3 => '-',
                _ => 'E', //rror
            };
        }
    }

    return result
        .iter()
        .map(|f| f.iter().filter(|x| **x == '0').count())
        .sum::<usize>() as i32;
}

fn pretty_print(lines: &Vec<Vec<char>>, pretty: Vec<(usize, usize)>) {
    use colored::Colorize; // debug purposes
    println!("{}", "-----------".purple());
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if pretty.contains(&(x, y)) {
                print!("{}", lines[y][x].to_string().red())
            } else {
                print!("{}", lines[y][x].to_string().bright_blue());
            }
        }
        println!()
    }
}

fn pretty_print2(lines: &Vec<Vec<char>>, pretty: Vec<(usize, usize)>) {
    use colored::Colorize; // debug purposes
    println!("{}", "-----------".purple());
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if pretty.contains(&(x, y)) {
                print!("{}", lines[y][x].to_string().red())
            } else {
                print!("{}", lines[y][x].to_string().white());
            }
        }
        println!()
    }
}

fn place_wall_creates_loop(
    lines: &mut Vec<Vec<char>>,
    (cx, cy): (usize, usize),
    dirs: [(isize, isize); 4],
    dir_index: usize,
) -> bool {
    // println!("try place wall");
    let mut new_dir_index = (dir_index + 1) % dirs.len();
    let mut nx = cx;
    let mut ny = cy;
    let mut visited: Vec<(usize, usize, usize)> = vec![];
    loop {
        let new_dir = dirs[new_dir_index];
        let search_for = match (dir_index + 1) % dirs.len() {
            0 => '^',
            1 => '>',
            2 => 'v',
            3 => '<',
            _ => 'E', //rror
        };

        if visited.contains(&(nx, ny, new_dir_index)) {
            // println!("visited before2");
            // pretty_print2(lines, visited.iter().map(|f| (f.0, f.1)).collect());
            return true;
        }

        visited.push((nx, ny, new_dir_index));
        let nx2 = match nx.checked_add_signed(new_dir.0) {
            Some(v) => v,
            None => {
                return false;
            }
        };
        let ny2 = match ny.checked_add_signed(new_dir.1) {
            Some(v) => v,
            None => {
                return false;
            }
        };
        if ny2 >= lines.len() || nx2 >= lines[ny].len() {
            return false;
        }
        if lines[ny2][nx2] == '#' {
            new_dir_index += 1;
            new_dir_index %= dirs.len();
            continue;
        }
        nx = nx2;
        ny = ny2;

        if lines[ny][nx] == search_for {
            return true;
        }
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

// 705 too low
// 3574 too high
