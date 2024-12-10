use std::collections::HashSet;
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

#[derive(Clone)]
struct Node {
    pos: (usize, usize),
    // neighbours: Vec<&Node>, // how???
    value: usize,
}

pub fn p1(input: &str) -> i32 {
    let (nodes, trailheads) = parse_graph(input);
    let mut result = 0;
    for head in trailheads{
        result += traverse_neighbours(&nodes, head.0, head.1).len();
    }

    return result as i32;
}


pub fn p2(input: &str) -> i32 {
    let (nodes, trailheads) = parse_graph(input);
    let mut result = 0;
    for head in trailheads{
        result += traverse_neighbours2(&nodes, head.0, head.1).len();
    }

    return result as i32;
}

fn traverse_neighbours(nodes: &Vec<Vec<Option<Node>>>, cx: usize, cy: usize) -> HashSet<(usize, usize)>{
    let current_value = nodes[cy][cx].as_ref().unwrap().value;
    if current_value == 9{
        let mut r: HashSet::<(usize, usize)> = HashSet::new();
        r.insert((cx, cy));
        return r;
    }

    let cx = cx as isize;
    let cy = cy as isize;
    let left: (isize, isize) = (cx - 1, cy);
    let up: (isize, isize) = (cx, cy - 1);
    let right: (isize, isize) = (cx + 1, cy);
    let down: (isize, isize) = (cx, cy + 1);
    let positions = [left, up, right, down];

    let mut result: HashSet<(usize, usize)> = HashSet::new();
    for p in positions{
        let tmp = check_if_valid(nodes, p.0, p.1);
        if tmp.is_some() && tmp.unwrap().value == current_value + 1{
            for v in traverse_neighbours(nodes, p.0 as usize, p.1 as usize){
                result.insert(v);
            }
        }
    }

    return result;

}

fn parse_graph(input: &str) -> (Vec<Vec<Option<Node>>>, Vec<(usize, usize)>){
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

    let mut nodes: Vec<Vec<Option<Node>>> = Vec::new();
    let mut trailheads: Vec<(usize, usize)> = Vec::new();

    for y in 0..lines.len() {
        nodes.push(Vec::new());
        for x in 0..lines[y].len() {
            if !lines[y][x].is_numeric() {
                nodes[y].push(None);
                continue;
            }
            let val = lines[y][x].to_digit(10).unwrap() as usize;
            let node = Node {
                pos: (x, y),
                value: val,
            };
            if val == 0 {
                trailheads.push(node.pos);
            }
            nodes[y].push(Some(node));
        }
    }

    return (nodes, trailheads);
}

fn check_if_valid(nodes: &Vec<Vec<Option<Node>>>, x: isize, y: isize) -> Option<&Node>{
    if !(x >= 0 && y >= 0 && y < nodes.len() as isize && x < nodes.len() as isize){
        return None;
    }
    return nodes[y as usize][x as usize].as_ref();
}


fn traverse_neighbours2(nodes: &Vec<Vec<Option<Node>>>, cx: usize, cy: usize) -> Vec<(usize, usize)>{
    let current_value = nodes[cy][cx].as_ref().unwrap().value;
    if current_value == 9{
        return vec![(cx, cy)];
    }

    let cx = cx as isize;
    let cy = cy as isize;
    let left: (isize, isize) = (cx - 1, cy);
    let up: (isize, isize) = (cx, cy - 1);
    let right: (isize, isize) = (cx + 1, cy);
    let down: (isize, isize) = (cx, cy + 1);
    let positions = [left, up, right, down];

    let mut result: Vec<(usize, usize)> = Vec::new();
    for p in positions{
        let tmp = check_if_valid(nodes, p.0, p.1);
        if tmp.is_some() && tmp.unwrap().value == current_value + 1{
            for v in traverse_neighbours2(nodes, p.0 as usize, p.1 as usize){
                result.push(v);
            }
        }
    }

    return result;

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = 1;
        let example = r"0123
1234
8765
9876";
        assert_eq!(p1(example), expected);
    }

    #[test]
    fn test_part1_2() {
        let example = r"...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9";
        assert_eq!(p1(example), 2);
    }

    #[test]
    fn test_part1_3() {
        let example = r"10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01";
        assert_eq!(p1(example), 3);
    }

    #[test]
    fn test_part1_4() {
        let example = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(p1(example), 36);
    }

    #[test]
    fn test_part2() {
        let example = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(p2(example), 81);
    }

    #[test]
    fn test_part2_1() {
        let example = r".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....";
        assert_eq!(p2(example), 3);
    }
}
