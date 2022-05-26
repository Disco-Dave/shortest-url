use std::{convert::TryFrom, fmt::Display};

use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, ToPrimitive, FromPrimitive, PartialEq)]
#[rustfmt::skip]
pub enum Digit {
    Digit0, Digit1, Digit2, Digit3, Digit4, Digit5, Digit6, Digit7, Digit8,
    Digit9, Digita, Digitb, Digitc, Digitd, Digite, Digitf, Digitg, Digith,
    Digiti, Digitj, Digitk, Digitl, Digitm, Digitn, Digito, Digitp, Digitq,
    Digitr, Digits, Digitt, Digitu, Digitv, Digitw, Digitx, Digity, Digitz,
    DigitA, DigitB, DigitC, DigitD, DigitE, DigitF, DigitG, DigitH, DigitI,
    DigitJ, DigitK, DigitL, DigitM, DigitN, DigitO, DigitP, DigitQ, DigitR,
    DigitS, DigitT, DigitU, DigitV, DigitW, DigitX, DigitY, DigitZ,
}

impl Digit {
    fn from_u64(digit: u64) -> Option<Self> {
        num::FromPrimitive::from_u64(digit)
    }

    fn to_u64(&self) -> u64 {
        num::ToPrimitive::to_u64(self).unwrap_or(0)
    }

    #[rustfmt::skip]
    fn to_char(&self) -> char {
        use Digit::*;

        match self {
            Digit0 => '0', Digit1 => '1', Digit2 => '2', Digit3 => '3', Digit4 => '4',
            Digit5 => '5', Digit6 => '6', Digit7 => '7', Digit8 => '8', Digit9 => '9',
            Digita => 'a', Digitb => 'b', Digitc => 'c', Digitd => 'd', Digite => 'e',
            Digitf => 'f', Digitg => 'g', Digith => 'h', Digiti => 'i', Digitj => 'j',
            Digitk => 'k', Digitl => 'l', Digitm => 'm', Digitn => 'n', Digito => 'o',
            Digitp => 'p', Digitq => 'q', Digitr => 'r', Digits => 's', Digitt => 't',
            Digitu => 'u', Digitv => 'v', Digitw => 'w', Digitx => 'x', Digity => 'y',
            Digitz => 'z', DigitA => 'A', DigitB => 'B', DigitC => 'C', DigitD => 'D',
            DigitE => 'E', DigitF => 'F', DigitG => 'G', DigitH => 'H', DigitI => 'I',
            DigitJ => 'J', DigitK => 'K', DigitL => 'L', DigitM => 'M', DigitN => 'N',
            DigitO => 'O', DigitP => 'P', DigitQ => 'Q', DigitR => 'R', DigitS => 'S',
            DigitT => 'T', DigitU => 'U', DigitV => 'V', DigitW => 'W', DigitX => 'X',
            DigitY => 'Y', DigitZ => 'Z',
        }
    }

    #[rustfmt::skip]
    fn from_char(digit: char) -> Option<Self> {
        use Digit::*;

        match digit {
            '0' => Some(Digit0), '1' => Some(Digit1), '2' => Some(Digit2), '3' => Some(Digit3),
            '4' => Some(Digit4), '5' => Some(Digit5), '6' => Some(Digit6), '7' => Some(Digit7),
            '8' => Some(Digit8), '9' => Some(Digit9), 'a' => Some(Digita), 'b' => Some(Digitb),
            'c' => Some(Digitc), 'd' => Some(Digitd), 'e' => Some(Digite), 'f' => Some(Digitf),
            'g' => Some(Digitg), 'h' => Some(Digith), 'i' => Some(Digiti), 'j' => Some(Digitj),
            'k' => Some(Digitk), 'l' => Some(Digitl), 'm' => Some(Digitm), 'n' => Some(Digitn),
            'o' => Some(Digito), 'p' => Some(Digitp), 'q' => Some(Digitq), 'r' => Some(Digitr),
            's' => Some(Digits), 't' => Some(Digitt), 'u' => Some(Digitu), 'v' => Some(Digitv),
            'w' => Some(Digitw), 'x' => Some(Digitx), 'y' => Some(Digity), 'z' => Some(Digitz),
            'A' => Some(DigitA), 'B' => Some(DigitB), 'C' => Some(DigitC), 'D' => Some(DigitD),
            'E' => Some(DigitE), 'F' => Some(DigitF), 'G' => Some(DigitG), 'H' => Some(DigitH),
            'I' => Some(DigitI), 'J' => Some(DigitJ), 'K' => Some(DigitK), 'L' => Some(DigitL),
            'M' => Some(DigitM), 'N' => Some(DigitN), 'O' => Some(DigitO), 'P' => Some(DigitP),
            'Q' => Some(DigitQ), 'R' => Some(DigitR), 'S' => Some(DigitS), 'T' => Some(DigitT),
            'U' => Some(DigitU), 'V' => Some(DigitV), 'W' => Some(DigitW), 'X' => Some(DigitX),
            'Y' => Some(DigitY), 'Z' => Some(DigitZ),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Base62(pub Vec<Digit>);

impl Display for Base62 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string_of_digits = self.0.iter().map(Digit::to_char).collect::<String>();
        f.write_str(&string_of_digits)
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidChar(char),
    IsEmpty,
}

impl TryFrom<&str> for Base62 {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let trimmed_value = value.trim();

        if trimmed_value.is_empty() {
            Err(Error::IsEmpty)
        } else {
            let digits = value
                .chars()
                .map(|c| Digit::from_char(c).ok_or(Error::InvalidChar(c)))
                .collect::<Result<Vec<Digit>, Self::Error>>()?;

            Ok(Self(digits))
        }
    }
}

const BASE: u64 = 62;

impl From<Base62> for u64 {
    fn from(base62: Base62) -> Self {
        base62
            .0
            .iter()
            .enumerate()
            .map(|(index, digit)| Digit::to_u64(digit) * num::pow(BASE, index))
            .sum()
    }
}

impl From<u64> for Base62 {
    fn from(number: u64) -> Self {
        let mut n = number;
        let mut digits: Vec<Digit> = Vec::new();

        while n > 0 {
            if let Some(digit) = Digit::from_u64(n % BASE) {
                digits.push(digit);
            }
            n /= BASE;
        }

        Self(digits)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use super::*;

    #[test]
    fn can_convert_to_base62_and_back() {
        for n in 0u64..10_000 {
            let base62: Base62 = n.into();
            let back_to_n: u64 = base62.into();

            assert_eq!(n, back_to_n, "{}", n);
        }
    }

    #[test]
    fn can_convert_string_back_base62() {
        for n in 10_000u64..60_000 {
            let base62: Base62 = n.into();
            let base62_string: &str = &base62.to_string();
            let back_to_base62: Result<Base62, Error> = base62_string.try_into();

            assert_eq!(Ok(base62), back_to_base62, "{}", n);
        }
    }

    #[test]
    fn reports_invalid_digits() {
        let base62: Result<Base62, Error> = "adb!930".try_into();
        assert_eq!(Err(Error::InvalidChar('!')), base62);
    }

    #[test]
    fn complains_about_empty_strings() {
        let empties = ["", "  ", "\n"];
        for empty in empties {
            let base62: Result<Base62, Error> = empty.try_into();
            assert_eq!(Err(Error::IsEmpty), base62);
        }
    }
}

