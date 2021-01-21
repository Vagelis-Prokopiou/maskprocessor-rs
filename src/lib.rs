use std::io::Write;

pub const PW_MAX: u8 = 255;
pub const CHARSET_LOWER: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
pub const CHARSET_UPPER: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
pub const CHARSET_DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
pub const CHARSET_SPECIAL: [char; 33] = [' ', '!', '\"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~'];
pub const CHARSET_ALL: [char; 95] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ' ', '!', '\"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~'];

pub fn recurse_print(data: &Vec<Vec<char>>, list_to_twiddle: usize, current_permutation: &mut Vec<char>) {
    // See: https://stackoverflow.com/questions/3536833/arbitrary-number-of-nested-loops
    if list_to_twiddle == data.len() {
        for i in current_permutation {
            print!("{}", i);
        }
        println!();
        return;
    }

    for i in 0..data[list_to_twiddle].len() {
        current_permutation.push(data[list_to_twiddle][i]);
        recurse_print(data, list_to_twiddle + 1, current_permutation);
        current_permutation.pop();
    }
}

pub fn recurse_write(data: &Vec<Vec<char>>
                     , list_to_twiddle: usize
                     , current_permutation: &mut Vec<char>
                     , buffer_to_write: &mut Vec<u8>) {
    if list_to_twiddle == data.len() {
        for i in current_permutation {
            write!(buffer_to_write, "{}", i).expect("recurse_write: Failed to write");
        }
        writeln!(buffer_to_write).expect("recurse_write: Failed to write");
        return;
    }

    for i in 0..data[list_to_twiddle].len() {
        current_permutation.push(data[list_to_twiddle][i]);
        recurse_write(data, list_to_twiddle + 1, current_permutation, buffer_to_write);
        current_permutation.pop();
    }
}

pub fn get_charset_special() -> Vec<char> {
    let mut characters: Vec<char> = vec![];
    // 1st batch
    for i in 32u8..=47 {
        characters.push(i as char);
    }
    // 2nd batch
    for i in 58u8..=64 {
        characters.push(i as char);
    }
    // 3rd batch
    for i in 91u8..=96 {
        characters.push(i as char);
    }
    // 4th batch
    for i in 123u8..=126 {
        characters.push(i as char);
    }
    return characters;
}

pub fn get_charset_ascii_all() -> Vec<char> {
    let mut characters: Vec<char> = vec![];
    for i in 0x00u8..=0xff {
        characters.push(i as char);
    }
    return characters;
}


/*
fn validate_increment(increment: &str) {
    let numbers: Vec<u8> = matches.values_of(increment).unwrap()
        .into_iter()
        .map(|value| value.parse::<u8>().unwrap())
        .collect();
    if numbers[0] < 1 {
        eprintln!("The first of the increment arguments must be greater than 0.");
        exit(1);
    }
    if numbers[1] < numbers[0] {
        eprintln!("The second of the increment arguments must be greater than the first.");
        exit(1);
    }
}
*/

pub fn get_number_of_combinations(input: &Vec<Vec<char>>) -> u64 {
    return input
        .iter()
        .map(|charset| charset.len())
        .fold(1, |acc, x| acc * x as u64);
}

pub fn increment_values_get(input: &str) -> Vec<u8> {
    let result: Vec<&str> = input.split(':')
        .into_iter()
        .filter(|value| *value != "")
        .collect();
    assert_eq!(result.len(), 2);

    let result: Vec<u8> = result
        .into_iter()
        .map(|i| i.parse::<u8>().unwrap())
        .collect();
    assert!(result[0] > 0);
    assert!(result[1] >= result[0]);

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment_values_get() {
        assert_eq!(increment_values_get("1:2"), vec![1, 2]);
    }

    #[test]
    fn test_get_number_of_combinations() {
        let input = vec![vec!['a']];
        assert_eq!(get_number_of_combinations(&input), 1);

        let input = vec![vec!['a', 'b'], vec!['a', 'b', 'c']];
        assert_eq!(get_number_of_combinations(&input), 6);

        // ?a?a?a?a?a == 7737809375
        let input = vec![
            vec!['a'; 95]
            , vec!['a'; 95]
            , vec!['a'; 95]
            , vec!['a'; 95]
            , vec!['a'; 95]
        ];
        assert_eq!(get_number_of_combinations(&input), 7737809375);
    }
}
