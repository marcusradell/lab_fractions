fn gcd(a: i64, b: i64) -> i64 {
    let mut b = b;
    let mut a = a;
    while b != 0 {
        let t = b;
        b = a % t;
        a = t;
    }

    a.abs()
}

mod tests {
    use super::*;

    #[test]
    fn reflexive() {
        assert_eq!(1, gcd(1, 1));
        assert_eq!(2, gcd(2, 2));
        assert_eq!(1, gcd(-1, -1));
    }

    #[test]
    fn relatively_prime() {
        assert_eq!(1, gcd(2, 3));
        assert_eq!(1, gcd(4, 7));
        assert_eq!(1, gcd(-2, -3));
    }

    #[test]
    fn one_is_multiple_of_the_other() {
        assert_eq!(3, gcd(3, 9));
        assert_eq!(5, gcd(5, 30));
    }

    #[test]
    fn common_factor() {
        assert_eq!(2, gcd(6, 8));
        assert_eq!(7, gcd(49, 315));
        assert_eq!(4, gcd(-24, -28));
    }

    #[test]
    fn negatives() {
        // GCD should always be positive.
        assert_eq!(4, gcd(-24, 28));
        assert_eq!(4, gcd(24, -28));
    }
}
