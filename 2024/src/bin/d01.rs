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

    let mut numbersLeft: Vec<i32> = Vec::new();
    let mut numbersRight: Vec<i32> = Vec::new();

    input.lines().for_each(|line| {
        let mut parts = line.split_whitespace();
        numbersLeft.push(parts.next().unwrap().parse().unwrap());
        numbersRight.push(parts.next_back().unwrap().parse().unwrap());
    });

    let num = numbersLeft.len() - 1;
    for n in 0..=num {
        let mut minL = 0;
        let mut minR = 0;
        for i in 0..=numbersLeft.len() - 1{
            if numbersLeft[i] < numbersLeft[minL] {
                minL = i;
            }
            if numbersRight[i] < numbersRight[minR] {
                minR = i;
            }
        }
        result += (numbersLeft.remove(minL) - numbersRight.remove(minR)).abs();
    }

    return result;
}

pub fn p2(input: &str) -> i32 {
    let mut result: i32 = 0;

    let mut numbersLeft: Vec<i32> = Vec::new();
    let mut numbersRight: Vec<i32> = Vec::new();

    input.lines().for_each(|line| {
        let mut parts = line.split_whitespace();
        numbersLeft.push(parts.next().unwrap().parse().unwrap());
        numbersRight.push(parts.next_back().unwrap().parse().unwrap());
    });

    let num = numbersLeft.len() - 1;
    for n in 0..=num {
        let current = numbersLeft[n];

        let mut count = 0;
        for i in 0..=numbersRight.len() - 1{
            if numbersRight[i] == current{
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
