use kybe_rs::{kyber_cpapke_dec, kyber_cpapke_enc, kyber_cpapke_key_gen, ByteArray, KyberParams};

fn main() {
    let params = KyberParams::kyber512();
    let m = ByteArray::random(32);
    let r = ByteArray::random(32);

    let (sk, pk) = kyber_cpapke_key_gen(params);
    let enc = kyber_cpapke_enc(params, &pk, &m, r.clone());
    let dec = kyber_cpapke_dec(params, &sk, &enc);

    println!("{:?}", dec);
}
