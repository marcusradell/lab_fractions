#[cfg(test)]
mod tests {
    use crate::math::Fraction;

    #[test]
    fn already_in_lowest_terms() {
        assert_eq!(Fraction::new(3, 4), Fraction::new(3, 4));
    }

    #[test]
    fn reduce_to_not_whole_numbers() {
        assert_eq!(Fraction::new(3, 4), Fraction::new(6, 8));
    }

    #[test]
    fn reduce_to_whole_numbers() {
        assert_eq!(Fraction::from_int(6), Fraction::new(24, 4))
    }

    #[test]
    fn reduce_zero() {
        assert_eq!(Fraction::from_int(0), Fraction::new(0, 24654));
    }
}
