pub const PW_MAX: u8 = 255;
pub const CHARSET_LOWER: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
pub const CHARSET_UPPER: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
pub const CHARSET_DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
pub const CHARSET_SPECIAL: [char; 33] = [' ', '!', '\"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~'];
pub const CHARSET_ALL: [char; 95] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ' ', '!', '\"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~'];

pub fn recurse(v: &Vec<Vec<char>>, list_to_twiddle: usize, current_permutation: &mut Vec<char>) {
    // See: https://stackoverflow.com/questions/3536833/arbitrary-number-of-nested-loops
    if list_to_twiddle == v.len() {
        for i in current_permutation {
            print!("{}", i);
        }
        println!();
        return;
    }

    for i in 0..v[list_to_twiddle].len() {
        current_permutation.push(v[list_to_twiddle][i]);
        recurse(v, list_to_twiddle + 1, current_permutation);
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

pub fn mp_get_compinations() {
    todo!();
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

#[cfg(test)]
mod tests {
    use super::*;
}
