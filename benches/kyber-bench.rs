use criterion::{criterion_group, criterion_main, Criterion};

use kybe_rs::{
    kyber_ccakem_dec, kyber_ccakem_enc, kyber_ccakem_key_gen, kyber_cpapke_dec, kyber_cpapke_enc,
    kyber_cpapke_key_gen, ByteArray, KyberParams,
};

pub fn bench_kyber512_pke(c: &mut Criterion) {
    let params = KyberParams::kyber512();
    let m = ByteArray::random(32);
    let r = ByteArray::random(32);

    let mut group = c.benchmark_group("Kyber 512 PKE");

    let (sk, pk) = kyber_cpapke_key_gen(params);
    let enc = kyber_cpapke_enc(params, &pk, &m, r.clone());
    let _dec = kyber_cpapke_dec(params, &sk, &enc);

    group.bench_function("Keygen", |b| b.iter(|| kyber_cpapke_key_gen(params)));
    group.bench_function("Encryption", |b| {
        b.iter(|| kyber_cpapke_enc(params, &pk, &m, r.clone()))
    });
    group.bench_function("Decryption", |b| {
        b.iter(|| kyber_cpapke_dec(params, &sk, &enc))
    });

    group.finish();
}

pub fn bench_kyber512_kem(c: &mut Criterion) {
    let params = KyberParams::kyber512();

    let mut group = c.benchmark_group("Kyber 512 KEM");
    let (sk, pk) = kyber_ccakem_key_gen(params);
    let (ctx, _shk) = kyber_ccakem_enc(params, &pk);
    let _shk2 = kyber_ccakem_dec(params, &ctx, &sk);

    group.bench_function("Keygen", |b| b.iter(|| kyber_ccakem_key_gen(params)));
    group.bench_function("Encapsulation", |b| {
        b.iter(|| kyber_ccakem_enc(params, &pk))
    });
    group.bench_function("Decapsulation", |b| {
        b.iter(|| kyber_ccakem_dec(params, &ctx, &sk))
    });

    group.finish();
}

pub fn bench_kyber768_pke(c: &mut Criterion) {
    let params = KyberParams::kyber768();
    let m = ByteArray::random(32);
    let r = ByteArray::random(32);

    let mut group = c.benchmark_group("Kyber 768 PKE");

    let (sk, pk) = kyber_cpapke_key_gen(params);
    let enc = kyber_cpapke_enc(params, &pk, &m, r.clone());
    let _dec = kyber_cpapke_dec(params, &sk, &enc);

    group.bench_function("Keygen", |b| b.iter(|| kyber_cpapke_key_gen(params)));
    group.bench_function("Encryption", |b| {
        b.iter(|| kyber_cpapke_enc(params, &pk, &m, r.clone()))
    });
    group.bench_function("Decryption", |b| {
        b.iter(|| kyber_cpapke_dec(params, &sk, &enc))
    });

    group.finish();
}

pub fn bench_kyber768_kem(c: &mut Criterion) {
    let params = KyberParams::kyber768();

    let mut group = c.benchmark_group("Kyber 768 KEM");
    let (sk, pk) = kyber_ccakem_key_gen(params);
    let (ctx, _shk) = kyber_ccakem_enc(params, &pk);
    let _shk2 = kyber_ccakem_dec(params, &ctx, &sk);

    group.bench_function("Keygen", |b| b.iter(|| kyber_ccakem_key_gen(params)));
    group.bench_function("Encapsulation", |b| {
        b.iter(|| kyber_ccakem_enc(params, &pk))
    });
    group.bench_function("Decapsulation", |b| {
        b.iter(|| kyber_ccakem_dec(params, &ctx, &sk))
    });

    group.finish();
}

pub fn config() -> Criterion {
    Criterion::default().sample_size(100)
}

criterion_group! {
    name = k512;
    config = config();
    targets = bench_kyber512_pke, bench_kyber512_kem
}

criterion_group! {
    name = k768;
    config = config();
    targets = bench_kyber768_pke, bench_kyber768_kem
}

criterion_group! {
    name = benches;
    config = config();
    targets = bench_kyber512_pke, bench_kyber512_kem, bench_kyber768_pke, bench_kyber768_kem
}

criterion_main!(benches);
