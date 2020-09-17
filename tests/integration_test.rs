use kybe_rs;

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
