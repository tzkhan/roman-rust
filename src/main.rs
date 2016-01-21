/*
** A simple program that allows basic arithmetic operations using roman numerals and conversions to decimal numbers.
*/

fn main() {

	let roman_number = "CLIII";
	let decimal_number = to_decimal(&roman_number);

	println!("The roman number {} is equal to {} in decimal numbers", roman_number, decimal_number);

	let roman_number = "XXXIV";
	let decimal_number = to_decimal(&roman_number);

	println!("The roman number {} is equal to {} in decimal numbers", roman_number, decimal_number);
}

fn to_decimal(roman_number: &str) -> i32 {

	let mut num = 0;
	let mut prev = 0;

	for c in roman_number.chars() {

		let digit = roman_char_to_decimal(c);

		num += digit;

		if prev != 0 && digit > prev {
			num -= 2 * prev;
		}

		prev = digit;
	}

	num
}

fn roman_char_to_decimal(roman_char: char) -> i32 {

	match roman_char {
	    'I' => 1,
	    'V' => 5,
	    'X' => 10,
	    'L' => 50,
	    'C' => 100,
	    'D' => 500,
	    'M' => 1000,
	    _ => 0,
	}
}