use crate::gcd::extended_gcd;

#[derive(Debug)]
pub struct Diophantine {
    a: i64,
    b: i64,
    c: i64,
}

impl Diophantine {
    pub fn new<Number>(a: Number, b: Number, c: Number) -> Self
    where
        Number: Into<i64>,
    {
        Self {
            a: a.into(),
            b: b.into(),
            c: c.into(),
        }
    }

    pub fn solve(&self) -> Option<(i64, i64)> {
        let (d, x, y) = extended_gcd(self.a, self.b);

        if self.c % d == 0 {
            let x = x * (self.c / d);
            let y = y * (self.c / d);

            Some((x, y))
        } else {
            None
        }
    }
}