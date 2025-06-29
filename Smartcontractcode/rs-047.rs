use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct C<T> {
    // real
    pub re: T,
    // imaginary
    pub im: T,
}

impl<T> Add for C<T> 
where
    T: Add<Output = T>,
{
    type Output = C<T>;

    fn add(self, rhs: C<T>) -> Self::Output {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<T> Mul for C<T> 
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
{
    type Output = C<T>;

    fn mul(self, rhs: C<T>) -> Self::Output {
        let a = self.re;
        let b = self.im;
        let c = rhs.re;
        let d = rhs.im;
        C {
            re: a * c - b * d,
            im: b * c + a * d,
        }
    }
}