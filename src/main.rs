use solana_secp256k1_schnorr::vaults::lattice_schnorr::LatticeSchnorrVault;

fn main() {
    // Generate a new key pair
    let (sk, pk) = LatticeSchnorrVault::generate_keypair();

    // Sign a message
    let message = b"test";
    let signature = LatticeSchnorrVault::sign(&sk, message);

    // Verify the signature
    LatticeSchnorrVault::verify(&pk, message, &signature).expect("Invalid signature");

    println!("Lattice-based Schnorr vault test passed!");
}
