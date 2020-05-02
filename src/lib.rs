#![deny(missing_docs)]
//! A lib impl Secret Sharing Scheme

//! A rust implementation of  Shamir Secret Sharing over Finite Field.
//!
//!
//! ## Example
//! ### shamir's secret sharing
//!
//!  ```rust
//! use verifiable_secret_sharing::ShamirSecretSharing as SSS;
//! use num_bigint::{BigInt, BigUint};
//! # fn main() {
//! let sss = SSS {
//!     threshold: 3,
//!     share_amount: 5,
//!     prime: BigInt::parse_bytes(b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f",16).unwrap()
//!     };
//!
//! let secret = BigInt::parse_bytes(b"ffffffffffffffffffffffffffffffffffffff", 16).unwrap();
//!
//! let shares = sss.split(secret.clone());
//!
//! println!("shares: {:?}", shares);
//! assert_eq!(secret, sss.recover(&shares[0..sss.threshold as usize]));
//! # }
//!
//! ```
//!
//! ### feldman's verifiable secret sharing
//!
//! ```rust
//! use verifiable_secret_sharing::VerifiableSecretSharing;
//! use verifiable_secret_sharing::Secp256k1Scalar;
//! # fn main(){
//! let secret: Secp256k1Scalar = Secp256k1Scalar::from_hex(b"7613c39ea009afd24ccf8c25f13591377091297b20a48ecaad0e92618d36dcc6");
//! let vss = VerifiableSecretSharing {
//!     threshold: 3,
//!     share_amount: 5,
//! };
//! let (shares, commitments) = vss.split(&secret);
//! let sub_shares = &shares[0..3];
//! let recovered = vss.recover(&sub_shares);
//! assert_eq!(secret, recovered);
//! for share in shares {
//!     assert!(VerifiableSecretSharing::verify(share, &commitments))
//! }
//! # }
//! ```
pub use feldman_vss::VerifiableSecretSharing;
pub use secp256k1_helper::{Secp256k1Point, Secp256k1Scalar};
pub use simple_sss::ShamirSecretSharing;

mod feldman_vss;
mod secp256k1_helper;
mod simple_sss;
