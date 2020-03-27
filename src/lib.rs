use rand;
use rand::distributions::Distribution;

#[derive(Clone, Debug)]
pub struct ShamirSecretSharing {
    pub threshold: i128,
    pub share_amount: i128,
    pub prime: i128,
}

impl ShamirSecretSharing {
    pub fn split(&self, secret: i128) -> Vec<(i128, i128)> {
        assert!(self.threshold < self.share_amount);
        let polynomial = self.sample_polynomial(secret);
        self.evaluate_polynomial(polynomial)
    }

    fn sample_polynomial(&self, secret: i128) -> Vec<i128> {
        let mut coefficients: Vec<i128> = vec![secret];
        let distr = rand::distributions::Uniform::new_inclusive(0, self.prime - 1);
        let mut rng = rand::thread_rng();
        let random_coefficients: Vec<i128> = (0..self.threshold)
            .map(|_| distr.sample(&mut rng))
            .collect();
        coefficients.extend(random_coefficients);
        coefficients
    }

    fn evaluate_polynomial(&self, polynomial: Vec<i128>) -> Vec<(i128, i128)> {
        (1..=self.share_amount)
            .map(|x| (x, self.mod_evaluate_at(&polynomial, x)))
            .collect()
    }

    fn mod_evaluate_at(&self, polynomial: &[i128], x: i128) -> i128 {
        polynomial.iter().rev().fold(0 as i128, |sum, item| {
            (sum * (x as i128) + item) % self.prime
        })
    }

    pub fn recover(&self, shares: &[(i128, i128)]) -> i128 {
        assert!(
            shares.len() as i128 == self.threshold,
            "wrong shares number"
        );
        let (xs, ys): (Vec<i128>, Vec<i128>) = shares.iter().cloned().unzip();
        self.lagrange_interpolation(0, xs, ys)
    }

    // pub fn verify(&self, all_shares: Vec<(i128, i128)>) -> Option<i128> {
    //     assert!(
    //         all_shares.len() == self.share_amount,
    //         "wrong all_shares number"
    //     );
    //     let shares = &all_shares[0..self.threshold];
    // }

    fn lagrange_interpolation(&self, x: i128, xs: Vec<i128>, ys: Vec<i128>) -> i128 {
        let len = xs.len();
        (0..len).fold(0, |sum, item| {
            let numerator = (0..len).fold(1, |product, i| {
                if i == item {
                    product
                } else {
                    product * ((x - xs[i]) as i128) % self.prime
                }
            });
            let denominator = (0..len).fold(1, |product, i| {
                if i == item {
                    product
                } else {
                    product * ((xs[item] - xs[i]) as i128) % self.prime
                }
            });
            self.mod_euc(
                sum + numerator * self.mod_reverse(self.mod_euc(denominator)) * ys[item]
                    % self.prime,
            )
        })
    }

    fn mod_reverse(&self, num: i128) -> i128 {
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
    fn extend_euclid_algo(&self, num: i128) -> (i128, i128, i128) {
        let (mut r, mut next_r, mut s, mut next_s, mut t, mut next_t) =
            (self.prime, num, 1, 0, 0, 1);
        let mut quotient;
        let mut tmp;
        while next_r > 0 {
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
    fn mod_euc(&self, lhs: i128) -> i128 {
        let r = lhs % self.prime;
        if r < 0 {
            r + self.prime.abs()
        } else {
            r
        }
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
            prime: 1613,
        };
        let shares = sss.evaluate_polynomial(vec![1234, 166, 94]);
        assert_eq!(
            shares,
            [(1, 1494), (2, 329), (3, 965), (4, 176), (5, 1188), (6, 775)]
        );
        assert_eq!(sss.recover(&[(1, 1494), (2, 329), (3, 965)]), 1234);
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
