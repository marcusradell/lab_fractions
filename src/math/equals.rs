#[cfg(test)]
mod tests {
    use crate::math::*;

    #[test]
    fn same_numerator_and_denominator() {
        assert_eq!(Fraction::new(3, 5), Fraction::new(3, 5));
    }
}
