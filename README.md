# Shamir Secret Sharing(Rust)

## Intro

A rust implementation of  Shamir Secret Sharing over Finite Field.

The lib support large field charactirics `prime` by taking advantage of `num_bigint` .
It's not optimized for production purpose, which can be improved in several aspects:
1.replace the `extended_euclid_algo` with machine-friendly `stein_algo` to calculate the modulo inverse; 

2. add commitment scheme to make it verifiable

## Example

``` rust
use shamir_secret_sharing::ShamirSecretSharing as SSS;
use num_bigint::{BigInt, BigUint};
use num_bigint::Sign::*;
fn main() {
let sss = SSS {
    threshold: 3,
    share_amount: 5,
    prime: BigInt::parse_bytes(b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f",16).unwrap()
    };

let secret = BigInt::parse_bytes(b"ffffffffffffffffffffffffffffffffffffff", 16).unwrap();

let shares = sss.split(secret.clone());

println!("shares: {:?}", shares);
assert_eq!(secret, sss.recover(&shares[0..sss.threshold as usize]));
}

```

