pub struct Fraction(i64);

impl Fraction {
    pub fn add(self, other: &Fraction) -> Fraction {
        if self.0 > 0 {
            return Fraction(self.0 + other.0);
        }
        Fraction(other.0)
    }

    pub fn to_int(&self) -> i64 {
        self.0
    }
}

#[test]
fn add_zero_to_zero() {
    let result = Fraction(0).add(&Fraction(0));
    assert_eq!(0, result.to_int());
}

#[test]
fn add_non_zero_to_zero() {
    let result = Fraction(0).add(&Fraction(3));
    assert_eq!(3, result.to_int())
}

#[test]
fn add_zero_to_non_zero() {
    let result = Fraction(5).add(&Fraction(0));
    assert_eq!(5, result.to_int())
}

#[test]
fn non_zero_integers() {
    let result = Fraction(3).add(&Fraction(4));
    assert_eq!(7, result.to_int())
}
