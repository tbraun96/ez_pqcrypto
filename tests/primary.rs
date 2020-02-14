#[cfg(test)]
mod tests {
    use ez_pqcrypto::{PostQuantumContainer, algorithm_dictionary};

    #[test]
    fn default() {
        run(None)
    }

    fn run(algorithm: Option<u8>) {
        let algorithm = algorithm.unwrap_or(algorithm_dictionary::BABYBEAR);
        // Alice wants to share data with Bob. She first creates a PostQuantumContainer
        let mut alice_container = PostQuantumContainer::new_alice(Some(algorithm));
        // Then, alice sender her public key to Bob. She must also send the byte value of algorithm_dictionary::BABYBEAR to him
        let alice_public_key = alice_container.get_public_key();
        let algorithm_byte_value = alice_container.get_algorithm_idx();
        //
        // Then, Bob gets the public key. To process it, he must create a PostQuantumContainer for himself
        let bob_container = PostQuantumContainer::new_bob(algorithm_byte_value, alice_public_key).unwrap();
        // Internally, this computes the CipherText. The next step is to send this CipherText back over to alice
        let bob_ciphertext = bob_container.get_ciphertext().unwrap();
        //
        // Next, alice received Bob's ciphertext. She must now run an update on her internal data in order to get the shared secret
        alice_container.alice_on_receive_ciphertext(bob_ciphertext).unwrap();

        assert_eq!(alice_container.get_shared_secret().unwrap(), bob_container.get_shared_secret().unwrap());
    }

    #[test]
    fn test_10() {
        for algorithm in 0..10 {
            println!("About to test {}", algorithm);
            run(Some(algorithm))
        }
    }

    #[test]
    fn test_serialize_deserialize() {
        for algorithm in 0..10 {
            println!("Test algorithm {}", algorithm);
            let mut alice_container = PostQuantumContainer::new_alice(Some(algorithm));
            let bob_container = PostQuantumContainer::new_bob(algorithm, alice_container.get_public_key()).unwrap();
            alice_container.alice_on_receive_ciphertext(bob_container.get_ciphertext().unwrap()).unwrap();

            let serialized = alice_container.serialize_to_vector().unwrap();
            let _ = PostQuantumContainer::deserialize_from_bytes(serialized).unwrap();
        }
    }
}