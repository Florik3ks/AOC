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
    let mut result = 0;
    for line in input.lines() {
        let split: Vec<i32> = line
            .split_whitespace()
            .flat_map(|f| f.parse::<i32>())
            .collect();
        let mut valid = true;

        let first = split[0];
        let second = split[1];
        let up = first < second;
        if !check(first, second, up) {
            continue;
        }

        for i in 0..split.len() - 1 {
            if !check(split[i], split[i + 1], up) {
                valid = false;
            }
        }

        if valid {
            result += 1
        }
    }
    return result;
}

pub fn p2(input: &str) -> i32 {
    let mut result = 0;
    for line in input.lines() {
        let mut split: Vec<i32> = line
            .split_whitespace()
            .flat_map(|f| f.parse::<i32>())
            .collect();

        //
        // stuff i tried first for way too long, does not work because whatever, idk
        //

        // let mut valid_strike = false;

        // let mut up = split[0] < split[1];

        // let mut valid;
        // // check if first pair works
        // // if not, extra logic is needed to determine if numbers should go up or down
        // if !check(split[0], split[1], up) {
        //     valid_strike = true;
        //     // try remove [1]
        //     up = split[0] < split[2];
        //     let temp = split[1];
        //     split[1] = split[0];
        //     valid = rest(split.clone(), 1, up, valid_strike);
        //     // if not valid, remove [0] instead
        //     if !valid {
        //         split[1] = temp;
        //         up = split[1] < split[2];
        //     }
        // }

        // valid = rest(split.clone(), 1, up, valid_strike);
        // if !valid && !valid_strike {
        //     valid_strike = true;
        //     up = split[1] < split[2];
        //     valid = rest(split, 1, up, valid_strike);
        // }

        // if valid {
        //     result += 1
        // }
        // else{
        //     println!("{}", line);
        //     println!("up: {} strike: {}", up, valid_strike);
        // }

        let mut valid = checkline(split.clone());
        for i in 0..split.len() {
            let mut cpy = split.clone();
            cpy.remove(i);
            if checkline(cpy){
                valid = true;
            }
        }
        if valid {
            result += 1;
        }
    }
    return result;
}

fn rest(mut split: Vec<i32>, i: usize, up: bool, mut strike: bool) -> bool {
    if i >= split.len() - 1 {
        return true;
    }

    if !check(split[i], split[i + 1], up) {
        // println!("{:?} {:?} aaa", split[i], split[i + 1]);
        if !strike {
            strike = true;
            if check(split[i - 1], split[i + 1], up) {
                // remove first
                split[i] = split[i - 1];
            } else {
                // remove second
                split[i + 1] = split[i];
            }
        } else {
            return false;
        }
    }
    return rest(split, i + 1, up, strike);
}

fn checkline(split: Vec<i32>) -> bool {
    let first = split[0];
    let second = split[1];
    let up = first < second;
    if !check(first, second, up) {
        return false;
    }

    for i in 0..split.len() - 1 {
        if !check(split[i], split[i + 1], up) {
            return false;
        }
    }
    return true;
}

fn check(num_a: i32, num_b: i32, up: bool) -> bool {
    if up && num_b <= num_a || !up && num_b >= num_a {
        return false;
    }
    let diff: i32 = (num_a - num_b).abs();
    if !(diff >= 1 && diff <= 3) {
        return false;
    }
    return true;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = 2;
        let example = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9        ";
        assert_eq!(p1(example), expected);
    }

    #[test]
    fn test_part2() {
        let expected = 4;
        let example = r"7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9        ";

        assert_eq!(p2(example), expected);
    }

    #[test]
    fn mt1() {
        assert_eq!(p2("3 1 4 5 6"), 1);
    }

    #[test]
    fn mt2() {
        assert_eq!(p2("1 8 4 5 6"), 1);
    }

    #[test]
    fn mt3() {
        assert_eq!(p2("1 8 4 5 3"), 0);
    }

    #[test]
    fn mt4() {
        assert_eq!(p2("1 2 4 5 3"), 1);
    }

    #[test]
    fn mt5() {
        assert_eq!(p2("4 5 4 5 3"), 0);
    }

    #[test]
    fn mt6() {
        assert_eq!(p2("4 5 3 2 1"), 1);
    }

    #[test]
    fn mt7() {
        assert_eq!(p2("4 5 3 2 6"), 0);
    }
}
