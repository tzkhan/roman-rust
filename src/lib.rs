static LOOKUP: [(usize, &'static str); 13] = [(1000, "M"), (900, "CM"), (500, "D"), (400, "CD"),
                                              (100, "C"), (90, "XC"), (50, "L"), (40, "XL"),
                                              (10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I")];

pub fn to_arabic(roman: &str) -> Option<usize> {

    let iter = roman
              .chars()
              .map(roman_char_to_arabic_digit);

    let mut num = 0;
    let mut prev = 0;

    for d in iter {

        if d <= 0 {
            return None;
        }

        num += d;

        if prev != 0 && d > prev {
            num -= prev * 2;
        }

        prev = d;
    }

    Some(num)
}

fn roman_char_to_arabic_digit(roman: char) -> usize {

    let mut i = 0;

    for &(n, s) in &LOOKUP {
        if i % 2 == 0 && s.chars().nth(0) == roman.to_uppercase().next() {
            return n;
        }
        i+=1;
    }

    0
}

pub fn to_roman(number: usize) -> Option<String> {

    if number < 1 || number >= 5000 {
        return None;
    }

    let mut roman = String::new();
    let mut num = number;

    while num > 0 {
        for &(n, s) in &LOOKUP {
            if n <= num {
                roman.push_str(s);
                num-= n;
                break;
            }
        }
    }
    
    Some(roman)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_arabic() {
        assert_eq!(to_arabic("CliiI"), Some(153));
        assert_eq!(to_arabic("XxXiV"), Some(34));
        assert_eq!(to_arabic("CM"), Some(900));
        assert_eq!(to_arabic("ABC0"), None);
        assert_eq!(to_arabic("vii"), Some(7));
        assert_eq!(to_arabic("IC"), Some(99));
        assert_eq!(to_arabic("XCIX"), Some(99));
        assert_eq!(to_arabic("XA"), None);
        assert_eq!(to_arabic("abc"), None);
    }

    #[test]
    fn test_to_roman() {
        assert_eq!(to_roman(5), Some("V".to_string()));
        assert_eq!(to_roman(101), Some("CI".to_string()));
        assert_eq!(to_roman(99), Some("XCIX".to_string()));
        assert_eq!(to_roman(4999), Some("MMMMCMXCIX".to_string()));
        assert_eq!(to_roman(5000), None);
        assert_eq!(to_roman(5001), None);
        assert_eq!(to_roman(10000), None);
        assert_eq!(to_roman(0), None);
    }

}