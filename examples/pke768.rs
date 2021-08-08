use kybe_rs::{kyber768pke, ByteArray};

fn main() {
    let pke = kyber768pke();
    let m = ByteArray::random(32);
    let r = ByteArray::random(32);

    let (sk, pk) = pke.keygen();
    let enc = pke.encrypt(&pk, &m, r.clone());
    let dec = pke.decrypt(&sk, &enc);

    println!("{:?}", dec);
}
