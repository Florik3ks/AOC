use std::collections::HashMap;
use std::{fs::File, io::Read};
use std::time::Instant;


fn main(){
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


pub fn p1(input: &str) -> u64{
    return solve(input, 25);
}
pub fn p2(input: &str) -> u64{
    return solve(input, 75);
}

fn solve(input: &str, steps: usize) -> u64{
    let mut map: HashMap<u64, u64> = HashMap::new();
    let nums: Vec<u64> = input.split_whitespace().map(|f| f.parse::<u64>().expect("a")).collect();
    for n in nums{
        map.insert(n, 1);
    }
    for _ in 0..steps{
        iterate(&mut map);
    }
    let mut result: u64 = 0;
    for v in map.values(){
        result += *v as u64;
    }

    return result;
}
fn add_map(map: &mut HashMap<u64, u64>, key: u64, to_add: u64){
    match map.get(&key) {
        Some(count) => { map.insert(key, count + to_add); }
        None => { map.insert(key, to_add); }
    }
    
}
fn iterate(map: &mut HashMap<u64, u64>){
    let mut new_map: HashMap<u64, u64> = HashMap::new();
    // todo: increment instead of overwrite
    for key in map.keys() {
        if *key == 0 {
            add_map(&mut new_map, 1, map[key]);
            continue;
        }
        let log: u64 = (*key as f64).log10() as u64 + 1;
        if log % 2 == 0 {
            let tmp = (10 as u64).pow((log / 2) as u32);
            let left: u64 =  *key / tmp;
            add_map(&mut new_map, left, map[key]);
            
            let right = *key - (left * tmp);
            add_map(&mut new_map, right, map[key]);
        }else{
            add_map(&mut new_map, *key * 2024, map[key]);
        }
    }
    *map = new_map;
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = 55312;
        let example = r"125 17";
        assert_eq!(p1(example), expected);
    }

    #[test]
    fn test_part2() {
        let expected = 0;
        let example = r"";
        assert_eq!(p2(example), expected);
    }
}
