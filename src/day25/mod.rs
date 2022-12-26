use core::panic;
use std::fs;

use pathfinding::num_traits::PrimInt;

#[allow(dead_code)]
fn print_vec<T: std::fmt::Display>(name: String, vec: &Vec<T>) {
    print!("{name}: [");
    for val in vec {
        print!("{val},");
    }
    println!("]");
}

fn convert_snafu_to_string(snafu: &Vec<char>) -> String {
    let mut result: String = "".to_string();
    for snafu_char in snafu {
        result += snafu_char.to_string().as_str();
    }
    return result;
}

fn convert_single_decimal_to_single_snafu(single_decimal: i128) -> char {
    match single_decimal {
        2 => return '2',
        1 => return '1',
        0 => return '0',
        -1 => return '-',
        -2 => return '=',
        _ => panic!("invalid single decimal"),
    }
}

fn convert_single_digit_snafu_to_decimal(single_digit_snafu: char) -> i128 {
    match single_digit_snafu {
        '2' => return 2,
        '1' => return 1,
        '0' => return 0,
        '-' => return -1,
        '=' => return -2,
        _ => panic!("invalid single digit snafu"),
    }
}

fn get_snafu_places_needed(decimal: u128) -> u128 {
    let mut snafu_places = 1;
    let mut factor = 1;
    let mut sum = 0;
    loop {
        sum += factor * 2;
        if decimal <= sum {
            return snafu_places;
        }

        factor *= 5;
        snafu_places += 1;
    }
}

fn convert_decimal_to_snafu(decimal: i128) -> Vec<char> {
    let mut decimal_number = decimal;
    let mut snafu = Vec::new();
    let snafu_places = get_snafu_places_needed(decimal.abs() as u128);
    for snafu_place in (0..snafu_places).rev() {
        let factor = 5.pow(snafu_place.try_into().unwrap());
        let mut snafu_value = 0;
        let mut min_diff = u128::MAX;
        for snafu_val in [-2, -1, 0, 1, 2] {
            let abs_diff = decimal_number.abs_diff(snafu_val * factor);
            if abs_diff < min_diff {
                min_diff = abs_diff;
                snafu_value = snafu_val;
            }
        }
        decimal_number -= snafu_value * factor;
        snafu.push(convert_single_decimal_to_single_snafu(snafu_value));
    }

    return snafu;
}

fn convert_snafu_to_decimal(snafu: &Vec<char>) -> i128 {
    let mut decimal = 0;
    let mut factor = 1;
    for single_digit_snafu in snafu.iter().rev() {
        decimal += factor * convert_single_digit_snafu_to_decimal(*single_digit_snafu);
        factor *= 5;
    }
    return decimal;
}

fn sum_snafu_list(snafu_list: &Vec<Vec<char>>) -> i128 {
    let mut sum = 0;
    for snafu in snafu_list {
        sum += convert_snafu_to_decimal(snafu);
    }
    return sum;
}

fn get_input(file: &str) -> Vec<Vec<char>> {
    let input = fs::read_to_string(file).expect("Should have been able to read the file");
    let rows: Vec<&str> = input.split("\r\n").collect();
    let mut snafu_list: Vec<Vec<char>> = Vec::new();
    for row_string in rows {
        snafu_list.push(row_string.chars().collect());
    }
    return snafu_list;
}

pub fn solver(_: bool) {
    let snafu_list = get_input("./src/day25/input.txt");

    println!("Day25:");
    let sum = sum_snafu_list(&snafu_list);
    println!("Sum of snafu list: {sum}");

    let snafu_sum = convert_decimal_to_snafu(sum);
    let snafu_string = convert_snafu_to_string(&snafu_sum);

    println!("Snafu for Bob's console: {snafu_string}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day25_simple_example_get_input() {
        let snafu_list = get_input("./src/day25/example_input.txt");
        let sum = sum_snafu_list(&snafu_list);
        assert_eq!(4890, sum);
        let snafu_sum = convert_decimal_to_snafu(sum);
        let snafu_string = convert_snafu_to_string(&snafu_sum);
        assert_eq!("2=-1=0", snafu_string);
    }

    #[test]
    fn day25_decimal_to_snafu_to_decimal() {
        let decimal = 4890;
        let snafu = convert_decimal_to_snafu(decimal);
        assert_eq!("2=-1=0", convert_snafu_to_string(&snafu));
        assert_eq!(4890, convert_snafu_to_decimal(&snafu));
    }

    #[test]
    fn day25_part_one() {
        let snafu_list = get_input("./src/day25/input.txt");
        let sum = sum_snafu_list(&snafu_list);
        assert_eq!(30638862852576, sum);
        let snafu_sum = convert_decimal_to_snafu(sum);
        let snafu_string = convert_snafu_to_string(&snafu_sum);
        assert_eq!("2=01-0-2-0=-0==-1=01", snafu_string);
        assert_eq!(30638862852576, convert_snafu_to_decimal(&snafu_sum));
    }
}
