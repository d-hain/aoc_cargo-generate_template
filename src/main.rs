fn main() {
    println!("Advent of Code 2022 Day X:\n"); //TODO Day
    
    let input = std::fs::read_to_string("../input.txt").unwrap();
    
    println!("Part 1: {}", rust::part1(&input));
    println!("Part 2: {}", rust::part2(&input));
}
