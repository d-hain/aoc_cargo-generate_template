pub fn part1(input: &str) -> u32 {
    //TODO: Part 1

    0
}

pub fn part2(input: &str) -> u32 {
    //TODO: Part 2
    
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_working(){
        let example_input = std::fs::read_to_string("../example.txt").unwrap();

        assert_eq!(crate::part1(&example_input), /*TODO: Example Result*/);
    }

    #[test]
    #[ignore] //TODO: Remove when starting Part 2
    fn part2_working(){
        let example_input = std::fs::read_to_string("../example.txt").unwrap();

        assert_eq!(crate::part2(&example_input), /*TODO: Example Result*/);
    }
}
