/// Converts a number to its corresponding alphabetic representation.
///
/// This function takes a positive integer `num` and converts it to a string representing
/// the "alphabetic" value (like Excel column naming). The result is case-sensitive, using uppercase letters.
///
/// # Arguments
/// * `num`: A positive integer representing the number to convert.
///
/// # Returns
/// A `String` representing the alphabetic value corresponding to `num`.
///
/// # Example
/// `1 -> A`, `26 -> Z`, `27 -> AA`
pub fn to_alphabet(mut num: usize) -> String {
    let mut result = String::new();

    while num > 0 {
        num -= 1; // Make it zero-based for modulo operation
        let remainder = (num % 26) as u8;
        result.insert(0, (b'A' + remainder) as char);
        num /= 26;
    }

    result
}