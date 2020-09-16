use kybe_rs;

#[test]
fn pke_keygen_call() {
    let params = kybe_rs::KyberParams::kyber512();
    kybe_rs::kyber_cpapke_key_gen(params);
}

#[test]
fn encrypt_then_decrypt() {
    let params = kybe_rs::KyberParams::kyber512();
    let (sk, pk) = kybe_rs::kyber_cpapke_key_gen(params);

    let M_SIZE = 4;
    let R_SIZE = 4;

    let m = kybe_rs::ByteArray::random(M_SIZE);
    let r = kybe_rs::ByteArray::random(R_SIZE);

    let enc = kybe_rs::kyber_cpapke_enc(&pk, &m, r);
    let dec = kybe_rs::kyber_cpapke_dec(&sk, &enc);

    assert_eq!(m, dec);
}

#[test]
fn kem_keygen_call() {

    let params = kybe_rs::KyberParams::kyber512();

    kybe_rs::kyber_ccakem_key_gen(params);
}

#[test]
fn encapsulate_then_decapsulate() {

    let params = kybe_rs::KyberParams::kyber512();

    let (sk, pk) = kybe_rs::kyber_ccakem_key_gen(params);
    let (ctx, shk) = kybe_rs::kyber_ccakem_enc(&pk);
    let shk2 = kybe_rs::kyber_ccakem_dec(&ctx, &sk);
    assert_eq!(shk, shk2);
}
