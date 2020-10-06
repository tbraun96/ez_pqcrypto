# ez_pqcrypto
A cryptographic container built in rust that handles post-quantum key exchange, encryption/decryption, and in-place "protection" of plaintext packets (less calls to allocator). Effectively, using the protection functions ensures that
packets sent over the wire maintain confidentiality, authenticity, and integrity. Furthermore, this crate provides protection against replay attacks by appending a u64 to the end of each payload and is thereafter encrypted as part of the payload ciphertext.

There are two classes of features in this crate. The first relates to the encryption/decryption algorithm used:

* aes
* chacha20

By default, ``aes`` is selected. The aes feature will imply that AES-GCM-SIV (protection against nonce-misuse) is compiled. The use of ``chacha20`` will imply XChaCha20-Poly1305 is used. Select one

The second class of features relates to the anti-replay attack container:

* ordered
* unordered

By default, the ``ordered`` feature is used. When this is used, a atomically-backed compare and swap (CAS) operation is used to keep track of packets as they flow outbound and inbound. If
you are using TCP, use ordered. If, however, you are using an unordered (yet reliable) protocol, then use ``unordered``. When using ``unordered``, a fixed-capacity circular ring buffer is used
to keep track of a neighborhood of packets to ensure packets are not necessarily expected in serial order.

This crate uses pqcrypto/pqclean for the underlying cryptographic primitives. Currently, only FIRESABER is selected from pqcrypto. Everything in the ``./pqcrypto`` folder is not my work.

Example of post-quantum key exchange:

```rust
let algorithm_byte_value = algorithm_dictionary::FIRESABER;
// Alice wants to share data with Bob. She first creates a PostQuantumContainer
let mut alice_container = PostQuantumContainer::new_alice(algorithm_byte_value);
// Then, alice sender her public key to Bob. She must also send the byte value of algorithm_dictionary::FIRESABER to him (the only one currently used)
let alice_public_key = alice_container.get_public_key();
//
// Then, Bob gets the public key. To process it, he must create a PostQuantumContainer for himself
let bob_container = PostQuantumContainer::new_bob(algorithm_byte_value, alice_public_key);
// Internally, this computes the CipherText. The next step is to send this CipherText back over to alice
let bob_ciphertext = bob_container.get_ciphertext();
//
// Next, alice received Bob's ciphertext. She must now run an update on her internal data in order to get the shared secret
alice_container.alice_on_receive_ciphertext(bob_ciphertext);

assert_eq!(alice_container.get_shared_secret(), bob_container.get_shared_secret());
```

Furthermore, supports serialization/deserialization
