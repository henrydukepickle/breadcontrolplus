use std::{
    fmt::Display,
    num::FpCategory,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

const HALF_SQRT: i128 = 0b10110101000001001111001100110011111110011101111001100100100001;
//x^2, x, 1
const LOG_QUAD_APPROX_COEFFS: (i128, i128, i128) = (
    HALF_SQRT - (1 << 62),
    ((4 * (1_i128 << 62)) + (-3 * (HALF_SQRT as i128))),
    ((-3 * (1_i128 << 62)) + (2 * (HALF_SQRT as i128))),
);

fn add(num: i64, num2: i64) -> (i64, i8) {
    resize(num as i128 + num2 as i128)
}
fn mul(num: i64, num2: i64) -> (i64, i8) {
    resize((num as i128 * num2 as i128) / (1 << 62))
}
fn inv(num: i64) -> (i64, i8) {
    resize((1 << 124) / (num as i128))
}
fn resize(num: i128) -> (i64, i8) {
    let m = num.abs().checked_ilog2();
    match m {
        Some(mag) => {
            let resized = if mag >= 62 {
                num / (1 << (mag - 62))
            } else {
                num * (1 << (62 - mag))
            };
            (resized as i64, (mag as i8) - 62)
        }
        None => (0, 0),
    }
}
fn mul_mant_without_resize(num1: i128, num2: i128) -> i128 {
    ((num1 as i128) * (num2 as i128)) / (1 << 62)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BigFloat {
    pub mant: i64,
    pub exp: i128,
}

impl BigFloat {
    pub fn from_float(float: f64) -> Self {
        match float.classify() {
            FpCategory::Normal => {
                let data = u64::from_le_bytes(float.to_le_bytes());
                let sign = (data >> 63) > 0;
                let exp = data % (1 << 63) >> 52;
                let mant = (1 << 62) + ((data % (1 << 52)) << 10);
                Self {
                    mant: if sign { -(mant as i64) } else { mant as i64 },
                    exp: (exp as i128) - 1023,
                }
            }
            FpCategory::Zero => Self { mant: 0, exp: 0 },
            _ => panic!("Pass a normal or zero float!"),
        }
    }
    pub fn from_int(num: i128) -> Self {
        match num.checked_ilog2() {
            Some(n) => Self {
                mant: resize(num).0,
                exp: n as i128,
            },
            None => Self { mant: 0, exp: 0 },
        }
    }
    pub fn pow2(pow: i128) -> Self {
        Self {
            mant: 1 << 62,
            exp: pow,
        }
    }
    pub fn inverse(self) -> Self {
        let (mant, overflow) = inv(self.mant);
        Self {
            mant,
            exp: -self.exp + overflow as i128,
        }
    }
    pub fn log2(self) -> Self {
        let exp = Self::from_int(self.exp);
        let x = self.mant as i128;
        let mant_part =
            mul_mant_without_resize(mul_mant_without_resize(x, x), LOG_QUAD_APPROX_COEFFS.0)
                + mul_mant_without_resize(x, LOG_QUAD_APPROX_COEFFS.1)
                + LOG_QUAD_APPROX_COEFFS.2;
        if mant_part == 0 {
            return exp;
        }
        let mag = mant_part.ilog2();
        let mant_float = Self {
            mant: (mant_part * (1 << (62 - mag))) as i64,
            exp: mag as i128 - 62,
        };
        exp + mant_float
    }
    pub fn exp2(self) -> Self {
        if self.mant == 0 {
            return Self {
                mant: 1 << 62,
                exp: 0,
            };
        } else if self.mant < 0 {
            return (-self).exp2().inverse();
        }
        if self.exp >= 63 {
            Self {
                exp: self.mant as i128 * (1 << (self.exp - 63)),
                mant: 1 << 62,
            }
        } else if self.exp >= 0 {
            let shift = 1 << (62 - self.exp);
            let exp = self.mant / shift;
            let x = ((self.mant % shift) * (1 << self.exp)) as i128;
            let leading_coeff = (1 << 62) - HALF_SQRT;
            let mant = (1 << 62)
                + mul_mant_without_resize(HALF_SQRT, x)
                + mul_mant_without_resize(leading_coeff, mul_mant_without_resize(x, x));
            Self {
                mant: mant as i64,
                exp: exp as i128,
            }
        } else if self.exp > -63 {
            let shift = 1 << (-self.exp);
            let x = (self.mant / shift) as i128;
            let leading_coeff = (1 << 62) - HALF_SQRT;
            let mant = (1 << 62)
                + mul_mant_without_resize(HALF_SQRT, x)
                + mul_mant_without_resize(leading_coeff, mul_mant_without_resize(x, x));
            Self {
                mant: mant as i64,
                exp: 0,
            }
        } else {
            Self {
                mant: 1 << 62,
                exp: 0,
            }
        }
    }
    pub fn pow(self, pow: Self) -> Self {
        (pow * self.log2()).exp2()
    }
    pub fn to_string_binary(&self) -> String {
        let mut str = String::from("1.");
        for i in 1..=10 {
            str += if (self.mant / (1 << (62 - i))) % 2 == 1 {
                "1"
            } else {
                "0"
            }
        }
        format!("{} * 2^{}", str, self.exp)
    }
}

impl Add for BigFloat {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.exp >= rhs.exp {
            let diff = self.exp - rhs.exp;
            if diff > 32 {
                return self;
            }
            let (mant, overflow) = add(self.mant, rhs.mant / (1 << diff));
            Self {
                mant,
                exp: self.exp + overflow as i128,
            }
        } else {
            let diff = rhs.exp - self.exp;
            if diff > 32 {
                return rhs;
            }
            let (mant, overflow) = add(self.mant / (1 << diff), rhs.mant);
            Self {
                mant,
                exp: rhs.exp + overflow as i128,
            }
        }
    }
}

impl Mul for BigFloat {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let (mant, overflow) = mul(self.mant, rhs.mant);
        Self {
            mant,
            exp: self.exp + rhs.exp + overflow as i128,
        }
    }
}

impl Neg for BigFloat {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            mant: -self.mant,
            exp: self.exp,
        }
    }
}

impl Sub for BigFloat {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Div for BigFloat {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inverse()
    }
}

impl PartialOrd for BigFloat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(match (self.mant.signum(), other.mant.signum()) {
            (0, 1) | (-1, 1) | (-1, 0) => std::cmp::Ordering::Less,
            (1, 0) | (1, -1) | (0, -1) => std::cmp::Ordering::Greater,
            (0, 0) => std::cmp::Ordering::Equal,
            (1, 1) => match self.exp.cmp(&other.exp) {
                std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                std::cmp::Ordering::Equal => self.mant.cmp(&other.mant),
                std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            },
            (-1, -1) => match other.exp.cmp(&self.exp) {
                std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                std::cmp::Ordering::Equal => other.mant.cmp(&self.mant),
                std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            },
            _ => unreachable!(),
        })
    }
}

impl Ord for BigFloat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Display for BigFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_binary())
    }
}

impl AddAssign for BigFloat {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl MulAssign for BigFloat {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl SubAssign for BigFloat {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl DivAssign for BigFloat {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}
