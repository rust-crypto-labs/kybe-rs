use kybe_rs::kyber1024kem;

fn main() {
    let kem = kyber1024kem();

    let (sk, pk) = kem.keygen();
    let (ctx, _shk) = kem.encaps(&pk);
    let shk2 = kem.decaps(&ctx, &sk);

    println!("{:?}", shk2);
}
