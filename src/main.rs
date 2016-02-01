/*
** A simple program that allows basic arithmetic operations using roman numerals and conversions to decimal numbers.
*/

fn main() {

	let roman_numbers = vec!["CliiI", "XxXiV", "CM", "ABC0", "vii"];

	for roman_number in roman_numbers {
		println!("The roman number {} is equal to {} in decimal.", roman_number, to_decimal(roman_number));
	}
}

fn to_decimal(roman_number: &str) -> u64 {

	let mut num = 0;
	let mut prev = 0;

	for c in roman_number.chars() {

		let digit = roman_char_to_decimal(c);

		if digit <= 0 { break; }

		num += digit;

		if prev != 0 && digit > prev {
			num -= 2 * prev;
		}

		prev = digit;
	}

	num
}

fn roman_char_to_decimal(roman_char: char) -> u64 {

	match roman_char.to_uppercase().next() {
	    Some('I') => 1,
	    Some('V') => 5,
	    Some('X') => 10,
	    Some('L') => 50,
	    Some('C') => 100,
	    Some('D') => 500,
	    Some('M') => 1000,
	    _ => 0,
	}
}
