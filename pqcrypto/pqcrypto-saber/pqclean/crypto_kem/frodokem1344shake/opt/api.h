#ifndef PQCLEAN_FRODOKEM1344SHAKE_OPT_API_H
#define PQCLEAN_FRODOKEM1344SHAKE_OPT_API_H

#include <stddef.h>
#include <stdint.h>

#define PQCLEAN_FRODOKEM1344SHAKE_OPT_CRYPTO_SECRETKEYBYTES  43088     // sizeof(s) + CRYPTO_PUBLICKEYBYTES + 2*PARAMS_N*PARAMS_NBAR + BYTES_PKHASH
#define PQCLEAN_FRODOKEM1344SHAKE_OPT_CRYPTO_PUBLICKEYBYTES  21520     // sizeof(seed_A) + (PARAMS_LOGQ*PARAMS_N*PARAMS_NBAR)/8
#define PQCLEAN_FRODOKEM1344SHAKE_OPT_CRYPTO_BYTES              32
#define PQCLEAN_FRODOKEM1344SHAKE_OPT_CRYPTO_CIPHERTEXTBYTES 21632     // (PARAMS_LOGQ*PARAMS_N*PARAMS_NBAR)/8 + (PARAMS_LOGQ*PARAMS_NBAR*PARAMS_NBAR)/8

#define PQCLEAN_FRODOKEM1344SHAKE_OPT_CRYPTO_ALGNAME "FrodoKEM-1344-SHAKE"

int PQCLEAN_FRODOKEM1344SHAKE_OPT_crypto_kem_keypair(uint8_t *pk, uint8_t *sk);

int PQCLEAN_FRODOKEM1344SHAKE_OPT_crypto_kem_enc(uint8_t *ct, uint8_t *ss, const uint8_t *pk);

int PQCLEAN_FRODOKEM1344SHAKE_OPT_crypto_kem_dec(uint8_t *ss, const uint8_t *ct, const uint8_t *sk);

#endif