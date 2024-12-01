use std::{fs::File, io::Read};

fn main() {
    let day = file!().split('/').last().unwrap()[1..3].to_owned();

    let path = format!("src/input/input{day}.txt");
    println!("{}", path);
    let mut file = File::open(path).expect("File not found");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("cannot read file");

    println!("{}", p1(&input));
    println!("{}", p2(&input));
}

pub fn p1(input: &str) -> i32 {
    let mut result: i32 = 0;

    let mut numbers_left: Vec<i32> = Vec::new();
    let mut numbers_right: Vec<i32> = Vec::new();

    input.lines().for_each(|line| {
        let mut parts = line.split_whitespace();
        numbers_left.push(parts.next().unwrap().parse().unwrap());
        numbers_right.push(parts.next_back().unwrap().parse().unwrap());
    });

    let num = numbers_left.len();
    for n in 0..num {
        let mut min_l = 0;
        let mut min_r = 0;
        for i in 0..numbers_left.len(){
            if numbers_left[i] < numbers_left[min_l] {
                min_l = i;
            }
            if numbers_right[i] < numbers_right[min_r] {
                min_r = i;
            }
        }
        result += (numbers_left.remove(min_l) - numbers_right.remove(min_r)).abs();
    }

    return result;
}

pub fn p2(input: &str) -> i32 {
    let mut result: i32 = 0;

    let mut numbers_left: Vec<i32> = Vec::new();
    let mut numbers_right: Vec<i32> = Vec::new();

    input.lines().for_each(|line| {
        let mut parts = line.split_whitespace();
        numbers_left.push(parts.next().unwrap().parse().unwrap());
        numbers_right.push(parts.next_back().unwrap().parse().unwrap());
    });

    let num = numbers_left.len();
    for n in 0..num {
        let current = numbers_left[n];

        let mut count = 0;
        for i in 0..numbers_right.len(){
            if numbers_right[i] == current{
                count += 1;
            }
        }
        result += current * count;
    }

    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = 11;
        let example = r"3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(p1(example), expected);
    }

    #[test]
    fn test_part2() {
        let expected = 31;
        let example = r"3   4
4   3
2   5
1   3
3   9
3   3        ";
        assert_eq!(p2(example), expected);
    }
}
