use clap::Parser;
use curve25519_dalek::{
    constants::RISTRETTO_BASEPOINT_POINT as G, ristretto::RistrettoPoint, scalar::Scalar,
};
use rand_core::OsRng;
use sha2::{Digest, Sha512};

/// Command-line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Ring size
    #[arg(short, long, default_value_t = 5)]
    n: usize,

    /// Index of signer
    #[arg(short, long, default_value_t = 2)]
    signer: usize,

    /// Message to sign
    #[arg(short, long, default_value = "Ring signatures are cool.")]
    message: String,
}

/// Domain-separated hash function returning a Scalar
fn hash_with_domain(domain: &[u8], inputs: &[&[u8]]) -> Scalar {
    let mut hasher = Sha512::new();
    hasher.update(domain);
    for input in inputs {
        hasher.update(input);
    }
    let result = hasher.finalize();
    let wide: [u8; 64] = result.into();
    Scalar::from_bytes_mod_order_wide(&wide)
}

/// Generate a keypair: (secret key, public key)
fn keygen() -> (Scalar, RistrettoPoint) {
    let sk = Scalar::random(&mut OsRng);
    let pk = sk * G;

    (sk, pk)
}

/// Convert Scalar to hex string
fn to_hex(scalar: &Scalar) -> String {
    hex::encode(scalar.to_bytes())
}

/// Produce a ring signature over the given message and public key ring
fn ring_sign(
    message: &[u8],
    public_keys: &[RistrettoPoint],
    sk: Scalar,
    signer_index: usize,
) -> (Scalar, Vec<Scalar>) {
    assert!(
        signer_index < public_keys.len(),
        "Signer index out of bounds"
    );

    let n = public_keys.len();
    // Initialize challenges and responses
    let mut cs = vec![Scalar::default(); n];
    let mut rs = vec![Scalar::default(); n];

    // Step 1: choose random nonce k and compute initial R
    let mut rng = OsRng;
    let k = Scalar::random(&mut rng);
    let r = k * G;

    // Step 2: initialize challenge at index (signer + 1) % n
    let mut i = (signer_index + 1) % n;
    cs[i] = hash_with_domain(b"ringsig", &[r.compress().as_bytes(), message]);

    // Step 3: loop through the rest of the ring
    for _ in 0..(n - 1) {
        // Compute response for current index
        rs[i] = Scalar::random(&mut rng);
        let r_i = rs[i] * G + cs[i] * public_keys[i];

        // Compute next challenge
        i = (i + 1) % n;
        cs[i] = hash_with_domain(b"ringsig", &[r_i.compress().as_bytes(), message]);
    }

    // Step 4: close the ring by computing the signer's response
    rs[signer_index] = k - sk * cs[signer_index];

    (cs[0], rs)
}

/// Verify a ring signature
fn ring_verify(message: &[u8], public_keys: &[RistrettoPoint], c0: Scalar, rs: &[Scalar]) -> bool {
    let n = public_keys.len();
    let mut cs = vec![c0];

    // Reconstruct challenges
    for i in 0..n {
        let r = rs[i] * G + cs[i] * public_keys[i];
        let c_next = hash_with_domain(b"ringsig", &[r.compress().as_bytes(), message]);
        cs.push(c_next);
    }

    // Valid if the challenge cycle closes
    cs[n] == cs[0]
}

fn main() {
    let args = Args::parse();

    let n = args.n;
    let signer_index = args.signer;
    let message = args.message;
    println!("Number of Signers : {}", n);
    println!("Signer Index      : {}", signer_index);
    println!("Message           : {}", message);
    let message_bytes = message.into_bytes();

    let mut keypairs = Vec::new();
    let mut public_keys = Vec::new();

    // Generate keypairs
    for _ in 0..n {
        let (sk, pk) = keygen();
        keypairs.push(sk);
        public_keys.push(pk);
    }

    // Sign and verify
    let (c0, s_vec) = ring_sign(
        &message_bytes,
        &public_keys,
        keypairs[signer_index],
        signer_index,
    );
    let valid = ring_verify(&message_bytes, &public_keys, c0, &s_vec);

    println!("\nSignature:");
    println!("  c0 = {}", to_hex(&c0));
    for (i, s) in s_vec.iter().enumerate() {
        println!("  s[{}] = {}", i, to_hex(s));
    }

    println!("\nValid signature: {}", valid);
}
