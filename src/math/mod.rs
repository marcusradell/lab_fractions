mod equals_test;
mod fractions_test;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Fraction {
    numerator: i64,
    denominator: i64,
}

#[allow(dead_code)]
impl Fraction {
    pub fn new(numerator: i64, denominator: i64) -> Fraction {
        Fraction {
            numerator,
            denominator,
        }
    }

    pub fn from_int(numerator: i64) -> Fraction {
        Fraction::new(numerator, 1)
    }

    pub fn add(self, other: Fraction) -> Fraction {
        Fraction {
            numerator: self.numerator + other.numerator,
            denominator: self.denominator,
        }
    }

    pub fn get_numerator(&self) -> i64 {
        self.numerator
    }

    pub fn get_denominator(&self) -> i64 {
        self.denominator
    }
}
