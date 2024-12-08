use rand::rngs::OsRng;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha2::{Sha256, Digest};

fn main() {
    let secp = Secp256k1::new();
    let prefix = "abcdef"; // The desired prefix for the public key (in hexadecimal)

    loop {
        // Generate a random private key
        let mut rng = OsRng::new().expect("Unable to access randomness");
        let secret_key = SecretKey::new(&mut rng);
        
        // Generate the corresponding public key
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        let public_key_serialized = public_key.serialize_uncompressed();

        // Convert the public key to a hexadecimal string
        let public_key_hex = hex::encode(&public_key_serialized);

        // Check if the public key starts with the desired prefix
        if public_key_hex.starts_with(prefix) {
            println!("Found matching key pair!");
            println!("Private Key: {:?}", secret_key.display_secret());
            println!("Public Key: {}", public_key_hex);
            break;
        }
    }
}
