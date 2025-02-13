use pqcrypto::prelude::*;
use pqcrypto_dilithium::dilithium3;
use crate::errors::Secp256k1SchnorrError;
use solana_secp256k1::{Secp256k1Point, UncompressedPoint};

/// Lattice-based Schnorr Challenge
pub struct LatticeChallenge;

impl Secp256k1SchnorrVerify for LatticeChallenge {
    fn challenge<T: Secp256k1Point>(r: &[u8; 32], pubkey: &T, message: &[u8]) -> [u8; 32] {
        let mut input = r.to_vec();
        input.extend_from_slice(&pubkey.x());
        input.extend_from_slice(message);
        dilithium3::detached_sign(&input, &dilithium3::keypair().0).as_bytes().to_vec()[..32].try_into().unwrap()
    }
}

impl Secp256k1SchnorrSign for LatticeChallenge {
    fn aux_randomness(secret_key: &[u8; 32], aux: &[u8; 32]) -> [u8; 32] {
        let mut t = dilithium3::detached_sign(aux, &dilithium3::keypair().0).as_bytes().to_vec()[..32].try_into().unwrap();
        for (a, b) in t.iter_mut().zip(secret_key.iter()) {
            *a ^= b;
        }
        t
    }

    fn nonce<T: Secp256k1Point>(
        pubkey: &T,
        message: &[u8],
        aux: &[u8; 32],
    ) -> Result<([u8; 32], UncompressedPoint), Secp256k1SchnorrError> {
        let k = dilithium3::detached_sign(&[aux, pubkey.x().as_ref(), message].concat(), &dilithium3::keypair().0)
            .as_bytes().to_vec()[..32].try_into().unwrap();
        let r = Curve::mul_g(&k).map_err(|_| Secp256k1SchnorrError::InvalidNonce)?;
        Ok((k, r))
    }
} 
