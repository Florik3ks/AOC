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

enum Operator {
    Add,
    Mul,
    Concat,
}

impl Operator {
    pub fn operate(&self, a: u64, b: u64) -> Option<u64> {
        match &self {
            Self::Add => a.checked_add(b),
            Self::Mul => a.checked_mul(b),
            Self::Concat => match format!("{}{}", a, b).parse::<u64>() {
                Ok(a) => Some(a),
                Err(_) => None,
            },
        }
    }
}

pub fn p1(input: &str) -> u64 {
    let mut result = 0;
    for line in input.lines() {
        let mut split = line.split(':');
        let test: u64 = split.next().unwrap().parse().unwrap();
        let mut nums: Vec<u64> = split
            .next()
            .unwrap()
            .split(' ')
            .map(|s| s.trim())
            .filter(|n| !n.is_empty())
            .map(|n| n.parse().unwrap())
            .collect();
        let first = nums.remove(0);
        if iterate(&mut nums, first, test, false) {
            result += test;
        }
    }

    return result;
}

fn iterate(nums: &mut Vec<u64>, current: u64, result: u64, use_concat: bool) -> bool {
    if nums.len() == 0 {
        return current == result;
    }
    let next_num = nums.remove(0);

    let operators = if use_concat {
        vec![Operator::Add, Operator::Mul, Operator::Concat]
    } else {
        vec![Operator::Add, Operator::Mul]
    };
    
    for o in operators {
        let nc = match o.operate(current, next_num) {
            Some(v) => v,
            None => return false,
        };
        if iterate(nums, nc, result, use_concat) {
            return true;
        }
    }
    nums.insert(0, next_num);
    return false;
}

pub fn p2(input: &str) -> u64 {
    let mut result = 0;
    for line in input.lines() {
        let mut split = line.split(':');
        let test: u64 = split.next().unwrap().parse().unwrap();
        let mut nums: Vec<u64> = split
            .next()
            .unwrap()
            .split(' ')
            .map(|s| s.trim())
            .filter(|n| !n.is_empty())
            .map(|n| n.parse().unwrap())
            .collect();
        let first = nums.remove(0);
        if iterate(&mut nums, first, test, true) {
            result += test;
        }
    }

    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = 3749;
        let example = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(p1(example), expected);
    }

    #[test]
    fn test_part2() {
        let expected = 11387;
        let example = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(p2(example), expected);
    }
}
