use kybe_rs;

#[test]
fn encode_decode_poly() {
    use kybe_rs::{decode_to_poly, encode_poly, Poly3329};
    let original = Poly3329::<256>::from_vec(vec![Default::default(); 256], 256);
    let encoded = encode_poly(original.clone(), 12);
    let decoded = decode_to_poly(encoded, 12);
    assert!(decoded == original);
}

#[test]
fn compress_decompress_poly() {
    use kybe_rs::{compress_poly, decompress_poly, Poly3329};
    let original = Poly3329::<256>::from_vec(vec![Default::default(); 256], 256);
    let encoded = compress_poly(original.clone(), 12, 3329);
    let decoded = decompress_poly(encoded, 12, 3329);
    assert!(decoded == original);
}

#[test]
fn pke_keygen_cpapke() {
    let params = kybe_rs::KyberParams::kyber512();
    kybe_rs::kyber_cpapke_key_gen(params);
}

#[test]
fn kem_keygen_ccakem() {
    let params = kybe_rs::KyberParams::kyber512();
    kybe_rs::kyber_ccakem_key_gen(params);
}

#[test]
fn encrypt_then_decrypt_cpapke() {
    let params = kybe_rs::KyberParams::kyber512();
    let (sk, pk) = kybe_rs::kyber_cpapke_key_gen(params);

    let m = kybe_rs::ByteArray::random(32);
    let r = kybe_rs::ByteArray::random(32);

    let enc = kybe_rs::kyber_cpapke_enc(params, &pk, &m, r);
    let dec = kybe_rs::kyber_cpapke_dec(params, &sk, &enc);

    assert_eq!(m, dec);
}

#[test]
fn encapsulate_then_decapsulate_ccakem() {
    let params = kybe_rs::KyberParams::kyber512();

    let (sk, pk) = kybe_rs::kyber_ccakem_key_gen(params);
    let (ctx, shk) = kybe_rs::kyber_ccakem_enc(params, &pk);
    let shk2 = kybe_rs::kyber_ccakem_dec(params, &ctx, &sk);
    assert_eq!(shk, shk2);
}
