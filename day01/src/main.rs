use std::fs;

fn parse_calibration_words(line: &str) -> u32 {
    let mut first = 0;
    let mut last = 0;

    let mut word = String::with_capacity(line.len());

    for char in line.chars() {
        let parsed = char.to_digit(10);

        if let Some(num) = parsed {
            word.clear();

            if first == 0 {
                first = num;
                last = num;
            } else {
                last = num;
            }
            continue;
        }

        word.push(char);

        let num = if word.ends_with("one") {
            1
        } else if word.ends_with("two") {
            2
        } else if word.ends_with("three") {
            3
        } else if word.ends_with("four") {
            4
        } else if word.ends_with("five") {
            5
        } else if word.ends_with("six") {
            6
        } else if word.ends_with("seven") {
            7
        } else if word.ends_with("eight") {
            8
        } else if word.ends_with("nine") {
            9
        } else {
            0
        };
        if num != 0 {
            if first == 0 {
                first = num;
                last = num;
            } else {
                last = num;
            }
        }
    }

    let value = first * 10 + last;
    return value;
}

fn parse_calibration(line: &str) -> u32 {
    let mut first = 0;
    let mut last = 0;

    for char in line.chars() {
        let parsed = char.to_digit(10);
        if let Some(num) = parsed {
            first = num;
            break;
        }
    }

    for char in line.chars().rev() {
        let parsed = char.to_digit(10);
        if let Some(num) = parsed {
            last = num;
            break;
        }
    }
    let value = first * 10 + last;
    return value;
}

fn parse_input(input: &String, words: bool) -> u32 {
    let mut calibration_values = Vec::new();

    for line in input.lines() {
        let calibration = if words {
            parse_calibration_words(line)
        } else {
            parse_calibration(line)
        };
        calibration_values.push(calibration);
    }

    return calibration_values.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_one() {
        let input_contents = fs::read_to_string("./test.txt").expect("Expected test file");

        let sum = parse_input(&input_contents, false);

        assert_eq!(sum, 142);
    }
    #[test]
    fn part_two() {
        let input_contents = fs::read_to_string("./test-2.txt").expect("Expected test file");

        let sum = parse_input(&input_contents, true);

        assert_eq!(sum, 281);
    }
}

fn main() {
    let input_contents =
        fs::read_to_string("./input.txt").expect("Expected input file at input.txt");

    let sum_one = parse_input(&input_contents, false);
    let sum_two = parse_input(&input_contents, true);

    println!("Sum of all calibration values (part one): {}", sum_one);
    println!("Sum of all calibration values (part two): {}", sum_two);
}
