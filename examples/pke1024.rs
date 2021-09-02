use kybe_rs::{kyber1024pke, ByteArray};

fn main() {
    let pke = kyber1024pke();
    let m = ByteArray::random(32);
    let r = ByteArray::random(32);

    let (sk, pk) = pke.keygen();
    let enc = pke.encrypt(&pk, &m, r.clone());
    let dec = pke.decrypt(&sk, &enc);

    println!("{:?}", dec);
}
