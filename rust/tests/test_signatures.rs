use hacl_star::prelude::*;

#[test]
fn test_p256_signature() {
    let msg = b"Message to sign";
    let (sk, pk) = signature_key_gen(SignatureMode::P256).unwrap();
    let sig = sign(
        SignatureMode::P256,
        DigestAlgorithm::Sha256,
        &sk,
        msg,
        &p256_ecdsa_random_nonce().unwrap(),
    )
    .unwrap();
    let verified = verify(SignatureMode::P256, DigestAlgorithm::Sha256, &pk, &sig, msg).unwrap();
    assert!(verified);
}
