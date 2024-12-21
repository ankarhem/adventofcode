pub fn extended_gcd<Number>(a: Number, b: Number) -> (Number, Number, Number)
where
    Number: Into<i64>,
{
    let a = a.into();
    let b = b.into();

    if b == 0 {
        (a, 1, 0)
    } else {
        let (d, x, y) = extended_gcd(b, a % b);
        (d, y, x - (a / b) * y)
    }
}

pub fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}