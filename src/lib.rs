use num_bigint::BigInt;
use num_traits::{One, Zero};
#[cfg(feature = "rand")]
use rand;
// use rand::distributions::Distribution;
#[derive(Clone, Debug)]
pub struct ShamirSecretSharing {
    pub threshold: usize,
    pub share_amount: usize,
    pub prime: BigInt,
}

impl ShamirSecretSharing {
    pub fn split(&self, secret: BigInt) -> Vec<(usize, BigInt)> {
        assert!(self.threshold < self.share_amount);
        let polynomial = self.sample_polynomial(secret);
        self.evaluate_polynomial(polynomial)
    }

    #[cfg(not(feature = "rand"))]
    fn sample_polynomial(&self, secret: BigInt) -> Vec<BigInt> {
        vec![]
    }
    #[cfg(feature = "rand")]
    fn sample_polynomial(&self, secret: BigInt) -> Vec<BigInt> {
        let mut coefficients: Vec<BigInt> = vec![secret];
        // let distr = rand::distributions::Uniform::new_inclusive(0, self.prime - 1);
        let mut rng = rand::thread_rng();
        let random_coefficients: Vec<BigInt> = (0..self.threshold)
            // .map(|_| distr.sample(&mut rng))
            .map(|_| rng.gen_bigint_range(&Zero::zero(), &(self.prime - One::one())))
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
            sum * (x_bigint + item) % self.prime
        })
    }

    pub fn recover(&self, shares: &[(usize, BigInt)]) -> BigInt {
        assert!(shares.len() == self.threshold, "wrong shares number");
        let (xs, ys): (Vec<usize>, Vec<BigInt>) = shares.iter().cloned().unzip();
        self.lagrange_interpolation(Zero::zero(), xs, ys)
    }

    // pub fn verify(&self, all_shares: Vec<(BigInt, BigInt)>) -> Option<BigInt> {
    //     assert!(
    //         all_shares.len() == self.share_amount,
    //         "wrong all_shares number"
    //     );
    //     let shares = &all_shares[0..self.threshold];
    // }

    fn lagrange_interpolation(&self, x: BigInt, xs: Vec<usize>, ys: Vec<BigInt>) -> BigInt {
        let len = xs.len();
        let xs_bigint: Vec<BigInt> = xs.iter().map(|&x| BigInt::from(x)).collect();
        (0..len).fold(Zero::zero(), |sum, item| {
            let numerator = (0..len).fold(One::one(), |product: BigInt, i| {
                if i == item {
                    product
                } else {
                    product * (x - xs_bigint[i]) % self.prime
                }
            });
            let denominator = (0..len).fold(One::one(), |product: BigInt, i| {
                if i == item {
                    product
                } else {
                    product * (xs_bigint[item] - xs_bigint[i]) % self.prime
                }
            });
            (sum + numerator * self.mod_reverse(denominator) * ys[item]) % self.prime
        })
    }

    fn mod_reverse(&self, num: BigInt) -> BigInt {
        let (_gcd, _, inv) = self.extend_euclid_algo(num);
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
            self.prime,
            num,
            One::one(),
            Zero::zero(),
            Zero::zero(),
            One::one(),
        );
        let mut quotient;
        let mut tmp;
        while next_r > Zero::zero() {
            quotient = r / next_r;
            tmp = next_r;
            next_r = r - next_r * quotient;
            r = tmp;
            tmp = next_s;
            next_s = s - next_s * quotient;
            s = tmp;
            tmp = next_t;
            next_t = t - next_t * quotient;
            t = tmp;
        }
        (r, s, t)
    }
    // fn mod_euc(&self, lhs: BigInt) -> BigInt {
    //     let r = lhs % self.prime;
    //     if r < 0 {
    //         r + self.prime.abs()
    //     } else {
    //         r
    //     }
    // }
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
        // assert_eq!(sss.recover(&[(1, 1494), (2, 329), (3, 965)]), 1234);
    }
    #[test]
    fn test_split_and_recover() {
        // let sss = ShamirSecretSharing {
        //     threshold: 3,
        //     share_amount: 5,
        //     prime: 6999213259363483493573619703,
        // };
        // let secret = 1234567890;
        // let shares = sss.split(secret);
        // assert_eq!(secret, sss.recover(&shares[0..sss.threshold as usize]));
    }
}
