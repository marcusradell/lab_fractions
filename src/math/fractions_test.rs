#[cfg(test)]
mod tests {
    use crate::math::Fraction;

    #[test]
    fn add_zero_to_zero() {
        let expected = Fraction::from_int(0);
        let result = Fraction::from_int(0).add(Fraction::from_int(0));
        assert_eq!(expected, result);
    }

    #[test]
    fn add_non_zero_to_zero() {
        let expected = Fraction::from_int(3);
        let result = Fraction::from_int(0).add(Fraction::from_int(3));
        assert_eq!(expected, result);
    }

    #[test]
    fn add_zero_to_non_zero() {
        let expected = Fraction::from_int(5);
        let result = Fraction::from_int(5).add(Fraction::from_int(0));
        assert_eq!(expected, result);
    }

    #[test]
    fn non_zero_integers() {
        let expected = Fraction::from_int(7);
        let result = Fraction::from_int(3).add(Fraction::from_int(4));
        assert_eq!(expected, result);
    }

    #[test]
    fn non_trivial_denominator() {
        let expected = Fraction::new(3, 5);
        let result = Fraction::new(1, 5).add(Fraction::new(2, 5));
        assert_eq!(expected, result);
    }

    #[test]
    fn different_denominators_no_reduction() {
        let expected = Fraction::new(5, 6);
        let result = Fraction::new(1, 2).add(Fraction::new(1, 3));
        assert_eq!(expected, result);
    }
}
