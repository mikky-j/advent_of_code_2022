use std::collections::HashSet;

pub fn solution(input: &str) {
    let (mut head_row, mut head_col) = (0isize, 0isize);
    let (mut tail_row, mut tail_col) = (0isize, 0isize);
    let mut v = Vec::<usize>::new();
    let mut spots = HashSet::new();
    spots.insert((tail_row, tail_col));
    for line in input.split_terminator('\n') {
        let (direction, distance) = line.split_once(' ').unwrap();
        let distance = distance.parse::<usize>().unwrap();
        for _ in 0..distance {
            match direction {
                // Moving left
                "L" => {
                    head_col -= 1;
                    if head_col.abs_diff(tail_col) > 2 {
                        tail_col -= 1;
                        spots.insert((tail_row, tail_col));
                    }
                }
                "R" => {}
                "U" => {}
                "D" => {}

                _ => unreachable!(),
            }
        }
    }
}
