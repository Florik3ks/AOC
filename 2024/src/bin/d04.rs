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
    // damn wtf
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

    let directions: [(i32, i32); 4] = [
        (1, 0),  // right
        (1, 1),  // right botttom
        (0, 1),  // bottom
        (-1, 1), // bottom left
    ];
    let search = "XMAS";
    let search2 = "SAMX";

    let mut result = 0;
    for y in 0..lines.len() {
        // println!("{}", lines[y]);
        for x in 0..lines[y].len() {
            if lines[y][x] != 'X' && lines[y][x] != 'S' {
                continue;
            }
            for (dx, dy) in directions.into_iter() {
                let search_ = if lines[y][x] == 'X' { search } else { search2 };
                if walk(x, y, &lines, &search_, dx, dy) {
                    result += 1;
                }
            }
        }
    }
    return result;
}

fn walk(
    mut cx: usize,
    mut cy: usize,
    lines: &Vec<Vec<char>>,
    search: &str,
    dx: i32,
    dy: i32,
) -> bool {

    for i in 1..4 {
        cx = match cx.checked_add_signed(dx as isize) {
            Some(u) => u,
            None => return false,
        };
        cy = match cy.checked_add_signed(dy as isize) {
            Some(u) => u,
            None => return false,
        };

        if cx >= lines[0].len() || cy >= lines.len() {
            return false;
        }
        let current: char = lines[cy][cx];
        if current != search.chars().nth(i).expect("must have values < 4") {
            return false;
        }
    }
    return true;
}
pub fn p2(input: &str) -> i32 {
    // damn wtf
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

    let directions: [(i32, i32); 2] = [
        (1, -1), // right top
        (1, 1),  // right botttom
    ];

    let mut result = 0;
    for y in 0..lines.len() {
        // println!("{}", lines[y]);
        for x in 0..lines[y].len() {
            if lines[y][x] != 'A' {
                continue;
            }
            if y == 0 || y >= lines.len() - 1 || x == 0 || x >= lines[y].len() - 1 {
                continue;
            }
            let mut cross = true;
            for (dx, dy) in directions.into_iter() {
                if !check_x(x, y, &lines, dx, dy) {
                    cross = false;
                }
            }
            if cross {
                result += 1;
            }
        }
    }
    return result;
}


fn check_x(mut cx: usize, mut cy: usize, lines: &Vec<Vec<char>>, dx: i32, dy: i32) -> bool {
    cx = match cx.checked_add_signed(dx as isize) {
        Some(u) => u,
        None => return false,
    };
    cy = match cy.checked_add_signed(dy as isize) {
        Some(u) => u,
        None => return false,
    };
    let mas: bool;
    match lines[cy][cx] {
        'M' => mas = true,
        'S' => mas = false,
        _ => return false,
    }
    cx = match cx.checked_add_signed((-2 * dx) as isize) {
        Some(u) => u,
        None => return false,
    };
    cy = match cy.checked_add_signed((-2 * dy) as isize) {
        Some(u) => u,
        None => return false,
    };
    return (!mas && lines[cy][cx] == 'M') || (mas && lines[cy][cx] == 'S');
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = 18;
        let example = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(p1(example), expected);
    }

    #[test]
    fn test_part2() {
        let expected = 9;
        let example = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(p2(example), expected);
    }
}
