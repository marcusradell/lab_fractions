#[allow(dead_code)]
struct Fraction {
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

    pub fn to_i64(&self) -> i64 {
        self.numerator
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_zero_to_zero() {
        let result = Fraction::from_int(0).add(Fraction::from_int(0)).to_i64();
        assert_eq!(0, result);
    }

    #[test]
    fn add_non_zero_to_zero() {
        let result = Fraction::from_int(0).add(Fraction::from_int(3)).to_i64();
        assert_eq!(3, result)
    }

    #[test]
    fn add_zero_to_non_zero() {
        let result = Fraction::from_int(5).add(Fraction::from_int(0)).to_i64();
        assert_eq!(5, result)
    }

    #[test]
    fn non_zero_integers() {
        let result = Fraction::from_int(3).add(Fraction::from_int(4)).to_i64();
        assert_eq!(7, result)
    }

    #[test]
    fn non_trivial_denominator() {
        let result = Fraction::new(1, 5).add(Fraction::new(2, 5));
        assert_eq!(3, result.numerator);
        assert_eq!(5, result.denominator);
    }
}
