/// # num2ordinal
/// A simple Rust crate for converting integers to ordinal representations, including support for alphabetic and Roman numeral formats.
///
/// ## Features
/// - **Alphabetic Representation**: Converts numbers to a base-26 alphabetic representation (e.g., 1 -> A, 27 -> AA).
/// - **Roman Numerals**: Converts numbers to their standard Roman numeral representation (up to 1000 using standard Roman numerals).
/// - **Extended Roman Numerals**: Converts numbers to extended Roman numerals, supporting values up to 10000 (using extended Roman numerals).
pub mod alphabet;
pub mod roman;
pub mod extended_roman;
