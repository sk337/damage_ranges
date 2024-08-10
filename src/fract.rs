use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fract {
    pub num: u32,
    pub den: u32,
}

impl Fract {
    pub fn new(num: u32, den: u32) -> Fract {
        Fract { num, den }
    }

    pub fn add(&self, other: &Fract) -> Fract {
        Fract {
            num: self.num * other.den + other.num * self.den,
            den: self.den * other.den,
        }
    }

    pub fn add_uint(&self, other: u32) -> Fract {
        Fract {
            num: self.num + other * self.den,
            den: self.den,
        }
    }

    pub fn sub(&self, other: &Fract) -> Fract {
        Fract {
            num: self.num * other.den - other.num * self.den,
            den: self.den * other.den,
        }
    }

    pub fn sub_uint(&self, other: u32) -> Fract {
        Fract {
            num: self.num - other * self.den,
            den: self.den,
        }
    }

    pub fn mul(&self, other: &Fract) -> Fract {
        Fract {
            num: self.num * other.num,
            den: self.den * other.den,
        }
    }

    pub fn mul_uint(&self, other: u32) -> Fract {
        Fract {
            num: self.num * other,
            den: self.den,
        }
    }

    pub fn div(&self, other: &Fract) -> Fract {
        Fract {
            num: self.num * other.den,
            den: self.den * other.num,
        }
    }

    pub fn div_uint(&self, other: u32) -> Fract {
        Fract {
            num: self.num,
            den: self.den * other,
        }
    }

    pub fn print(&self) {
        println!("{}/{}", self.num, self.den);
    }

    pub fn to_float(&self) -> f64 {
        self.num as f64 / self.den as f64
    }

    pub fn from_u32(num: u32) -> Fract {
        Fract { num, den: 1 }
    }
}

impl Add for Fract {
    type Output = Fract;

    fn add(self, other: Fract) -> Fract {
        Fract {
            num: self.num * other.den + other.num * self.den,
            den: self.den * other.den,
        }
    }
}

impl From<u32> for Fract {
    fn from(value: u32) -> Self {
        Fract { num: value, den: 1 }
    }
}

impl Add<u32> for Fract {
    type Output = Fract;

    fn add(self, other: u32) -> Fract {
        Fract {
            num: self.num + other as u32 * self.den,
            den: self.den,
        }
    }
}

impl Sub for Fract {
    type Output = Fract;

    fn sub(self, other: Fract) -> Fract {
        Fract {
            num: self.num * other.den - other.num * self.den,
            den: self.den * other.den,
        }
    }
}

impl Sub<u32> for Fract {
    type Output = Fract;

    fn sub(self, other: u32) -> Fract {
        Fract {
            num: self.num - other * self.den,
            den: self.den,
        }
    }
}

impl Mul for Fract {
    type Output = Fract;

    fn mul(self, other: Fract) -> Fract {
        Fract {
            num: self.num * other.num,
            den: self.den * other.den,
        }
    }
}

impl Mul<u32> for Fract {
    type Output = Fract;

    fn mul(self, other: u32) -> Fract {
        Fract {
            num: self.num,
            den: self.den,
        }
    }
}

impl Div for Fract {
    type Output = Fract;

    fn div(self, other: Fract) -> Fract {
        Fract {
            num: self.num * other.den,
            den: self.den * other.num,
        }
    }
}

impl Div<u32> for Fract {
    type Output = Fract;

    fn div(self, other: u32) -> Fract {
        Fract {
            num: self.num,
            den: self.den * other,
        }
    }
}

impl fmt::Display for Fract {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}
