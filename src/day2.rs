use std::{fs::File, io::Read};

// Opponents input - A(Rock), B(paper), C(Scissors)
// Your Input - X(Rock), Y(Paper), Z(Scissors)
pub fn solution() {
    let mut file = File::open("src/day_2.txt").unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    let (naive, actual) = text
        .lines()
        .fold((0_usize, 0_usize), |(naive, actual), input| {
            let (calc_naive, calc_actual) = match input {
                "A X" => (4, 3),
                "B X" => (1, 1),
                "C X" => (7, 2),
                "A Y" => (8, 4),
                "B Y" => (5, 5),
                "C Y" => (2, 6),
                "A Z" => (3, 8),
                "B Z" => (9, 9),
                "C Z" => (6, 7),
                _ => panic!(),
            };
            (naive + calc_naive, actual + calc_actual)
        });

    println!("The naive answer is {naive}");
    println!("The actual answer is {actual}");
}
