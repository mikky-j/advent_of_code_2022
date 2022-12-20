use std::{fs::File, io::Read};

pub fn solution() {
    let mut file = File::open("src/day_1.txt").unwrap();
    let mut text = String::new();
    let _ = file.read_to_string(&mut text).unwrap();
    let mut acc: usize = 0;
    let (mut max1, mut max2, mut max3) = (0, 0, 0);
    for line in text.lines() {
        if line.is_empty() {
            if acc > max1 {
                max3 = max2;
                max2 = max1;
                max1 = acc;
            } else if acc > max2 {
                max3 = max2;
                max2 = acc;
            } else if acc > max3 {
                max3 = acc;
            }
            acc = 0;
        } else {
            let number: usize = line.parse().unwrap();
            acc += number;
        }
    }
    println!("The top one is {max1}");
    println!("The total of the top 3 is {}", max1 + max2 + max3);
}
