fn gcd(a: i64, b: i64) -> i64 {
    let mut b = b;
    let mut a = a;
    while b != 0 {
        let t = b;
        b = a % t;
        a = t;
    }

    a
}

mod tests {
    use super::*;

    #[test]
    fn one_and_one() {
        assert_eq!(1, gcd(1, 1));
    }

    #[test]
    fn reflexive() {
        assert_eq!(1, gcd(1, 1));
        assert_eq!(2, gcd(2, 2));
        assert_eq!(-1, gcd(-1, -1));
    }
}
