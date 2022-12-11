/** to calculate visibility from one direction:
subtract the value in front from the value behind: 0 and below is invisible
for each field there are 4 directions **/


pub fn process_part1(input: &str) -> String {
    let parsed_input: Vec<String> = input
        .lines()
        .map(|line| line.to_string())
        .collect();

    let reformatted_input: Vec<Vec<i32>> =
        parsed_input
            .iter()
            .map(|line| {
                line
                    .chars()
                    .map(|char| char.to_digit(10).unwrap() as i32)
                    .collect()
            })
            .collect();

    visibilty(sequence: Vec<i32>);

    return String::from_iter(parsed_input);
}


pub fn visibility() -> Vec<bool> {
    let testvector: Vec<i32> = Vec::from_iter("30373".chars().map(|char| char.to_digit(10).unwrap() as i32));
    let mut result_main_dir: Vec<bool> = vec![];

    result_main_dir = testvector
        .windows(2)
        .map()

    return result_main_dir;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_input_then() {
        let input =
            "30373
        25512
        65332
        33549
        35390";
        let result = process_part1(input);
        assert_eq!(result.parse::<i32>().unwrap(), 21)
    }
}

pub fn process_part2(input: &str) -> String {
    return "21".to_string();
}
