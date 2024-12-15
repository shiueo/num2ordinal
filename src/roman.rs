use once_cell::sync::Lazy;

static ROMAN_NUMERALS: Lazy<Vec<(usize, &str)>> = Lazy::new(|| {
    vec![
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ]
});

/// Converts a number to its corresponding Roman numeral representation.
///
/// This function takes a positive integer `num` and converts it into a Roman numeral
/// using the standard Roman numeral symbols for values up to 1000 (i.e., the numeral "M" is used for 1000).
///
/// # Arguments
/// * `num`: A positive integer representing the number to convert.
///
/// # Returns
/// A `String` representing the Roman numeral corresponding to `num`.
///
/// # Example
/// `1 -> I`, `4 -> IV`, `1999 -> MCMXCIX`
pub fn to_roman(mut num: usize) -> String {
    let mut result = String::new();
    let mut index = 0;

    while num > 0 {
        let (value, symbol) = ROMAN_NUMERALS[index];
        if num >= value {
            result.push_str(symbol);
            num -= value;
        } else {
            index += 1;
        }
    }

    result
}
