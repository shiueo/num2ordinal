use num2ordinal::alphabet::to_alphabet;
use num2ordinal::extended_roman::to_extended_roman;
use num2ordinal::roman::to_roman;

fn main() {
    let num = 1999;
    let alphabet = to_alphabet(num);
    println!("Alphabetic representation for {} is {}", num, alphabet);
    // Alphabetic representation for 1999 is BXW

    let num = 1999;
    let roman = to_roman(num);
    println!("Roman numeral for {} is {}", num, roman);
    // Roman numeral for 1999 is MCMXCIX

    let num = 14000;
    let extended_roman = to_extended_roman(num);
    println!("Extended Roman numeral for {} is {}", num, extended_roman);
    // Extended Roman numeral for 14000 is X̄ĪV̄
}