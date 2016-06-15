[![Build Status](https://travis-ci.org/tzkhan/roman-rust.svg?branch=master)](https://travis-ci.org/tzkhan/roman-rust)

# roman_rust
###Conversions from and to roman numerals in rust lang.

A simple program that allows conversions from roman to arabic numerals and vice versa.

List of functions available:
```rust
to_arabic(roman: &str) -> Option<usize>
to_roman(number: usize) -> Option<String>
```

Also handles validation of roman numerals.