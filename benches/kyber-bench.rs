use criterion::{criterion_group, criterion_main, Criterion};

use kybe_rs::*;

pub fn bench_kyber512_pke(c: &mut Criterion) {
    let pke = kyber512pke();
    let m = ByteArray::random(32);
    let r = ByteArray::random(32);

    let mut group = c.benchmark_group("Kyber 512 PKE");

    let (sk, pk) = pke.keygen();
    let enc = pke.encrypt(&pk, &m, r.clone());
    let _dec = pke.decrypt(&sk, &enc);

    group.bench_function("Keygen", |b| b.iter(|| pke.keygen()));
    group.bench_function("Encryption", |b| b.iter(|| pke.encrypt(&pk, &m, r.clone())));
    group.bench_function("Decryption", |b| b.iter(|| pke.decrypt(&sk, &enc)));

    group.finish();
}

pub fn bench_kyber512_kem(c: &mut Criterion) {
    let kem = kyber512kem();

    let mut group = c.benchmark_group("Kyber 512 KEM");
    let (sk, pk) = kem.keygen();
    let (ctx, _shk) = kem.encaps(&pk);
    let _shk2 = kem.decaps(&ctx, &sk);

    group.bench_function("Keygen", |b| b.iter(|| kem.keygen()));
    group.bench_function("Encapsulation", |b| b.iter(|| kem.encaps(&pk)));
    group.bench_function("Decapsulation", |b| b.iter(|| kem.decaps(&ctx, &sk)));

    group.finish();
}

pub fn bench_kyber768_pke(c: &mut Criterion) {
    let pke = kyber768pke();
    let m = ByteArray::random(32);
    let r = ByteArray::random(32);

    let mut group = c.benchmark_group("Kyber 768 PKE");

    let (sk, pk) = pke.keygen();
    let enc = pke.encrypt(&pk, &m, r.clone());
    let _dec = pke.decrypt(&sk, &enc);

    group.bench_function("Keygen", |b| b.iter(|| pke.keygen()));
    group.bench_function("Encryption", |b| b.iter(|| pke.encrypt(&pk, &m, r.clone())));
    group.bench_function("Decryption", |b| b.iter(|| pke.decrypt(&sk, &enc)));

    group.finish();
}

pub fn bench_kyber768_kem(c: &mut Criterion) {
    let kem = kyber768kem();

    let mut group = c.benchmark_group("Kyber 768 KEM");
    let (sk, pk) = kem.keygen();
    let (ctx, _shk) = kem.encaps(&pk);
    let _shk2 = kem.decaps(&ctx, &sk);

    group.bench_function("Keygen", |b| b.iter(|| kem.keygen()));
    group.bench_function("Encapsulation", |b| b.iter(|| kem.encaps(&pk)));
    group.bench_function("Decapsulation", |b| b.iter(|| kem.decaps(&ctx, &sk)));
}

pub fn bench_kyber1024_pke(c: &mut Criterion) {
    let pke = kyber1024pke();
    let m = ByteArray::random(32);
    let r = ByteArray::random(32);

    let mut group = c.benchmark_group("Kyber 1024 PKE");

    let (sk, pk) = pke.keygen();
    let enc = pke.encrypt(&pk, &m, r.clone());
    let _dec = pke.decrypt(&sk, &enc);

    group.bench_function("Keygen", |b| b.iter(|| pke.keygen()));
    group.bench_function("Encryption", |b| b.iter(|| pke.encrypt(&pk, &m, r.clone())));
    group.bench_function("Decryption", |b| b.iter(|| pke.decrypt(&sk, &enc)));

    group.finish();
}

pub fn bench_kyber1024_kem(c: &mut Criterion) {
    let kem = kyber1024kem();

    let mut group = c.benchmark_group("Kyber 1024 KEM");
    let (sk, pk) = kem.keygen();
    let (ctx, _shk) = kem.encaps(&pk);
    let _shk2 = kem.decaps(&ctx, &sk);

    group.bench_function("Keygen", |b| b.iter(|| kem.keygen()));
    group.bench_function("Encapsulation", |b| b.iter(|| kem.encaps(&pk)));
    group.bench_function("Decapsulation", |b| b.iter(|| kem.decaps(&ctx, &sk)));
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
    name = k1024;
    config = config();
    targets = bench_kyber1024_pke, bench_kyber1024_kem
}

criterion_group! {
    name = benches;
    config = config();
    targets = bench_kyber512_pke, bench_kyber512_kem, bench_kyber768_pke, bench_kyber768_kem, bench_kyber1024_pke, bench_kyber1024_kem
}

criterion_main!(benches);
