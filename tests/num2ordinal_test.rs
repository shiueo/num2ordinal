use num2ordinal::alphabet::to_alphabet;
use num2ordinal::extended_roman::to_extended_roman;
use num2ordinal::roman::to_roman;

#[test]
fn num_to_alphabet() {
    assert_eq!(to_alphabet(1), "A");
    assert_eq!(to_alphabet(26), "Z");
    assert_eq!(to_alphabet(27), "AA");
    assert_eq!(to_alphabet(28), "AB");
    assert_eq!(to_alphabet(52), "AZ");
    assert_eq!(to_alphabet(53), "BA");
    assert_eq!(to_alphabet(702), "ZZ");
    assert_eq!(to_alphabet(703), "AAA");
    assert_eq!(to_alphabet(18278), "ZZZ");
    assert_eq!(to_alphabet(18279), "AAAA");
    assert_eq!(to_alphabet(475254), "ZZZZ");
    assert_eq!(to_alphabet(475255), "AAAAA");
}

#[test]
fn num_to_roman() {
    assert_eq!(to_roman(1), "I");
    assert_eq!(to_roman(4), "IV");
    assert_eq!(to_roman(9), "IX");
    assert_eq!(to_roman(58), "LVIII");
    assert_eq!(to_roman(1999), "MCMXCIX");
    assert_eq!(to_roman(14000), "MMMMMMMMMMMMMM");
}

#[test]
fn num_to_extended_roman() {
    assert_eq!(to_extended_roman(1), "I");
    assert_eq!(to_extended_roman(4), "IV");
    assert_eq!(to_extended_roman(9), "IX");
    assert_eq!(to_extended_roman(58), "LVIII");
    assert_eq!(to_extended_roman(1999), "MCMXCIX");
    assert_eq!(to_extended_roman(14000), "X̄ĪV̄");
}
