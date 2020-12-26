pub struct Fraction(i64);

impl Fraction {
    pub fn add(&self, other: Fraction) -> Fraction {
        Fraction(0)
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
