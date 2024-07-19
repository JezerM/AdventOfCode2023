use std::fs;

#[derive(Debug)]
enum Symbol {
    Number(u32, usize, usize),
    Symbol(char, usize, usize),
    None,
}

fn symbol_intersects(number: &Symbol, symbol: &Symbol) -> bool {
    match (number, symbol) {
        (Symbol::Number(num, i, j), Symbol::Symbol(_, y, x)) => {
            if (*i as i32) < (*y as i32) - 1 || *i > *y + 1 {
                return false;
            }

            let num_length: usize = (num.checked_ilog10().unwrap_or(0) + 1).try_into().unwrap();
            let num_start = j + 1 - num_length;
            let num_end = *j;

            return (num_start >= x - 1 && num_start <= x + 1)
                || (num_end >= x - 1 && num_end <= x + 1);
        }
        _ => {}
    }
    return false;
}

fn part_numbers_sum(symbols: &Vec<Symbol>) -> u32 {
    let mut sum = 0;

    for symbol in symbols {
        if let Symbol::Number(number, _, _) = symbol {
            let found = symbols.iter().find(|s| symbol_intersects(symbol, s));
            if found.is_some() {
                sum += number;
            }
        }
    }

    return sum;
}

fn gear_ratio_sum(symbols: &Vec<Symbol>) -> u32 {
    let mut sum = 0;

    for symbol in symbols {
        if let Symbol::Symbol(char, _, _) = symbol {
            if *char != '*' {
                continue;
            }
            let mut filtered = symbols.iter().filter(|s| symbol_intersects(s, symbol));
            let count = filtered.clone().count();

            if count == 2 {
                let first = filtered.next().unwrap();
                let second = filtered.next().unwrap();

                let ratio = match (first, second) {
                    (Symbol::Number(n1, _, _), Symbol::Number(n2, _, _)) => n1 * n2,
                    _ => 0,
                };

                sum += ratio;
            }
        }
    }

    return sum;
}

fn parse_input(input: &String) -> Vec<Symbol> {
    let mut values = vec![];

    for (i, line) in input.lines().enumerate() {
        let mut num_str = String::new();

        for (j, char) in line.char_indices() {
            if char.is_digit(10) {
                num_str.push(char);
            }
            if !num_str.is_empty() {
                let number = num_str.parse::<u32>().unwrap();

                let value = if char.is_ascii_punctuation() {
                    Symbol::Number(number, i, j)
                } else if j == line.len() - 1 {
                    Symbol::Number(number, i, j + 1)
                } else {
                    Symbol::None
                };

                if !matches!(value, Symbol::None) {
                    values.push(value);
                    num_str.clear();
                }
            }
            if char.is_ascii_punctuation() && char != '.' {
                let value = Symbol::Symbol(char, i, j + 1);
                values.push(value);
            }
        }
    }

    return values;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_one() {
        let input_contents = fs::read_to_string("./test.txt").expect("Expected test file");

        let symbols = parse_input(&input_contents);
        let sum = part_numbers_sum(&symbols);

        assert_eq!(sum, 4361);
    }
    #[test]
    fn part_two() {
        let input_contents = fs::read_to_string("./test.txt").expect("Expected test file");

        let symbols = parse_input(&input_contents);
        let sum = gear_ratio_sum(&symbols);

        assert_eq!(sum, 467835);
    }
}

fn main() {
    let input_contents =
        fs::read_to_string("./input.txt").expect("Expected input file at input.txt");

    let symbols = parse_input(&input_contents);

    // println!("Symbols: {:?}", symbols);

    let sum_one = part_numbers_sum(&symbols);
    let sum_two = gear_ratio_sum(&symbols);

    println!("Sum of all part numbers (part one): {}", sum_one);
    println!("Sum of all gear ratios (part one): {}", sum_two);
}
