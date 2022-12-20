use std::collections::HashSet;

pub fn solution(input: String, distinct_characters: usize) {
    let mut set = HashSet::with_capacity(distinct_characters);
    let mut index = 0;
    for &character in input.as_bytes() {
        let length = set.len();
        if length % (distinct_characters - 1) == 0 && length != 0 {
            if set.contains(&character) {
                set.clear();
                set.insert(character);
            } else {
                break;
            }
        } else {
            if !set.insert(character) {
                set.clear();
            }
        }
        index += 1;
    }

    // If y
    println!("The answer is {index}");
}
