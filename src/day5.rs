use std::{collections::VecDeque, fs::File, io::Read};

pub fn solution() {
    let mut file = File::open("src/day_5_large.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let mut stacks = vec![
        VecDeque::<char>::new(),
        VecDeque::<char>::new(),
        VecDeque::<char>::new(),
        VecDeque::<char>::new(),
        VecDeque::<char>::new(),
        VecDeque::<char>::new(),
        VecDeque::<char>::new(),
        VecDeque::<char>::new(),
        VecDeque::<char>::new(),
    ];

    for line in input.lines().take_while(|x| x.starts_with('[')) {
        for (index, character) in line.chars().skip(1).step_by(4).enumerate() {
            if character != ' ' {
                stacks[index].push_front(character);
            }
        }
    }

    for instruction in input.lines().skip(10) {
        let length = instruction.len();
        let mut instructon_char = instruction.chars();
        let (items, from, to) = match length {
            18 => {
                let items = instructon_char.nth(5).unwrap() as usize - 48;
                let from = instructon_char.nth(6).unwrap() as usize - 49;
                let to = instructon_char.last().unwrap() as usize - 49;
                (items, from, to)
            }
            19 => {
                let items = instruction.get(5..=6).unwrap().parse::<usize>().unwrap();
                let from = instructon_char.nth(13).unwrap() as usize - 49;
                let to = instructon_char.last().unwrap() as usize - 49;
                (items, from, to)
            }
            _ => panic!(),
        };

        let from_length = stacks[from].len() - items;

        let removed_items: Vec<_> = stacks[from].drain(from_length..).collect();
        stacks[to].extend(&removed_items);
    }

    println!("The solution is ");
    stacks.iter_mut().for_each(|stack| {
        if let Some(character) = stack.pop_back() {
            print!("{}", character);
        }
    });
    println!();
}

pub fn faster_solution() {
    let mut file = File::open("src/day_5_large.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let mut stacks = vec![];
    let number_of_stacks = (input.split_terminator('\n').next().unwrap().len() + 1) / 4;
    (0..number_of_stacks).for_each(|_| stacks.push(VecDeque::<char>::new()));
    for line in input.split_terminator('\n') {
        // Building the initial stacks
        if line.starts_with('[') {
            for (index, character) in line.chars().skip(1).step_by(4).enumerate() {
                if character != ' ' {
                    stacks[index].push_front(character);
                }
            }
        }
        // Performing the moves
        else if line.starts_with('m') {
            let mut move_array = line.split(' ');
            let items = move_array.nth(1).unwrap().parse::<usize>().unwrap();
            let from = move_array.nth(1).unwrap().parse::<usize>().unwrap() - 1;
            let to = move_array.nth(1).unwrap().parse::<usize>().unwrap() - 1;

            let from_length = stacks[from].len() - items;
            let removed_items: Vec<char> = stacks[from].drain(from_length..).collect();
            stacks[to].extend(removed_items);
        }
    }

    println!("The solution is ");
    stacks.iter_mut().for_each(|stack| {
        if let Some(character) = stack.pop_back() {
            print!("{}", character);
        }
    });
    println!();
}
