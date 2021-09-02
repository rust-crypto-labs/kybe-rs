use kybe_rs::kyber512kem;

fn main() {
    let kem = kyber512kem();

    let (sk, pk) = kem.keygen();
    let (ctx, _shk) = kem.encaps(&pk);
    let shk2 = kem.decaps(&ctx, &sk);

    println!("{:?}", shk2);
}
