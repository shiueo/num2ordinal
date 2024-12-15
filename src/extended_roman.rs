use once_cell::sync::Lazy;

static EXTENDED_ROMAN_NUMERALS: Lazy<Vec<(usize, &str)>> = Lazy::new(|| {
    vec![
        (10000, "X̄"),
        (9000, "ĪX̄"),
        (5000, "V̄"),
        (4000, "ĪV̄"),
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

/// Converts a number to its corresponding Extended Roman numeral representation.
///
/// This function takes a positive integer `num` and converts it into an Extended Roman numeral
/// using the standard Roman numeral symbols for values up to 10000 (i.e., the numeral "X̄" is used for 10000).
///
/// # Arguments
/// * `num`: A positive integer representing the number to convert.
///
/// # Returns
/// A `String` representing the Extended Roman numeral corresponding to `num`.
///
/// # Example
/// `1 -> I`, `4 -> IV`, `1999 -> MCMXCIX`, `14000 -> X̄ĪV̄`
pub fn to_extended_roman(mut num: usize) -> String {
    let mut result = String::new();
    let mut index = 0;

    while num > 0 {
        let (value, symbol) = EXTENDED_ROMAN_NUMERALS[index];
        if num >= value {
            result.push_str(symbol);
            num -= value;
        } else {
            index += 1;
        }
    }

    result
}
