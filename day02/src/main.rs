use std::{cmp::max, fs};

enum Color {
    Red(u32),
    Green(u32),
    Blue(u32),
}

fn parse_color(set: &str) -> Option<Color> {
    let split_color = set.split_once(" ").unwrap();
    let num = split_color.0.parse::<u32>().unwrap();
    let color = split_color.1;

    match color {
        "red" => Some(Color::Red(num)),
        "green" => Some(Color::Green(num)),
        "blue" => Some(Color::Blue(num)),
        _ => None,
    }
}

fn parse_game(line: &str) -> (u32, u32, u32) {
    let game_sets = line.split("; ");

    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    for set in game_sets {
        let colors = set.split(", ");
        for color in colors {
            let parsed_color = parse_color(color);

            match parsed_color {
                Some(Color::Red(num)) => max_red = max(num, max_red),
                Some(Color::Green(num)) => max_green = max(num, max_green),
                Some(Color::Blue(num)) => max_blue = max(num, max_blue),
                None => continue,
            }
        }
    }

    return (max_red, max_green, max_blue);
}

fn parse_input(input: &String, power: bool) -> u32 {
    let lines = input.lines();
    let mut value = 0;

    for line in lines {
        let split_line = line.split_once(": ").unwrap();

        let game = split_line.0.split_once(" ").unwrap();
        let game_id = game.1.parse::<u32>().unwrap();
        let game_sets = split_line.1;

        let (red, green, blue) = parse_game(game_sets);
        if power {
            value += red * green * blue;
        } else if red <= 12 && green <= 13 && blue <= 14 {
            value += game_id;
        }
    }
    return value;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_one() {
        let input_contents = fs::read_to_string("./test.txt").expect("Expected test file");

        let sum = parse_input(&input_contents, false);

        assert_eq!(sum, 8);
    }
    #[test]
    fn part_two() {
        let input_contents = fs::read_to_string("./test.txt").expect("Expected test file");

        let sum = parse_input(&input_contents, true);

        assert_eq!(sum, 2286);
    }
}

fn main() {
    let input_contents =
        fs::read_to_string("./input.txt").expect("Expected input file at input.txt");

    let sum_one = parse_input(&input_contents, false);
    let sum_two = parse_input(&input_contents, true);

    println!("Sum of all valid IDs (part one): {}", sum_one);
    println!("Multiply of all max color values (part two): {}", sum_two);
}
