pub use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla, // zero
    I,     // 1
    V,     // 5
    X,     // 10
    L,     // 50
    C,     // 100
    D,     // 500
    M,     // 1000
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>, pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
struct RomanValue {
    value: u32,
    digit1: RomanDigit,
    digit2: Option<RomanDigit>,
}

impl From<u32> for RomanDigit {
    fn from(num: u32) -> Self {
        match num {
            1 => RomanDigit::I,
            5 => RomanDigit::V,
            10 => RomanDigit::X,
            50 => RomanDigit::L,
            100 => RomanDigit::C,
            500 => RomanDigit::D,
            1000 => RomanDigit::M,
            _ => RomanDigit::Nulla,
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        let initial_num = num;

        let mut digits = Vec::new();

        let values = [
            RomanValue { value: 1000, digit1: RomanDigit::M, digit2: None },
            RomanValue { value: 900, digit1: RomanDigit::C, digit2: Some(RomanDigit::M) },
            RomanValue { value: 500, digit1: RomanDigit::D, digit2: None },
            RomanValue { value: 400, digit1: RomanDigit::C, digit2: Some(RomanDigit::D) },
            RomanValue { value: 100, digit1: RomanDigit::C, digit2: None },
            RomanValue { value: 90, digit1: RomanDigit::X, digit2: Some(RomanDigit::C) },
            RomanValue { value: 50, digit1: RomanDigit::L, digit2: None },
            RomanValue { value: 40, digit1: RomanDigit::X, digit2: Some(RomanDigit::L) },
            RomanValue { value: 10, digit1: RomanDigit::X, digit2: None },
            RomanValue { value: 9, digit1: RomanDigit::I, digit2: Some(RomanDigit::X) },
            RomanValue { value: 5, digit1: RomanDigit::V, digit2: None },
            RomanValue { value: 4, digit1: RomanDigit::I, digit2: Some(RomanDigit::V) },
            RomanValue { value: 1, digit1: RomanDigit::I, digit2: None },
        ];

        for value in values.iter() {
            while num >= value.value {
                digits.push(value.digit1);

                if let Some(digit2) = value.digit2 {
                    digits.push(digit2);
                }

                num -= value.value;
            }
        }

        RomanNumber(digits, initial_num)
    }
}

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        self.1 += 1;

        let new_digits = RomanNumber::from(self.1).0;
        self.0 = new_digits.clone();

        Some(RomanNumber(new_digits, self.1))
    }
}