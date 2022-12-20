use std::{fs::File, io::Read};

pub fn solution1() {
    let mut file = File::open("src/day_4.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let mut acc = 0;
    for line in input.lines() {
        let numbers: Vec<_> = line
            .split(&['-', ','][..])
            .map(|x| x.parse::<u8>().unwrap())
            .collect();
        if (numbers[0] <= numbers[2] && numbers[1] >= numbers[3])
            || (numbers[2] <= numbers[0] && numbers[3] >= numbers[1])
        {
            acc += 1usize
        }
    }
    println!("The solution is {acc}");
}

pub fn solution2() {
    let mut file = File::open("src/day_4.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let mut acc = 0;
    for line in input.lines() {
        let numbers: Vec<_> = line
            .split(&['-', ','][..])
            .map(|x| x.parse::<u8>().unwrap())
            .collect();
        if (numbers[0] <= numbers[2] && numbers[2] <= numbers[1])
            || (numbers[0] <= numbers[3] && numbers[3] <= numbers[1])
            || (numbers[2] <= numbers[0] && numbers[0] <= numbers[3])
            || (numbers[2] <= numbers[1] && numbers[1] <= numbers[3])
        {
            acc += 1usize
        }
    }
    println!("The solution is {acc}");
}
