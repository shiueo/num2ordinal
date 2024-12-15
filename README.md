# num2ordinal
A simple Rust crate for converting integers to ordinal representations, including support for alphabetic and Roman numeral formats.

## Features

- **Alphabetic Representation**: Converts numbers to a base-26 alphabetic representation (e.g., 1 -> A, 27 -> AA).
- **Roman Numerals**: Converts numbers to their standard Roman numeral representation (up to 1000 using standard Roman numerals).
- **Extended Roman Numerals**: Converts numbers to extended Roman numerals, supporting values up to 10000 (using extended Roman numerals).

## Installation
Add this to your `Cargo.toml`:
```toml
[dependencies]
num2ordinal = "0.1"
```

## Usage
### Alphabetic Representation
The `to_alphabet` function converts an integer to its corresponding alphabetic representation, following the Excel-style column naming convention.
```rust
use num2ordinal::alphabet::to_alphabet;

fn main() {
    let num = 1999;
    let alphabet = to_alphabet(num);
    println!("Alphabetic representation for {} is {}", num, alphabet);
    // Alphabetic representation for 1999 is BXW
}
```
### Roman Numerals
```rust
use num2ordinal::roman::to_roman;

fn main() {
    let num = 1999;
    let roman = to_roman(num);
    println!("Roman numeral for {} is {}", num, roman);
    // Roman numeral for 1999 is MCMXCIX
}
```
### Extended Roman Numerals
```rust
use num2ordinal::roman::to_roman;

fn main() {
    let num = 14000;
    let extended_roman = to_extended_roman(num);
    println!("Extended Roman numeral for {} is {}", num, extended_roman);
    // Extended Roman numeral for 14000 is X̄ĪV̄
}
```
## License
This crate is licensed under the MIT license. See LICENSE for more details.