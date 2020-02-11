#![allow(non_camel_case_types)]

use bytes::{BytesMut, BufMut};
use crate::algorithm_dictionary::*;

pub mod prelude {
    pub use crate::{PQNode, PostQuantumContainer, PostQuantumType, algorithm_dictionary};
}

/// Contains the public keys for Alice and Bob
pub struct PostQuantumContainer {
    algorithm: u8,
    data: Box<dyn PostQuantumType>,
    node: PQNode,
}

/// Used to denote the local node's instance type
#[derive(PartialEq, Copy, Clone)]
pub enum PQNode {
    /// The first node in the exchange. Alice generates a key, gets a public key (pk)
    /// and a secret key (sk). Alice sends pk to Bob
    Alice,
    /// The second node in the exchange. Bob receives the Public key, pk, and encapsulates it.
    /// The encapsulation function returns a shared secret (ss) and a ciphertext (ct) for Bob.
    /// Bob then sends ct to Alice. Finally, Bob uses the newly received ct, coupled with his
    /// local sk to get the shared secret, ss. Ultimately, the ss is used to xor the bytes
    Bob,
}

impl PostQuantumContainer {
    /// Creates a new [PostQuantumContainer] for Alice. This will panic if the algorithm is
    /// invalid
    pub fn new_alice(algorithm: u8) -> Self {
        Self { algorithm, data: Self::get_new_alice(algorithm), node: PQNode::Alice }
    }

    /// Creates a new [PostQuantumContainer] for Bob. This will panic if the algorithm is
    /// invalid
    pub fn new_bob(algorithm: u8, public_key: &[u8]) -> Self {
        Self { algorithm, data: Self::get_new_bob(algorithm, public_key), node: PQNode::Bob }
    }

    /// Internally creates shared key after bob sends a response back to Alice
    pub fn alice_on_receive_ciphertext(&mut self, ciphertext: &[u8]) {
        debug_assert!(self.node == PQNode::Alice);
        self.data.alice_on_receive_ciphertext(ciphertext)
    }
    /// Gets the public key
    pub fn get_public_key(&self) -> &[u8] {
        self.data.get_public_key()
    }
    /// Gets the secret key (If node is Alice type)
    pub fn get_secret_key(&self) -> &[u8] {
        self.data.get_secret_key()
    }
    /// Gets the ciphertext
    pub fn get_ciphertext(&self) -> &[u8] {
        self.data.get_ciphertext()
    }
    /// Gets the shared secret
    pub fn get_shared_secret(&self) -> &[u8] {
        self.data.get_shared_secret()
    }

    /// Returns either Alice or Bob
    pub fn get_node_type(&self) -> PQNode {
        self.node
    }

    /// Returns the byte-sized representation of the algorithm used
    pub fn get_algorithm_idx(&self) -> u8 {
        self.algorithm
    }

    fn get_new_alice(algorithm: u8) -> Box<dyn PostQuantumType> {
        assert!(algorithm < ALGORITHM_COUNT);
        crate::function_pointers::ALICE_FP[algorithm as usize]()
    }

    fn get_new_bob(algorithm: u8, public_key: &[u8]) -> Box<dyn PostQuantumType> {
        assert!(algorithm < ALGORITHM_COUNT);
        crate::function_pointers::BOB_FP[algorithm as usize](public_key)
    }
}

/// Used for packet transmission
#[allow(missing_docs)]
pub mod algorithm_dictionary {

    pub const ALGORITHM_COUNT: u8 = 42;

    pub const BABYBEAR: u8 = 0;
    pub const BABYBEAREPHEM: u8 = 1;

    pub const FIRESABER: u8 = 2;

    pub const FRODOKEM640AES: u8 = 3;
    pub const FRODOKEM640SHAKE: u8 = 4;
    pub const FRODOKEM976AES: u8 = 5;
    pub const FRODOKEM976SHAKE: u8 = 6;
    pub const FRODOKEM1344AES: u8 = 7;
    pub const FRODOKEM1344SHAKE: u8 = 8;

    pub const KYBER512: u8 = 9;
    pub const KYBER768: u8 = 10;
    pub const KYBER1024: u8 = 11;
    pub const KYBER51290S: u8 = 12;
    pub const KYBER76890S: u8 = 13;
    pub const KYBER102490S: u8 = 14;

    pub const LEDAKEMLT12: u8 = 15;
    pub const LEDAKEMLT32: u8 = 16;
    pub const LEDAKEMLT52: u8 = 17;

    pub const LIGHTSABER: u8 = 18;

    pub const MAMABEAR: u8 = 19;
    pub const MAMABEAREPHEM: u8 = 20;

    pub const MCELIECE348864: u8 = 21;
    pub const MCELIECE348864F: u8 = 22;
    pub const MCELIECE460896: u8 = 23;
    pub const MCELIECE460896F: u8 = 24;
    pub const MCELIECE6688128: u8 = 25;
    pub const MCELIECE6688128F: u8 = 26;
    pub const MCELIECE6960119: u8 = 27;
    pub const MCELIECE6960119F: u8 = 28;
    pub const MCELIECE8192128: u8 = 29;
    pub const MCELIECE8192128F: u8 = 30;

    pub const NEWHOPE512CCA: u8 = 31;
    pub const NEWHOPE512CPA: u8 = 32;
    pub const NEWHOPE1024CCA: u8 = 33;
    pub const NEWHOPE1024CPA: u8 = 34;

    pub const NTRUHPS2048509: u8 = 35;
    pub const NTRUHPS2048677: u8 = 36;
    pub const NTRUHPS4096821: u8 = 37;
    pub const NTRUHRSS701: u8 = 38;

    pub const PAPABEAR: u8 = 39;
    pub const PAPABEAREPHEM: u8 = 40;

    pub const SABER: u8 = 41;
}

/// Used to get different algorithm types dynamically
pub trait PostQuantumType {
    /// Creates a new self for the initiating node
    fn new_alice() -> Self where Self: Sized;
    /// Creates a new self for the receiving node
    fn new_bob(public_key: &[u8]) -> Self where Self: Sized;
    /// Internally creates shared key after bob sends a response back to Alice
    fn alice_on_receive_ciphertext(&mut self, ciphertext: &[u8]);
    /// Gets the public key
    fn get_public_key(&self) -> &[u8];
    /// Gets the secret key (If node is Alice type)
    fn get_secret_key(&self) -> &[u8];
    /// Gets the ciphertext
    fn get_ciphertext(&self) -> &[u8];
    /// Gets the shared secret
    fn get_shared_secret(&self) -> &[u8];
    /// Encrypts the data. Since this is meant for small data, no heavy optimization is used
    fn encrypt_data<T: AsRef<[u8]>>(&self, input: T, output: &mut BytesMut) where Self: Sized {
        let input = input.as_ref();
        let ss = self.get_shared_secret();
        let ss_len = ss.len();

        for (idx, byte) in input.iter().enumerate() {
            output.put_u8(*byte ^ ss[idx % ss_len])
        }
    }
    /// Decrypts the data
    fn decrypt_data<T: AsRef<[u8]>>(&self, input: T, output: &mut BytesMut) where Self: Sized {
        let input = input.as_ref();
        let ss = self.get_shared_secret();
        let ss_len = ss.len();

        for (idx, byte) in input.iter().enumerate() {
            output.put_u8(*byte ^ ss[idx % ss_len])
        }
    }
}

macro_rules! create_struct {
    ($base:ident, $name:ident) => {
        /// Auto generated
        pub(crate) struct $base {
            /// The public key. Both Alice and Bob get this
            public_key: pqcrypto::kem::$name::PublicKey,
            /// Only Alice gets this one
            secret_key: Option<pqcrypto::kem::$name::SecretKey>,
            /// Both Bob and Alice get this one
            ciphertext: Option<pqcrypto::kem::$name::Ciphertext>,
            /// Both Alice and Bob get this (at the end)
            shared_secret: Option<pqcrypto::kem::$name::SharedSecret>
        }

        impl PostQuantumType for $base {
            fn new_alice() -> Self {
                let (public_key, secret_key) = pqcrypto::kem::$name::keypair();
                let ciphertext = None;
                let shared_secret = None;
                let secret_key = Some(secret_key);
                Self { public_key, secret_key, ciphertext, shared_secret }
            }

            fn new_bob(public_key: &[u8]) -> Self {
                let public_key = pqcrypto::kem::$name::PublicKey::from_bytes(public_key).unwrap();
                let (shared_secret, ciphertext) = pqcrypto::kem::$name::encapsulate(&public_key);
                let secret_key = None;
                let shared_secret = Some(shared_secret);
                let ciphertext = Some(ciphertext);
                Self { public_key, secret_key, ciphertext, shared_secret }
            }

            fn alice_on_receive_ciphertext(&mut self, ciphertext: &[u8]) {
                // These functions should only be called once upon response back from Bob
                debug_assert!(self.shared_secret.is_none());
                debug_assert!(self.ciphertext.is_none());
                debug_assert!(self.secret_key.is_some());

                let ciphertext = pqcrypto::kem::$name::Ciphertext::from_bytes(ciphertext).unwrap();
                let shared_secret = pqcrypto::kem::$name::decapsulate(&ciphertext, self.secret_key.as_ref().unwrap());
                self.ciphertext = Some(ciphertext);
                self.shared_secret = Some(shared_secret);
            }

            fn get_public_key(&self) -> &[u8] {
                self.public_key.as_bytes()
            }

            fn get_secret_key(&self) -> &[u8] {
                self.secret_key.as_ref().unwrap().as_bytes()
            }

            fn get_ciphertext(&self) -> &[u8] {
                self.ciphertext.as_ref().unwrap().as_bytes()
            }

            fn get_shared_secret(&self) -> &[u8] {
                self.shared_secret.as_ref().unwrap().as_bytes()
            }
        }
    };
}

pub(crate) mod function_pointers {
    use crate::PostQuantumType;
    use crate::algorithm_dictionary::ALGORITHM_COUNT;

    macro_rules! box_alice {
    ($constructor:expr) => {{
        #[inline(never)]
        fn alice_box_fn() -> Box<dyn PostQuantumType> {
            Box::new(($constructor)())
        }

        alice_box_fn
    }};
}

    macro_rules! box_bob {
    ($constructor:expr) => {{
        #[inline(never)]
        fn bob_box_fn(arr: &[u8]) -> Box<dyn PostQuantumType> {
            Box::new(($constructor)(arr))
        }

        bob_box_fn
    }};
}

    pub(crate) static ALICE_FP: [fn() -> Box<dyn PostQuantumType>; ALGORITHM_COUNT as usize] = [
        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_babybear::new_alice),
        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_babybearephem::new_alice),

        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_firesaber::new_alice),

        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_frodokem640aes::new_alice),
        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_frodokem640shake::new_alice),
        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_frodokem976aes::new_alice),
        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_frodokem976shake::new_alice),
        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_frodokem1344aes::new_alice),
        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_frodokem1344shake::new_alice),

        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_kyber512::new_alice),
        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_kyber768::new_alice),
        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_kyber1024::new_alice),
        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_kyber51290s::new_alice),
        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_kyber76890s::new_alice),
        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_kyber102490s::new_alice),

        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_ledakemlt12::new_alice),
        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_ledakemlt32::new_alice),
        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_ledakemlt52::new_alice),

        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_lightsaber::new_alice),

        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_mamabear::new_alice),
        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_mamabearephem::new_alice),

        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_mceliece348864::new_alice),
        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_mceliece348864f::new_alice),
        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_mceliece460896::new_alice),
        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_mceliece460896f::new_alice),
        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_mceliece6688128::new_alice),
        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_mceliece6688128f::new_alice),
        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_mceliece6960119::new_alice),
        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_mceliece6960119f::new_alice),
        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_mceliece8192128::new_alice),
        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_mceliece8192128f::new_alice),

        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_newhope512cca::new_alice),
        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_newhope512cpa::new_alice),
        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_newhope1024cca::new_alice),
        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_newhope1024cpa::new_alice),

        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_ntruhps2048509::new_alice),
        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_ntruhps2048677::new_alice),
        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_ntruhps4096821::new_alice),
        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_ntruhrss701::new_alice),

        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_papabear::new_alice),
        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_papabearephem::new_alice),

        box_alice!(crate::post_quantum_structs::PostQuantumAlgorithmData_saber::new_alice)
    ];

    pub(crate) static BOB_FP: [fn(&[u8]) -> Box<dyn PostQuantumType>; ALGORITHM_COUNT as usize] = [
        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_babybear::new_bob),
        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_babybearephem::new_bob),

        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_firesaber::new_bob),

        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_frodokem640aes::new_bob),
        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_frodokem640shake::new_bob),
        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_frodokem976aes::new_bob),
        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_frodokem976shake::new_bob),
        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_frodokem1344aes::new_bob),
        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_frodokem1344shake::new_bob),

        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_kyber512::new_bob),
        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_kyber768::new_bob),
        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_kyber1024::new_bob),
        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_kyber51290s::new_bob),
        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_kyber76890s::new_bob),
        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_kyber102490s::new_bob),

        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_ledakemlt12::new_bob),
        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_ledakemlt32::new_bob),
        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_ledakemlt52::new_bob),

        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_lightsaber::new_bob),

        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_mamabear::new_bob),
        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_mamabearephem::new_bob),

        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_mceliece348864::new_bob),
        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_mceliece348864f::new_bob),
        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_mceliece460896::new_bob),
        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_mceliece460896f::new_bob),
        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_mceliece6688128::new_bob),
        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_mceliece6688128f::new_bob),
        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_mceliece6960119::new_bob),
        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_mceliece6960119f::new_bob),
        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_mceliece8192128::new_bob),
        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_mceliece8192128f::new_bob),

        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_newhope512cca::new_bob),
        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_newhope512cpa::new_bob),
        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_newhope1024cca::new_bob),
        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_newhope1024cpa::new_bob),

        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_ntruhps2048509::new_bob),
        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_ntruhps2048677::new_bob),
        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_ntruhps4096821::new_bob),
        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_ntruhrss701::new_bob),

        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_papabear::new_bob),
        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_papabearephem::new_bob),

        box_bob!(crate::post_quantum_structs::PostQuantumAlgorithmData_saber::new_bob)
    ];
}

/// A set of auto generated structs corresponding to one of many possible encryption schemes
pub(crate) mod post_quantum_structs {
    use pqcrypto::traits::kem::*;
    use super::PostQuantumType;

    create_struct!(PostQuantumAlgorithmData_babybear, babybear);
    create_struct!(PostQuantumAlgorithmData_babybearephem, babybearephem);

    create_struct!(PostQuantumAlgorithmData_firesaber, firesaber);

    create_struct!(PostQuantumAlgorithmData_frodokem640aes, frodokem640aes);
    create_struct!(PostQuantumAlgorithmData_frodokem640shake, frodokem640shake);
    create_struct!(PostQuantumAlgorithmData_frodokem976aes, frodokem976aes);
    create_struct!(PostQuantumAlgorithmData_frodokem976shake, frodokem976shake);
    create_struct!(PostQuantumAlgorithmData_frodokem1344aes, frodokem1344aes);
    create_struct!(PostQuantumAlgorithmData_frodokem1344shake, frodokem1344shake);

    create_struct!(PostQuantumAlgorithmData_kyber512, kyber512);
    create_struct!(PostQuantumAlgorithmData_kyber768, kyber768);
    create_struct!(PostQuantumAlgorithmData_kyber1024, kyber1024);
    create_struct!(PostQuantumAlgorithmData_kyber51290s, kyber51290s);
    create_struct!(PostQuantumAlgorithmData_kyber76890s, kyber76890s);
    create_struct!(PostQuantumAlgorithmData_kyber102490s, kyber102490s);

    create_struct!(PostQuantumAlgorithmData_ledakemlt12, ledakemlt12);
    create_struct!(PostQuantumAlgorithmData_ledakemlt32, ledakemlt32);
    create_struct!(PostQuantumAlgorithmData_ledakemlt52, ledakemlt52);

    create_struct!(PostQuantumAlgorithmData_lightsaber, lightsaber);

    create_struct!(PostQuantumAlgorithmData_mamabear, mamabear);
    create_struct!(PostQuantumAlgorithmData_mamabearephem, mamabearephem);

    create_struct!(PostQuantumAlgorithmData_mceliece348864, mceliece348864);
    create_struct!(PostQuantumAlgorithmData_mceliece348864f, mceliece348864f);
    create_struct!(PostQuantumAlgorithmData_mceliece460896, mceliece460896);
    create_struct!(PostQuantumAlgorithmData_mceliece460896f, mceliece460896f);
    create_struct!(PostQuantumAlgorithmData_mceliece6688128, mceliece6688128);
    create_struct!(PostQuantumAlgorithmData_mceliece6688128f, mceliece6688128f);
    create_struct!(PostQuantumAlgorithmData_mceliece6960119, mceliece6960119);
    create_struct!(PostQuantumAlgorithmData_mceliece6960119f, mceliece6960119f);
    create_struct!(PostQuantumAlgorithmData_mceliece8192128, mceliece8192128);
    create_struct!(PostQuantumAlgorithmData_mceliece8192128f, mceliece8192128f);

    create_struct!(PostQuantumAlgorithmData_newhope512cca, newhope512cca);
    create_struct!(PostQuantumAlgorithmData_newhope512cpa, newhope512cpa);
    create_struct!(PostQuantumAlgorithmData_newhope1024cca, newhope1024cca);
    create_struct!(PostQuantumAlgorithmData_newhope1024cpa, newhope1024cpa);

    create_struct!(PostQuantumAlgorithmData_ntruhps2048509, ntruhps2048509);
    create_struct!(PostQuantumAlgorithmData_ntruhps2048677, ntruhps2048677);
    create_struct!(PostQuantumAlgorithmData_ntruhps4096821, ntruhps4096821);

    create_struct!(PostQuantumAlgorithmData_ntruhrss701, ntruhrss701);

    create_struct!(PostQuantumAlgorithmData_papabear, papabear);
    create_struct!(PostQuantumAlgorithmData_papabearephem, papabearephem);
    create_struct!(PostQuantumAlgorithmData_saber, saber);
}