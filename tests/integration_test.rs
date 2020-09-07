use kybe_rs;

#[test]
fn pke_keygen_call() {
    let params = (2, 0, 0);
    kybe_rs::kyber_cpapke_key_gen(params);
}

#[test]
fn encrypt_then_decrypt() {
    let params = (2, 0, 0);
    let (sk, pk) = kybe_rs::kyber_cpapke_key_gen(params);

    let m = kybe_rs::ByteArray::random();
    let r = kybe_rs::ByteArray::random();

    let enc = kybe_rs::kyber_cpapke_enc(&pk, &m, r);
    let dec = kybe_rs::kyber_cpapke_dec(&sk, &enc);

    assert_eq!(m, dec);
}

#[test]
fn kem_keygen_call() {
    kybe_rs::kyber_ccakem_key_gen();
}

#[test]
fn encapsulate_then_decapsulate() {
    let (sk, pk) = kybe_rs::kyber_ccakem_key_gen();
    let (ctx, shk) = kybe_rs::kyber_ccakem_enc(&pk);
    let shk2 = kybe_rs::kyber_ccakem_dec(&ctx, &sk);
    assert_eq!(shk, shk2);
}
