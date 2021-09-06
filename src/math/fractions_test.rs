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

    #[test]
    fn reduce_result_to_whole_number() {
        assert_eq!(
            Fraction::from_int(1),
            Fraction::new(1, 3).add(Fraction::new(2, 3))
        );
    }

    #[test]

    fn denominator_multiple_of_other() {
        assert_eq!(
            Fraction::new(11, 8),
            Fraction::new(3, 4).add(Fraction::new(5, 8))
        )
    }

    #[test]
    fn common_factor_in_denominators() {
        assert_eq!(
            Fraction::new(11, 18),
            Fraction::new(1, 6).add(Fraction::new(4, 9))
        )
    }

    #[test]
    fn reduce_result_when_denominators_are_equal() {
        assert_eq!(
            Fraction::new(3, 2),
            Fraction::new(3, 4).add(Fraction::new(3, 4))
        )
    }

    #[test]
    fn negative_fraction_and_reducing() {
        assert_eq!(
            Fraction::new(1, 2),
            Fraction::new(-1, 4).add(Fraction::new(3, 4))
        );
        assert_eq!(
            Fraction::new(-1, 8),
            Fraction::new(3, 8).add(Fraction::new(-1, 2))
        );
        assert_eq!(
            Fraction::new(1, 2),
            Fraction::new(1, -4).add(Fraction::new(-3, -4))
        );
    }
}
