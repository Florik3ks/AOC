use std::collections::{HashMap, HashSet};
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

pub fn solve(input: &str, p2: bool) -> i32 {
    let lines: Vec<Vec<char>> = input
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
    let mut signals: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            let c = lines[y][x];
            if c != '.' {
                // how do you do this correctly in rust, this does not feel right at all
                let signal: &Vec<(usize, usize)> = match signals.get(&c) {
                    Some(e) => e,
                    _ => &Vec::new(),
                };
                let mut signal: Vec<(usize, usize)> = signal.clone();
                signal.push((x, y));
                signals.insert(c, signal);
                // idk
            }
        }
    }

    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    let max_x = (lines[0].len() - 1) as isize;
    let max_y = (lines.len() - 1) as isize;
    for key in signals.keys() {
        let signals_c: &Vec<(usize, usize)> = signals.get(key).expect("should have key");
        let combinations: Vec<((usize, usize), (usize, usize))> = signals_c
            .iter()
            .map(|&c| signals_c.iter().map(move |&d| (c, d)))
            .flatten()
            .filter(|a| a.0 .0 != a.1 .0 && a.0 .1 != a.1 .1)
            .collect();
        for (sig1, sig2) in combinations {
            for a in get_antinodes_from_signals(sig1, sig2, max_x, max_y, p2) {
                antinodes.insert(a);
            }
        }
    }

    return antinodes.len() as i32;
}

pub fn p1(input: &str) -> i32 {
    return solve(input, false);
}

pub fn p2(input: &str) -> i32 {
    return solve(input, true);
}
fn get_antinodes_from_signals(
    sig1: (usize, usize),
    sig2: (usize, usize),
    max_x: isize,
    max_y: isize,
    all: bool,
) -> Vec<(usize, usize)> {
    let sig1 = (sig1.0 as isize, sig1.1 as isize);
    let sig2 = (sig2.0 as isize, sig2.1 as isize);

    let mut result: Vec<(usize, usize)> = Vec::new();

    let mut fst = sig1;
    let temp = (sig1.0 - sig2.0, sig1.1 - sig2.1);
    loop {
        fst = (fst.0 + temp.0, fst.1 + temp.1);
        if fst.0 >= 0 && fst.1 >= 0 && fst.0 <= max_x && fst.1 <= max_y {
            result.push((fst.0 as usize, fst.1 as usize));
        } else if all {
            result.push((sig1.0 as usize, sig1.1 as usize));
            break;
        }
        if !all {
            break;
        }
    }

    let mut snd = sig2;
    let temp = (sig2.0 - sig1.0, sig2.1 - sig1.1);
    loop {
        snd = (snd.0 + temp.0, snd.1 + temp.1);

        if snd.0 >= 0 && snd.1 >= 0 && snd.0 <= max_x && snd.1 <= max_y {
            result.push((snd.0 as usize, snd.1 as usize));
        } else if all{
            result.push((sig2.0 as usize, sig2.1 as usize));
            break;
        }
        if !all {
            break;
        }
    }
    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = 14;
        let example = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(p1(example), expected);
    }

    #[test]
    fn test_part1_2() {
        let expected = 4;
        let example = r"............
............
............
.......0....
....0.......
............
............
............
........A...
.........A..
............
............";
        assert_eq!(p1(example), expected);
    }

    #[test]
    fn test_part2() {
        let expected = 9;
        let example = r"T....#....
...T......
.T........
..........
..........
..........
..........
..........
..........
..........";
        assert_eq!(p2(example), expected);
    }

    #[test]
    fn test_part2_2() {
        let expected = 34;
        let example = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(p2(example), expected);
    }
}
