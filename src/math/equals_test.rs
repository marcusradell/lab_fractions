#[cfg(test)]
mod tests {
    use crate::math::Fraction;

    #[test]
    fn same_numerator_and_denominator() {
        assert_eq!(Fraction::new(3, 5), Fraction::new(3, 5));
    }

    #[test]
    fn different_numerators() {
        assert_ne!(Fraction::new(1, 5), Fraction::new(1, 3));
    }

    #[test]
    fn different_denominators() {
        assert_ne!(Fraction::new(5, 1), Fraction::new(1, 7));
    }

    #[test]
    fn whole_number_equals_same_fraction() {
        assert_eq!(Fraction::new(5, 1), Fraction::from_int(5));
    }

    #[test]
    fn whole_numbers_not_equal() {
        assert_ne!(Fraction::from_int(6), Fraction::from_int(5));
    }

    #[test]
    fn negative_denominator() {
        assert_eq!(Fraction::new(1, 2), Fraction::new(-1, -2));
        assert_eq!(Fraction::new(-1, 2), Fraction::new(1, -2));
    }
}
