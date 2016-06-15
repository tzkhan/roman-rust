extern crate roman_rust;

use roman_rust::*;

fn main(){
    println!("1234 in roman is {:?}", to_roman(1234));
    println!("XCIX in arabic is {:?}", to_arabic("XCIX"));
}