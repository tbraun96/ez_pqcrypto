#[cfg(test)]
mod tests {
    use ez_pqcrypto::{PostQuantumContainer, algorithm_dictionary};

    #[test]
    fn default() {
        /*
        std::thread::Builder::new()
            .stack_size(64*1024*1024)
            .spawn(run)
            .unwrap()
            .join()
            .unwrap();
            */
        run()
    }

    fn run() {
        // Alice wants to share data with Bob. She first creates a PostQuantumContainer
        let mut alice_container = PostQuantumContainer::new_alice(algorithm_dictionary::BABYBEAR);
        // Then, alice sender her public key to Bob. She must also send the byte value of algorithm_dictionary::BABYBEAR to him
        let alice_public_key = alice_container.get_public_key();
        let algorithm_byte_value = algorithm_dictionary::BABYBEAR;
        //
        // Then, Bob gets the public key. To process it, he must create a PostQuantumContainer for himself
        let bob_container = PostQuantumContainer::new_bob(algorithm_byte_value, alice_public_key);
        // Internally, this computes the CipherText. The next step is to send this CipherText back over to alice
        let bob_ciphertext = bob_container.get_ciphertext();
        //
        // Next, alice received Bob's ciphertext. She must now run an update on her internal data in order to get the shared secret
        alice_container.alice_on_receive_ciphertext(bob_ciphertext);

        assert_eq!(alice_container.get_shared_secret(), bob_container.get_shared_secret());
    }
}