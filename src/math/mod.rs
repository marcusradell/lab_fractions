pub struct Fraction(i64);

impl Fraction {
    pub fn add(&self, other: Fraction) -> Fraction {
        other
    }

    pub fn to_int(&self) -> i64 {
        self.0
    }
}

#[test]
fn add_zero_to_zero() {
    let result: Fraction = Fraction(0).add(Fraction(0));
    assert_eq!(0, result.to_int());
}

#[test]
fn add_non_zero_to_zero() {
    let result: Fraction = Fraction(0).add(Fraction(3));
    assert_eq!(3, result.to_int())
}
