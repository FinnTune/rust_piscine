#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, Clone, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

use std::str::FromStr;
use std::fmt;

// Implementing FromStr for Antigen
impl FromStr for Antigen {
    type Err = fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "AB" => Ok(Antigen::AB),
            "B" => Ok(Antigen::B),
            "O" => Ok(Antigen::O),
            _ => Err(fmt::Error),
        }
    }
}

// Implementing FromStr for RhFactor
impl FromStr for RhFactor {
    type Err = fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err(fmt::Error),
        }
    }
}

// Implementing FromStr for BloodType
impl FromStr for BloodType {
    type Err = fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 || s.len() > 3 {
            return Err(fmt::Error);
        }

        let (antigen_str, rh_str) = s.split_at(s.len() - 1);
        let antigen = antigen_str.parse()?;
        let rh_factor = rh_str.parse()?;

        Ok(BloodType { antigen, rh_factor })
    }
}

use std::cmp::{Ord, Ordering};

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.antigen.cmp(&other.antigen) {
            Ordering::Equal => self.rh_factor.cmp(&other.rh_factor),
            order => order,
        }
    }
}

use std::fmt::Debug;

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rh_symbol = match &self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };
        write!(f, "{:?}{}", &self.antigen, rh_symbol)
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        let antigen_compatible = match (&self.antigen, &other.antigen) {
            (Antigen::A, Antigen::A) | (Antigen::A, Antigen::O) => true,
            (Antigen::B, Antigen::B) | (Antigen::B, Antigen::O) => true,
            (Antigen::AB, _) => true,
            (Antigen::O, Antigen::O) => true,
            _ => false,
        };

        let rh_compatible = match (&self.rh_factor, &other.rh_factor) {
            (RhFactor::Positive, _) => true,
            (RhFactor::Negative, RhFactor::Negative) => true,
            _ => false,
        };

        antigen_compatible && rh_compatible
    }

    pub fn donors(&self) -> Vec<Self> {
        let mut donors = vec![];
        let all_types = vec![
            BloodType { antigen: Antigen::A, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::A, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::B, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::B, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::O, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::O, rh_factor: RhFactor::Negative },
        ];

        for blood in all_types {
            if self.can_receive_from(&blood) {
                donors.push(blood);
            }
        }

        donors
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        let mut recipients = vec![];
        let all_types = vec![
            BloodType { antigen: Antigen::A, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::A, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::B, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::B, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::O, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::O, rh_factor: RhFactor::Negative },
        ];

        for blood in all_types {
            if blood.can_receive_from(self) {
                recipients.push(blood);
            }
        }

        recipients
    }
}

