use kybe_rs::{kyber_ccakem_dec, kyber_ccakem_enc, kyber_ccakem_key_gen, KyberParams};

fn main() {
    let params = KyberParams::kyber512();

    let (sk, pk) = kyber_ccakem_key_gen(params);
    let (ctx, _shk) = kyber_ccakem_enc(params, &pk);
    let shk2 = kyber_ccakem_dec(params, &ctx, &sk);

    println!("{:?}", shk2);
}
