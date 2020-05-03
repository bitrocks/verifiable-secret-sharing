# Verifiable Secret Sharing(Rust)

## Intro

A rust implementation of Verifiable Secret Sharing over Finite Field.

* [x] impl naive shamir's secret sharing; 
* [x] impl feldman's verifiable secret sharing; 
* [ ] improve the mod_inv impl, replace extended_euclid_algorithm with stein_algorithm
* [ ] impl publicly verifiable secret sharing
* [ ] client-server mode

It's not optimized for production purpose yet.

## Simple Shamir Secret Sharing

The lib support large field charactirics `prime` by taking advantage of `num_bigint` .

### Example

``` rust
use verifiable_secret_sharing::ShamirSecretSharing as SSS;
use num_bigint::{BigInt, BigUint};
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

## Verifiable Secret Sharing(VSS)

[A practical scheme for non-interactive verifiable secret sharing](./paper/feldmanVSS.pdf)

### How it works?
check [feldmanVSS review note](./paper/feldmanVSS_review_cn.md).

### Example
``` rust

use verifiable_secret_sharing::VerifiableSecretSharing;
use verifiable_secret_sharing::Secp256k1Scalar;
fn main(){
    let secret: Secp256k1Scalar = Secp256k1Scalar::from_hex(b"7613c39ea009afd24ccf8c25f13591377091297b20a48ecaad0e92618d36dcc6");
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
```

## Publicly Verifiable Secret Sharing(PVSS)
[Publicly Verifiable Secret Sharing](./paper/stadlerPVSS.pdf)











