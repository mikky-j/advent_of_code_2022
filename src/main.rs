fn main() {
    let input = include_str!("inputs/day_9.txt");
    let start = std::time::Instant::now();
    adventofcode2022::day9::solution(input);
    println!("took {} μs to run", start.elapsed().as_micros());
}
