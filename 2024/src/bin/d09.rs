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

pub fn p1(input: &str) -> u64 {
    let diskmap = diskmap_to_vec(input);
    let diskmap: Vec<u32> = reorder_diskmap(diskmap)
        .iter()
        .filter(|e| e.is_some())
        .map(|e| e.unwrap())
        .collect();

    let mut result: u64 = 0;
    for i in 0..diskmap.len() {
        result += ((i as u32) * diskmap[i]) as u64;
    }
    return result;
}

fn reorder_diskmap(mut diskmap: Vec<Option<u32>>) -> Vec<Option<u32>> {
    let mut lptr: usize = 0;
    let mut rptr: usize = diskmap.len() - 1;
    while lptr < rptr {
        if diskmap[lptr].is_some() {
            lptr += 1;
            continue;
        }
        if diskmap[rptr].is_none() {
            rptr -= 1;
            continue;
        }
        diskmap.swap(lptr, rptr);
        lptr += 1;
        rptr -= 1;
    }

    return diskmap;
}

fn diskmap_to_vec(diskmap: &str) -> Vec<Option<u32>> {
    let diskmap_nums: Vec<u32> = diskmap.chars().map(|c| c as u32 - '0' as u32).collect();

    let mut result: Vec<Option<u32>> = Vec::new();
    for i in 0..diskmap_nums.len() {
        // push disks
        if i % 2 == 0 {
            for _ in 0..diskmap_nums[i] {
                result.push(Some(i.div_ceil(2) as u32));
            }
        }
        // push free spaces
        else {
            for _ in 0..diskmap_nums[i] {
                result.push(None);
            }
        }
    }

    return result;
}

pub fn p2(input: &str) -> u64 {
    let diskmap = diskmap_to_file_vec(input);
    let diskmap = reorder_file_diskmap(diskmap);

    let mut resultvec: Vec<Option<usize>> = Vec::new();

    for f in diskmap{
        let to_push = if f.is_empty { None } else {Some(f.id)};
        for _ in 0..f.space{
            resultvec.push(to_push);
        }
    }


    let mut result: u64 = 0;
    for i in 0..resultvec.len() {
        if resultvec[i].is_none(){
            continue;
        }
        result += (i * resultvec[i].unwrap()) as u64;
    }
    return result;
}

#[derive(Debug)]
#[derive(Clone)]
struct DataFile {
    is_empty: bool,
    id: usize,
    space: u32,
}

fn diskmap_to_file_vec(diskmap: &str) -> Vec<DataFile> {
    let diskmap_nums: Vec<u32> = diskmap.chars().map(|c| c as u32 - '0' as u32).collect();
    let mut result: Vec<DataFile> = Vec::new();
    for i in 0..diskmap_nums.len() {
        // ignore empty
        if diskmap_nums[i] == 0{
            continue;
        }
        // push disks
        let empty = i % 2 != 0;
        let id = if empty {0} else {i.div_ceil(2)} ;
        result.push(DataFile {
            is_empty: empty,
            id: id,
            space: diskmap_nums[i],
        });
    }

    return result;
}

fn reorder_file_diskmap(mut diskmap: Vec<DataFile>) -> Vec<DataFile> {
    let mut rptr: usize = diskmap.len() - 1;
    while rptr > 0 {
        if diskmap[rptr].is_empty {
            rptr -= 1;
            continue;
        }
        for lptr in 0..rptr {
            if !diskmap[lptr].is_empty{
                continue;
            }
            if diskmap[lptr].space >= diskmap[rptr].space {
                let diff = diskmap[lptr].space - diskmap[rptr].space;
                if diff == 0{
                    diskmap.swap(lptr, rptr);
                    break;
                }
                if diff > 0{
                    diskmap[lptr] = diskmap[rptr].clone();
                    diskmap.insert(lptr + 1, DataFile{is_empty: true, id: 0, space: diff});
                    // increase rptr by one because one additional (empty) datafile has been created somewhere to its left
                    rptr += 1;
                    diskmap[rptr] = DataFile{is_empty: true, id: 0, space: diskmap[lptr].space};
                    break;
                }
            }
        }
        rptr -= 1;
    }

    return diskmap;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = 1928;
        let example = r"2333133121414131402";
        assert_eq!(p1(example), expected);
    }

    #[test]
    fn test_part2() {
        let expected = 2858;
        let example = r"2333133121414131402";
        assert_eq!(p2(example), expected);
    }
}
