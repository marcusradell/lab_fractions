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
}
