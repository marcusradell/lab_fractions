mod equals_test;
mod fractions_test;
mod gcd;
mod reduce_test;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Fraction {
    numerator: i64,
    denominator: i64,
}

#[allow(dead_code)]
impl Fraction {
    pub fn new(numerator: i64, denominator: i64) -> Fraction {
        let gcd = gcd::gcd(numerator, denominator);
        Fraction {
            numerator: numerator / gcd,
            denominator: denominator / gcd,
        }
    }

    pub fn from_int(numerator: i64) -> Fraction {
        Fraction::new(numerator, 1)
    }

    pub fn add(self, other: Fraction) -> Fraction {
        if self.denominator != other.denominator {
            return Fraction::new(
                self.numerator * other.denominator + other.numerator * self.denominator,
                self.denominator * other.denominator,
            );
        }

        Fraction {
            numerator: self.numerator + other.numerator,
            denominator: self.denominator,
        }
    }
}
