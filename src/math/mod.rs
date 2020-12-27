pub struct Fraction {
    numerator: i64,
    denominator: i64,
}

impl Fraction {
    pub fn new(numerator: i64) -> Fraction {
        Fraction {
            numerator,
            denominator: 1,
        }
    }

    pub fn add(self, other: Fraction) -> Fraction {
        Fraction {
            numerator: self.numerator + other.numerator,
            denominator: self.denominator,
        }
    }

    pub fn to_int(&self) -> i64 {
        self.numerator
    }
}

mod tests {
    use super::*;

    #[test]
    fn add_zero_to_zero() {
        let result = Fraction::new(0).add(Fraction::new(0)).to_int();
        assert_eq!(0, result);
    }

    #[test]
    fn add_non_zero_to_zero() {
        let result = Fraction::new(0).add(Fraction::new(3)).to_int();
        assert_eq!(3, result)
    }

    #[test]
    fn add_zero_to_non_zero() {
        let result = Fraction::new(5).add(Fraction::new(0)).to_int();
        assert_eq!(5, result)
    }

    #[test]
    fn non_zero_integers() {
        let result = Fraction::new(3).add(Fraction::new(4)).to_int();
        assert_eq!(7, result)
    }

    #[test]
    fn non_trivial_denominator() {
        let result = Fraction {
            numerator: 1,
            denominator: 5,
        }
        .add(Fraction {
            numerator: 2,
            denominator: 5,
        });
        assert_eq!(3, result.numerator);
        assert_eq!(5, result.denominator);
    }
}
