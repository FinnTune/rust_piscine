#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

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
        if num == 0 {
            return RomanNumber(vec![RomanDigit::Nulla]);
        }

        let mut digits = Vec::new();

        while num > 0 {
            if num >= 1000 {
                digits.push(RomanDigit::M);
                num -= 1000;
            } else if num >= 900 {
                digits.push(RomanDigit::C);
                digits.push(RomanDigit::M);
                num -= 900;
            } else if num >= 500 {
                digits.push(RomanDigit::D);
                num -= 500;
            } else if num >= 400 {
                digits.push(RomanDigit::C);
                digits.push(RomanDigit::D);
                num -= 400;
            } else if num >= 100 {
                digits.push(RomanDigit::C);
                num -= 100;
            } else if num >= 90 {
                digits.push(RomanDigit::X);
                digits.push(RomanDigit::C);
                num -= 90;
            } else if num >= 50 {
                digits.push(RomanDigit::L);
                num -= 50;
            } else if num >= 40 {
                digits.push(RomanDigit::X);
                digits.push(RomanDigit::L);
                num -= 40;
            } else if num >= 10 {
                digits.push(RomanDigit::X);
                num -= 10;
            } else if num == 9 {
                digits.push(RomanDigit::I);
                digits.push(RomanDigit::X);
                num -= 9;
            } else if num >= 5 {
                digits.push(RomanDigit::V);
                num -= 5;
            } else if num == 4 {
                digits.push(RomanDigit::I);
                digits.push(RomanDigit::V);
                num -= 4;
            } else {
                digits.push(RomanDigit::I);
                num -= 1;
            }
        }

        RomanNumber(digits)
    }
}

