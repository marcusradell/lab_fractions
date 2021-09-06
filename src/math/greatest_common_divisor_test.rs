fn gcd(a: i64, b: i64) -> i64 {
    let mut bb = b;
    let mut aa = a;
    while bb != 0 {
        let t = bb;
        bb = aa % t;
        aa = t;
    }

    aa
}

mod tests {
    use super::*;

    #[test]
    fn one_and_one() {
        assert_eq!(1, gcd(1, 1));
    }
}
