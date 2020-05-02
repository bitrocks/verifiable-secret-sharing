use num_bigint_dig::traits::ModInverse;
use num_bigint_dig::BigInt;
use num_bigint_dig::Sign::Plus;
use num_integer::Integer;
use rand::{thread_rng, Rng};
use secp256k1::constants::{CURVE_ORDER, GENERATOR_X, GENERATOR_Y, SECRET_KEY_SIZE};
use secp256k1::{PublicKey, Secp256k1, SecretKey, VerifyOnly};
use std::ops::{Add, Mul, Sub};
use std::sync::Once;
/// The `Secp256k1Scalar` is a scalar, wrapping the `SecretKey`
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Secp256k1Scalar(SecretKey);

/// The `Secp256k1Point` is a point in elliptic curve, wrapping the `PublicKey`
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Secp256k1Point(PublicKey);

impl Secp256k1Point {
    ///
    pub fn generator() -> Secp256k1Point {
        let mut v = vec![4 as u8];
        v.extend(GENERATOR_X.as_ref());
        v.extend(GENERATOR_Y.as_ref());
        Secp256k1Point(PublicKey::from_slice(&v).unwrap())
    }

    // fn random_point() -> Secp256k1Point {
    //     let random_scalar: Secp256k1Scalar = Secp256k1Scalar::new_random();
    //     let base_point = Self::generator();
    //     base_point.scalar_mul(&random_scalar)
    // }

    fn add_point(&self, other: &PublicKey) -> Secp256k1Point {
        Secp256k1Point(self.0.combine(other).unwrap())
    }

    fn scalar_mul(&self, other: &Secp256k1Scalar) -> Secp256k1Point {
        let mut new_point = *self;
        new_point
            .0
            .mul_assign(get_context(), &other.0[..])
            .expect("Assignment expected");
        new_point
    }
}

impl Secp256k1Scalar {
    ///
    pub fn new_random() -> Secp256k1Scalar {
        // let rand_bytes = thread_rng().gen::<[u8; 32]>();
        let mut rand_bytes = [0u8; 32];
        thread_rng().fill(&mut rand_bytes[..]);
        Secp256k1Scalar(SecretKey::from_slice(&rand_bytes[..]).unwrap())
    }

    ///
    pub fn zero() -> Secp256k1Scalar {
        let zero_arr = [0u8; 32];
        let zero = unsafe { std::mem::transmute::<[u8; 32], SecretKey>(zero_arr) };
        Secp256k1Scalar(zero)
    }

    ///
    pub fn one() -> Secp256k1Scalar {
        Secp256k1Scalar::from_bigint(&BigInt::from(1))
    }
    fn to_bigint(&self) -> BigInt {
        // Scalar is big endian in bitcoin secp256k1 impl
        BigInt::from_bytes_be(Plus, &self.0[..])
    }

    ///
    pub fn curve_order() -> BigInt {
        BigInt::from_bytes_be(Plus, &CURVE_ORDER)
    }

    fn add_scalar(&self, other: &Secp256k1Scalar) -> Secp256k1Scalar {
        let result_bigint = self.to_bigint() + other.to_bigint();
        let result_bigint_mod = result_bigint.mod_floor(&Secp256k1Scalar::curve_order());
        Secp256k1Scalar::from_bigint(&result_bigint_mod)
    }

    fn sub_scalar(&self, other: &Secp256k1Scalar) -> Secp256k1Scalar {
        let result_bigint = self.to_bigint() - other.to_bigint();
        let result_bigint_mod = result_bigint.mod_floor(&Secp256k1Scalar::curve_order());
        Secp256k1Scalar::from_bigint(&result_bigint_mod)
    }

    fn mul_scalar(&self, other: &Secp256k1Scalar) -> Secp256k1Scalar {
        let result_bigint = self.to_bigint() * other.to_bigint();
        let result_bigint_mod = result_bigint.mod_floor(&Secp256k1Scalar::curve_order());
        Secp256k1Scalar::from_bigint(&result_bigint_mod)
    }

    ///
    pub fn inv(&self) -> Secp256k1Scalar {
        let element = self.to_bigint();
        let modulus = Secp256k1Scalar::curve_order();
        Secp256k1Scalar::from_bigint(&element.mod_inverse(&modulus).unwrap())
    }

    /// Calculate the inverse of Scalar, using the Euclid Extend Algorithm
    pub fn inverse(&self) -> Secp256k1Scalar {
        let num = self.to_bigint();
        let order = Secp256k1Scalar::curve_order();
        let (mut r, mut next_r, mut s, mut next_s, mut t, mut next_t) = (
            order.clone(),
            num.clone(),
            BigInt::from(1),
            BigInt::from(0),
            BigInt::from(0),
            BigInt::from(1),
        );
        let mut quotient;
        let mut tmp;
        while next_r > BigInt::from(0) {
            quotient = r.clone() / next_r.clone();
            tmp = next_r.clone();
            next_r = r.clone() - next_r.clone() * quotient.clone();
            r = tmp.clone();
            tmp = next_s.clone();
            next_s = s - next_s.clone() * quotient.clone();
            s = tmp;
            tmp = next_t.clone();
            next_t = t - next_t * quotient;
            t = tmp;
        }
        Secp256k1Scalar::from_bigint(&t)
    }

    ///
    pub fn mod_scalar(&self) -> Secp256k1Scalar {
        let bigint_self = self.to_bigint();
        let mod_bigint_self = bigint_self.mod_floor(&Secp256k1Scalar::curve_order());
        // println!(
        //     "bigint_self: {:?}, curve_order: {:?}, mod_bigint_self: {:?}",
        //     bigint_self,
        //     Secp256k1Scalar::curve_order(),
        //     mod_bigint_self
        // );
        Secp256k1Scalar::from_bigint(&mod_bigint_self)
    }

    ///
    pub fn from_bigint(n: &BigInt) -> Secp256k1Scalar {
        if *n == BigInt::from(0) {
            Secp256k1Scalar::zero()
        } else {
            let (_sign, mut result_bytes) = n.to_bytes_be();
            if result_bytes.len() < SECRET_KEY_SIZE {
                let mut padding = vec![0; SECRET_KEY_SIZE - result_bytes.len()];
                padding.extend(result_bytes.iter());
                result_bytes = padding
            }
            // println!("secret_key: {:?}", result_bytes);
            Secp256k1Scalar(SecretKey::from_slice(&result_bytes).unwrap())
        }
    }
    ///
    pub fn from_hex(hex: &[u8]) -> Secp256k1Scalar {
        Secp256k1Scalar::from_bigint(&BigInt::parse_bytes(hex, 16).unwrap())
    }
}
impl Add<Secp256k1Scalar> for Secp256k1Scalar {
    type Output = Secp256k1Scalar;
    fn add(self, other: Secp256k1Scalar) -> Self::Output {
        self.add_scalar(&other)
    }
}
impl Sub<Secp256k1Scalar> for Secp256k1Scalar {
    type Output = Secp256k1Scalar;
    fn sub(self, other: Secp256k1Scalar) -> Self::Output {
        self.sub_scalar(&other)
    }
}
impl Mul<Secp256k1Scalar> for Secp256k1Scalar {
    type Output = Secp256k1Scalar;
    fn mul(self, other: Secp256k1Scalar) -> Self::Output {
        self.mul_scalar(&other)
    }
}
impl Add<Secp256k1Point> for Secp256k1Point {
    type Output = Secp256k1Point;
    fn add(self, other: Secp256k1Point) -> Self::Output {
        self.add_point(&other.0)
    }
}

impl Mul<Secp256k1Scalar> for Secp256k1Point {
    type Output = Secp256k1Point;
    fn mul(self, other: Secp256k1Scalar) -> Self::Output {
        self.scalar_mul(&other)
    }
}

static mut CONTEXT: Option<Secp256k1<VerifyOnly>> = None;
///
pub fn get_context() -> &'static Secp256k1<VerifyOnly> {
    static INIT_CONTEXT: Once = Once::new();
    INIT_CONTEXT.call_once(|| unsafe {
        CONTEXT = Some(Secp256k1::verification_only());
    });
    unsafe { CONTEXT.as_ref().unwrap() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bigint_to_scalar() {
        let random_scalar = Secp256k1Scalar::new_random();
        let bigint = random_scalar.to_bigint();
        let scalar2 = Secp256k1Scalar::from_bigint(&bigint);
        assert_eq!(random_scalar, scalar2);
    }
}
