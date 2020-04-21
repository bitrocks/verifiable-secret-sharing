pub use num_bigint;
use num_bigint::{BigInt, RandBigInt};
use num_traits::{One, Zero};
use rand;
/// The `ShamirSecretSharing` stores threshold, share_amount and the prime of finite field.
#[derive(Clone, Debug)]
pub struct ShamirSecretSharing {
    /// the threshold of shares to recover the secret.
    pub threshold: usize,
    /// the total number of shares to generate from the secret.
    pub share_amount: usize,
    /// the characteristic of finite field.
    pub prime: BigInt,
}

impl ShamirSecretSharing {
    /// Split a secret according to the config.
    pub fn split(&self, secret: BigInt) -> Vec<(usize, BigInt)> {
        assert!(self.threshold < self.share_amount);
        let polynomial = self.sample_polynomial(secret);
        // println!("polynomial: {:?}", polynomial);
        self.evaluate_polynomial(polynomial)
    }

    fn sample_polynomial(&self, secret: BigInt) -> Vec<BigInt> {
        let mut coefficients: Vec<BigInt> = vec![secret];
        let mut rng = rand::thread_rng();
        let low = BigInt::from(0);
        let high = &self.prime - BigInt::from(1);
        let random_coefficients: Vec<BigInt> = (0..(self.threshold - 1))
            .map(|_| rng.gen_bigint_range(&low, &high))
            .collect();
        coefficients.extend(random_coefficients);
        coefficients
    }

    fn evaluate_polynomial(&self, polynomial: Vec<BigInt>) -> Vec<(usize, BigInt)> {
        (1..=self.share_amount)
            .map(|x| (x, self.mod_evaluate_at(&polynomial, x)))
            .collect()
    }

    fn mod_evaluate_at(&self, polynomial: &[BigInt], x: usize) -> BigInt {
        let x_bigint = BigInt::from(x);
        polynomial.iter().rev().fold(Zero::zero(), |sum, item| {
            (&x_bigint * sum + item) % &self.prime
        })
    }

    /// Recover the secret by the shares.
    pub fn recover(&self, shares: &[(usize, BigInt)]) -> BigInt {
        assert!(shares.len() == self.threshold, "wrong shares number");
        let (xs, ys): (Vec<usize>, Vec<BigInt>) = shares.iter().cloned().unzip();
        let result = self.lagrange_interpolation(Zero::zero(), xs, ys);
        if result < Zero::zero() {
            result + &self.prime
        } else {
            result
        }
    }

    fn lagrange_interpolation(&self, x: BigInt, xs: Vec<usize>, ys: Vec<BigInt>) -> BigInt {
        let len = xs.len();
        // println!("x: {}, xs: {:?}, ys: {:?}", x, xs, ys);
        let xs_bigint: Vec<BigInt> = xs.iter().map(|x| BigInt::from(*x as i64)).collect();
        // println!("sx_bigint: {:?}", xs_bigint);
        (0..len).fold(Zero::zero(), |sum, item| {
            let numerator = (0..len).fold(One::one(), |product: BigInt, i| {
                if i == item {
                    product
                } else {
                    product * (&x - &xs_bigint[i]) % &self.prime
                }
            });
            let denominator = (0..len).fold(One::one(), |product: BigInt, i| {
                if i == item {
                    product
                } else {
                    product * (&xs_bigint[item] - &xs_bigint[i]) % &self.prime
                }
            });
            // println!(
            // "numerator: {}, donominator: {}, y: {}",
            // numerator, denominator, &ys[item]
            // );
            (sum + numerator * self.mod_reverse(denominator) * &ys[item]) % &self.prime
        })
    }

    fn mod_reverse(&self, num: BigInt) -> BigInt {
        let num1 = if num < Zero::zero() {
            num + &self.prime
        } else {
            num
        };
        let (_gcd, _, inv) = self.extend_euclid_algo(num1);
        // println!("inv:{}", inv);
        inv
    }

    /**
     * https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
     *
     * a*s + b*t = gcd(a,b) a > b
     * r_0 = a*s_0 + b*t_0    s_0 = 1    t_0 = 0
     * r_1 = a*s_1 + b*t_1    s_1 = 0    t_1 = 1
     * r_2 = r_0 - r_1*q_1
     *     = a(s_0 - s_1*q_1) + b(t_0 - t_1*q_1)   s_2 = s_0 - s_1*q_1     t_2 = t_0 - t_1*q_1
     * ...
     * stop when r_k = 0
     */
    fn extend_euclid_algo(&self, num: BigInt) -> (BigInt, BigInt, BigInt) {
        let (mut r, mut next_r, mut s, mut next_s, mut t, mut next_t) = (
            self.prime.clone(),
            num.clone(),
            BigInt::from(1),
            BigInt::from(0),
            BigInt::from(0),
            BigInt::from(1),
        );
        let mut quotient;
        let mut tmp;
        while next_r > Zero::zero() {
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
        // println!(
        // "{} * {} + {} * {} = {} mod {}",
        // num, t, &self.prime, s, r, &self.prime
        // );
        (r, s, t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_wikipedia_example() {
        let sss = ShamirSecretSharing {
            threshold: 3,
            share_amount: 6,
            prime: BigInt::from(1613),
        };
        let shares = sss.evaluate_polynomial(vec![
            BigInt::from(1234),
            BigInt::from(166),
            BigInt::from(94),
        ]);
        assert_eq!(
            shares,
            [
                (1, BigInt::from(1494)),
                (2, BigInt::from(329)),
                (3, BigInt::from(965)),
                (4, BigInt::from(176)),
                (5, BigInt::from(1188)),
                (6, BigInt::from(775))
            ]
        );
        assert_eq!(
            sss.recover(&[
                (1, BigInt::from(1494)),
                (2, BigInt::from(329)),
                (3, BigInt::from(965))
            ]),
            BigInt::from(1234)
        );
    }
    #[test]
    fn test_large_prime() {
        let sss = ShamirSecretSharing {
            threshold: 3,
            share_amount: 5,
            // prime: BigInt::from(6999213259363483493573619703 as i128),
            prime: BigInt::parse_bytes(
                b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f",
                16,
            )
            .unwrap(),
        };
        let secret = BigInt::parse_bytes(b"ffffffffffffffffffffffffffffffffffffff", 16).unwrap();
        let shares = sss.split(secret.clone());
        assert_eq!(secret, sss.recover(&shares[0..sss.threshold as usize]));
    }

    #[test]
    fn test_secp256k1() {
        use secp256k1::{Message, Secp256k1};
        let secp = Secp256k1::new();
        let mut rng = rand::thread_rng();
        let (secret, public) = secp.generate_keypair(&mut rng);
        let message = Message::from_slice(&[0xab; 32]).expect("32 bytes");
        let sig = secp.sign(&message, &secret);
        assert!(secp.verify(&message, &sig, &public).is_ok());
    }
}
