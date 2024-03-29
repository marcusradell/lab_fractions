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
        let sign_of_denominator = if denominator < 0 { -1 } else { 1 };
        let gcd = gcd::gcd(numerator, denominator) * sign_of_denominator;
        Fraction {
            numerator: numerator / gcd,
            denominator: denominator / gcd,
        }
    }

    pub fn from_int(numerator: i64) -> Fraction {
        Fraction::new(numerator, 1)
    }

    pub fn add(self, other: Fraction) -> Fraction {
        Fraction::new(
            self.numerator * other.denominator + other.numerator * self.denominator,
            self.denominator * other.denominator,
        )
    }
}
