use std::time::Instant;
use std::{fs::File, io::Read};

fn main() {

    let day = file!().split('/').last().unwrap()[1..3].to_owned();

    let path = format!("src/input/input{day}.txt");
    println!("{}", path);
    let mut file = File::open(path).expect("File not found");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("cannot read file");

    let mut now = Instant::now();
    let p1result = p1(&input);
    let mut elapsed = now.elapsed();

    println!("Part 1 solution in: {:.2?}", elapsed);
    println!("{}", p1result);

    now = Instant::now();
    let p2result = p2(&input);
    elapsed = now.elapsed();
    println!("Part 2 solution in: {:.2?}", elapsed);
    println!("{}", p2result);
}

pub fn p1(input: &str) -> i32 {
    use regex::Regex;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut result = 0;
    for line in input.lines() {
        for (_, [mulx, muly]) in re.captures_iter(line).map(|c| c.extract()) {
            result += mulx.parse::<i32>().unwrap() * muly.parse::<i32>().unwrap();
        }
    }
    return result;
}

pub fn p2(input: &str) -> i32 {
    use regex::Regex;

    let mut result = 0;

    let re = Regex::new(r"don't\(\)|do\(\)|mul\(\d{1,3},\d{1,3}\)").unwrap();
    let mut do_ = true;
    for line in input.lines() {
        for (full, []) in re.captures_iter(line).map(|c| c.extract()) {
            if full == "don't()" {
                do_ = false;
            } else if full == "do()" {
                do_ = true;
            } else if do_ {
                let inner_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
                let (_, [mulx, muly]) = inner_re.captures(full).expect("should capture").extract();
                result += mulx.parse::<i32>().unwrap() * muly.parse::<i32>().unwrap();
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
        let expected = 161;
        let example = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(p1(example), expected);
    }

    #[test]
    fn test_part2() {
        let expected = 48;
        let example = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(p2(example), expected);
    }
}
