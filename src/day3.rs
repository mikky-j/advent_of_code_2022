use std::{collections::HashSet, fs::File, io::Read};

pub fn solution1() {
    let mut file = File::open("src/day_3.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let mut acc = 0;
    let mut set = HashSet::new();
    for line in input.lines() {
        let length = line.len();
        let half = length / 2;
        for (index, character) in line.bytes().enumerate() {
            if index >= half {
                if set.contains(&character) {
                    if character.is_ascii_lowercase() {
                        acc += (character - 96) as usize;
                    } else {
                        acc += (character - 38) as usize;
                    }
                    break;
                }
            } else {
                set.insert(character);
            }
        }
        set.clear();
    }

    println!("The solution is {acc}");
}

pub fn solution2() {
    let mut file = File::open("src/day_3.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let lines: Vec<_> = input.lines().collect();
    let mut set1 = HashSet::new();
    let mut set2 = HashSet::new();
    let mut acc = 0;
    for line in lines.chunks_exact(3) {
        line[0].as_bytes().iter().for_each(|byte| {
            set1.insert(byte);
        });
        line[1].as_bytes().iter().for_each(|byte| {
            set2.insert(byte);
        });
        let common: HashSet<_> = set1.intersection(&set2).collect();
        for character in line[2].as_bytes() {
            if common.contains(&character) {
                if character.is_ascii_lowercase() {
                    acc += (character - 96) as usize;
                } else {
                    acc += (character - 38) as usize;
                }
                break;
            }
        }
        set1.clear();
        set2.clear();
    }
    println!("The solution is {acc}");
}
