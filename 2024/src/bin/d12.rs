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

pub fn p1(input: &str) -> usize {
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

    let mut q: Vec<(usize, usize)> = Vec::new();
    for y in 0..lines.len(){
        for x in 0..lines[y].len(){
            q.push((x,y));
        }
    }

    let mut result = 0;

    while !q.is_empty(){
        let next = q.pop().unwrap();
        let (area, perimeter) = traverse_neighbours(&lines, &mut q, next.0, next.1);
        result += area * perimeter;
    }

    return result;
}

pub fn p2(input: &str) -> i32 {
    return 0;
}

fn check_if_valid(lines: &Vec<Vec<char>>, x: isize, y: isize) -> Option<&char>{
    if !(x >= 0 && y >= 0 && y < lines.len() as isize && x < lines.len() as isize){
        return None;
    }
    return Some(&lines[y as usize][x as usize]);
}

fn traverse_neighbours(lines: &Vec<Vec<char>>, q: &mut Vec<(usize, usize)>, cx: usize, cy: usize) -> (usize, usize){
    let current_value = lines[cy][cx];

    let cx = cx as isize;
    let cy = cy as isize;
    let left: (isize, isize) = (cx - 1, cy);
    let up: (isize, isize) = (cx, cy - 1);
    let right: (isize, isize) = (cx + 1, cy);
    let down: (isize, isize) = (cx, cy + 1);
    let positions = [left, up, right, down];

    let mut area = 1;
    let mut perimeter = 0;

    for p in positions{
        let p_usize = (p.0 as usize, p.1 as usize);

        let tmp = check_if_valid(lines, p.0, p.1);
        if tmp.is_some(){
            if *tmp.unwrap() != current_value{
                perimeter += 1;
            }else if q.contains(&p_usize){
                let index = q.iter().position(|x| x.0 == p_usize.0 && x.1 == p_usize.1);
                q.remove(index.unwrap());

                let temp = traverse_neighbours(lines, q, p_usize.0, p_usize.1);
                area += temp.0;
                perimeter += temp.1;
            }
        }else{
            perimeter += 1;
        }
    }

    return (area, perimeter);

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = 140;
        let example = r"AAAA
BBCD
BBCC
EEEC";
        assert_eq!(p1(example), expected);
    }

    #[test]
    fn test_part2() {
        let expected = 0;
        let example = r"";
        assert_eq!(p2(example), expected);
    }
}
