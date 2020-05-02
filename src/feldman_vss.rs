use crate::secp256k1_helper::{Secp256k1Point, Secp256k1Scalar};
use num_bigint_dig::BigInt;
/// The `VerifiableSecretSharing` stores.
pub struct VerifiableSecretSharing {
    /// the threshold of shares to recover the secret.
    pub threshold: usize,
    /// the total number of shares to generate from the secret.
    pub share_amount: usize,
}

impl VerifiableSecretSharing {
    /// Split the secret to shares and commitments.
    ///
    /// Shares are represented as Secp256k1Scalar, which is 32 bytes slice.
    /// Commitment are represented as Secp256k1Point
    pub fn split(
        &self,
        secret: &Secp256k1Scalar,
    ) -> (Vec<(usize, Secp256k1Scalar)>, Vec<Secp256k1Point>) {
        assert!(self.threshold <= self.share_amount);
        let polynomial = self.sample_polynomial(secret);
        let shares = self.evaluate_polynomial(&polynomial);
        println!("polynomial: {:?}", polynomial);
        let commitments = Self::generate_commitments(&polynomial);
        (shares, commitments)
    }

    /// Recover the secret by threshold+1 shares.
    pub fn recover(&self, shares: &[(usize, Secp256k1Scalar)]) -> Secp256k1Scalar {
        assert!(shares.len() == self.threshold);
        let (xs, ys): (Vec<usize>, Vec<Secp256k1Scalar>) = shares.iter().cloned().unzip();
        self.lagrange_interpolation(Secp256k1Scalar::zero(), &xs, &ys)
    }

    /// Verify a specific share distributed by the dealer is valid.
    pub fn verify(share: (usize, Secp256k1Scalar), commitments: &[Secp256k1Point]) -> bool {
        let generator = Secp256k1Point::generator();
        let (share_index, share_value) = share;
        let share_value_commitment = generator * share_value;
        let share_index_scalar = Secp256k1Scalar::from_bigint(&BigInt::from(share_index));
        let mut commitments_iter_rev = commitments.iter().rev();
        let commitments_head = commitments_iter_rev.next().unwrap();
        // println!(
        //     "share_index_scalar:{:?}, share_value:{:?}, commitments: {:?}",
        //     share_index_scalar, share_value, commitments
        // );
        let share_index_commitment = commitments_iter_rev.fold(*commitments_head, |sum, item| {
            // println!(
            //     "sum * share_index_scalar + *item: {:?} * {:?} + {:?}",
            //     sum, share_index_scalar, *item
            // );
            sum * share_index_scalar + *item
        });
        // println!(
        //     "share_value_commitment: {:?} \n
        //     share_index_commitment:{:?} \n
        //     manual_value_commitment: {:?} \n
        //     manual_a0_commitment: {:?}, \n
        //     manual_a1_commitment: {:?}, \n
        //     manual_index_commitment: {:?}",
        //     share_value_commitment,
        //     share_index_commitment,
        //     generator * Secp256k1Scalar::from_bigint(&BigInt::from(11)),
        //     generator * Secp256k1Scalar::from_bigint(&BigInt::from(5)),
        //     generator * Secp256k1Scalar::from_bigint(&BigInt::from(3)),
        //     generator
        //         * Secp256k1Scalar::from_bigint(&BigInt::from(3))
        //         * Secp256k1Scalar::from_bigint(&BigInt::from(2))
        //         + generator * Secp256k1Scalar::from_bigint(&BigInt::from(5)),
        // );
        share_value_commitment == share_index_commitment
    }

    fn generate_commitments(polynomial: &[Secp256k1Scalar]) -> Vec<Secp256k1Point> {
        let generator: Secp256k1Point = Secp256k1Point::generator();
        let len = polynomial.len();
        (0..len).map(|i| generator * polynomial[i]).collect()
    }

    fn sample_polynomial(&self, secret: &Secp256k1Scalar) -> Vec<Secp256k1Scalar> {
        let mut coefficients = vec![*secret];
        let random_coefficients: Vec<Secp256k1Scalar> = (0..(self.threshold - 1))
            .map(|_| Secp256k1Scalar::new_random())
            .collect();
        coefficients.extend(random_coefficients);
        coefficients
    }

    fn evaluate_polynomial(&self, polynomial: &[Secp256k1Scalar]) -> Vec<(usize, Secp256k1Scalar)> {
        (1..=self.share_amount)
            .map(|x| (x, self.mod_evaluate_at(polynomial, x)))
            .collect()
    }

    fn mod_evaluate_at(&self, polynomial: &[Secp256k1Scalar], x: usize) -> Secp256k1Scalar {
        let scalar_x: Secp256k1Scalar = Secp256k1Scalar::from_bigint(&BigInt::from(x));
        polynomial
            .iter()
            .rev()
            .fold(Secp256k1Scalar::zero(), |sum, item| {
                // println!(
                //     "scalar_x * sum + item: {:?} * {:?} + {:?}",
                //     scalar_x, sum, *item
                // );
                (scalar_x * sum + *item).mod_scalar()
            })
    }

    fn lagrange_interpolation(
        &self,
        x: Secp256k1Scalar,
        xs: &[usize],
        ys: &[Secp256k1Scalar],
    ) -> Secp256k1Scalar {
        let scalar_xs: Vec<Secp256k1Scalar> = xs
            .iter()
            .map(|x| Secp256k1Scalar::from_bigint(&BigInt::from(*x)))
            .collect();
        (0..self.threshold).fold(Secp256k1Scalar::zero(), |sum, item| {
            let numerator: Secp256k1Scalar =
                (0..self.threshold).fold(Secp256k1Scalar::one(), |product, i| {
                    if i == item {
                        product
                    } else {
                        (product * (x - scalar_xs[i])).mod_scalar()
                    }
                });
            let denominator: Secp256k1Scalar =
                (0..self.threshold).fold(Secp256k1Scalar::one(), |product, i| {
                    if i == item {
                        product
                    } else {
                        product * (scalar_xs[item] - scalar_xs[i]).mod_scalar()
                    }
                });
            // println!(
            //     "sum + num/den * ys[{:?}]: {:?} + {:?} / {:?} * {:?} ",
            //     item, sum, numerator, denominator, ys[item]
            // );
            (sum + numerator * denominator.inv() * ys[item]).mod_scalar()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vss_3_of_5_works() {
        let secret: Secp256k1Scalar = Secp256k1Scalar::new_random();
        let vss = VerifiableSecretSharing {
            threshold: 3,
            share_amount: 5,
        };
        let (shares, commitments) = vss.split(&secret);
        let sub_shares = &shares[0..3];
        let recovered = vss.recover(&sub_shares);
        assert_eq!(secret, recovered);
        for share in shares {
            assert!(VerifiableSecretSharing::verify(share, &commitments))
        }
    }

    #[test]
    fn test_vss_2_of_2_works() {
        let secret: Secp256k1Scalar = Secp256k1Scalar::new_random();
        let vss = VerifiableSecretSharing {
            threshold: 2,
            share_amount: 2,
        };
        let (shares, commitments) = vss.split(&secret);
        println!("shares: {:?}", shares);
        let sub_shares = &shares[0..2];
        let recovered = vss.recover(&sub_shares);
        assert_eq!(secret, recovered);
        for share in shares {
            assert!(VerifiableSecretSharing::verify(share, &commitments))
        }
    }

    // y = 5 + 3x, point1(1,8), point2(2,11)
    #[test]
    fn test_vss_simple_2_of_2_works() {
        let secret: Secp256k1Scalar = Secp256k1Scalar::from_hex(
            b"0000000000000000000000000000000000000000000000000000000000000005",
        );
        let cof = Secp256k1Scalar::from_hex(
            b"0000000000000000000000000000000000000000000000000000000000000003",
        );
        let polynomial = vec![secret, cof];
        let vss = VerifiableSecretSharing {
            threshold: 2,
            share_amount: 2,
        };
        let shares = vss.evaluate_polynomial(&polynomial);
        println!("shares: {:?}", shares);

        let recovered = vss.recover(&shares);
        assert_eq!(secret, recovered);
        let commitments = VerifiableSecretSharing::generate_commitments(&polynomial);
        for share in shares {
            assert!(VerifiableSecretSharing::verify(share, &commitments))
        }
    }

    #[test]
    fn test_vss_67_of_100_works() {
        let secret: Secp256k1Scalar = Secp256k1Scalar::new_random();
        let vss = VerifiableSecretSharing {
            threshold: 67,
            share_amount: 100,
        };
        let (shares, commitments) = vss.split(&secret);
        let sub_shares = &shares[0..67];
        let recovered = vss.recover(&sub_shares);
        assert_eq!(secret, recovered);
        for share in shares {
            assert!(VerifiableSecretSharing::verify(share, &commitments))
        }
    }
}
