use std::collections::HashMap;
use std::time::Instant;
use std::vec;
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
    let mut split = input.split("\n\n");
    let page_ordering_rules = split.next().expect("no page rules").lines();
    let updates = split.next().expect("no updates");
    let mut update_nums: Vec<Vec<u32>> = updates
        .lines()
        .map(|f| f.split(',').map(|f| f.parse::<u32>().unwrap()).collect())
        .collect();
    let mut incorrect_updates: Vec<Vec<u32>> = Vec::new();

    for line in page_ordering_rules {
        let fst_num = line
            .split('|')
            .next()
            .expect("no first element")
            .parse::<u32>()
            .expect("should be a num");
        let snd_num = line
            .split('|')
            .next_back()
            .expect("no second element")
            .parse::<u32>()
            .expect("should be a num");

        for i in (0..update_nums.len()).rev() {
            let row = &update_nums[i];
            let mut incorrect = false;
            if row.contains(&fst_num) && row.contains(&snd_num) {
                let mut found_fst = false;
                for j in row {
                    if j == &fst_num {
                        found_fst = true;
                    } else if j == &snd_num && !found_fst {
                        incorrect = true;
                    }
                }
            }
            if incorrect {
                incorrect_updates.push(update_nums.remove(i));
            }
        }
    }

    let mut result: i32 = 0;
    for l in update_nums {
        result += l[l.len() / 2] as i32;
    }
    return result;
}

pub fn p2(input: &str) -> i32 {
    let mut split = input.split("\n\n");
    let page_ordering_rules = split.next().expect("no page rules").lines().map(|f| {
        (
            f.split('|')
                .next()
                .expect("no first element")
                .parse::<u32>()
                .expect("should be a num"),
            f.split('|')
                .next_back()
                .expect("no second element")
                .parse::<u32>()
                .expect("should be a num"),
        )
    });
    let mut rule_map: HashMap<u32, Vec<u32>> = HashMap::new();
    // fill map with vecs of numbers that should come after them
    for (p1, p2) in page_ordering_rules.clone() {
        let element: &Vec<u32> = match rule_map.get(&p1){
            Some(e) => e,
            _ => &Vec::new()
        };
        let mut element: Vec<u32> = element.clone();
        element.push(p2);
        rule_map.insert(p1, element);
    }

    let updates = split.next().expect("no updates");

    let mut update_nums: Vec<Vec<u32>> = updates
        .lines()
        .map(|f| f.split(',').map(|f| f.parse::<u32>().unwrap()).collect())
        .collect();
    let mut incorrect_updates: Vec<Vec<u32>> = Vec::new();

    for (fst_num, snd_num) in page_ordering_rules {

        for i in (0..update_nums.len()).rev() {
            let row = &update_nums[i];

            let incorrect = is_update_incorrect(row, &fst_num, &snd_num);
            if incorrect {
                let mut new_row = row.clone();
                new_row.sort_by(|a, b| {
                    let fst_count = rule_map.get(a).unwrap_or(&vec![]).iter().filter(|e| row.contains(e)).count() as isize;
                    let snd_count = rule_map.get(b).unwrap_or(&vec![]).iter().filter(|e| row.contains(e)).count() as isize;
                    let diff: isize = fst_count - snd_count;
                    if diff == 0 {
                        std::cmp::Ordering::Equal
                    }else if diff > 0{
                        std::cmp::Ordering::Greater
                    }else{
                        std::cmp::Ordering::Less
                    }});
                update_nums.remove(i);
                incorrect_updates.push(new_row);
            }
        }
    }
    let mut result: i32 = 0;
    for l in &incorrect_updates {
        result += l[l.len() / 2] as i32;
    }
    return result;
}

fn is_update_incorrect(row: &Vec<u32>, fst_num: &u32, snd_num: &u32) -> bool {
    if row.contains(&fst_num) && row.contains(&snd_num) {
        let mut found_fst = false;

        for j in 0..row.len() {
            if row[j] == *fst_num {
                found_fst = true;
            } else if row[j] == *snd_num && !found_fst {
                return true;
            }
        }
    }
    return false;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = 143;
        let example = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(p1(example), expected);
    }

    #[test]
    fn test_part2() {
        let expected = 123;
        let example = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(p2(example), expected);
    }
}
