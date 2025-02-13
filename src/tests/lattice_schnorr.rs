use crate::vaults::lattice_schnorr::LatticeSchnorrVault;

#[test]
fn test_lattice_schnorr_vault() {
    // Generate a new key pair
    let (sk, pk) = LatticeSchnorrVault::generate_keypair();

    // Sign a message
    let message = b"test";
    let signature = LatticeSchnorrVault::sign(&sk, message);

    // Verify the signature
    assert!(LatticeSchnorrVault::verify(&pk, message, &signature).is_ok());
}

#[test]
fn test_lattice_schnorr_vault_invalid_signature() {
    // Generate a new key pair
    let (sk, pk) = LatticeSchnorrVault::generate_keypair();

    // Sign a message
    let message = b"test";
    let mut signature = LatticeSchnorrVault::sign(&sk, message);

    // Tamper with the signature
    let mut sig_bytes = signature.as_bytes().to_vec();
    sig_bytes[0] ^= 0xFF;  // Flip a bit to invalidate the signature
    let invalid_signature = DetachedSignature::from_bytes(&sig_bytes).unwrap();

    // Verify the invalid signature
    assert!(LatticeSchnorrVault::verify(&pk, message, &invalid_signature).is_err());
} 
