use std::{fs::File, io::Read};



fn main(){
    let day = file!().split('/').last().unwrap()[1..3].to_owned();

    let path = format!("src/input/input{day}.txt");
    println!("{}", path);
    let mut file = File::open(path).expect("File not found");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("cannot read file");

    println!("{}", p1(&input));
    println!("{}", p2(&input));
}


pub fn p1(input: &str) -> i32{


    return 0;
}

pub fn p2(input: &str) -> i32{


    return 0;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = 0;
        let example = r"

        ";
        assert_eq!(p1(example), expected);
    }

    #[test]
    fn test_part2() {
        let expected = 0;
        let example = r"

        ";
        assert_eq!(p2(example), expected);
    }
}
