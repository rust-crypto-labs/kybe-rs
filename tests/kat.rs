use ron::de::from_reader;
use serde::Deserialize;
use std::fs::File;
use hex::decode;

use kybe_rs::*;

#[derive(Debug, Deserialize)]
struct Test {
    seed: String,
    pk: String,
    sk: String,
    ss: String,
    ct: String,
}

#[derive(Debug, Deserialize)]
struct KAT {
    kem_type: String,
    tests: Vec<Test>
}

#[test]
fn kat_kyber512() {
    let kem = kyber512kem();
    let f = File::open("./tests/PQCkemKAT_1632.ron").expect("Failed opening file");
    let kat: KAT = from_reader(f).expect("Failed parsing file");

    println!("Running KAT for {}", kat.kem_type);
    let mut i = 0;

    for test in kat.tests {
        println!("test #{}", i);
        let pk = ByteArray::from_bytes(decode(test.pk).expect("failed to decode").as_slice());
        let sk = ByteArray::from_bytes(decode(test.sk).expect("failed to decode").as_slice());
        let ss = ByteArray::from_bytes(decode(test.ss).expect("failed to decode").as_slice());
        let ct = ByteArray::from_bytes(decode(test.ct).expect("failed to decode").as_slice());
        let k_recovered = kem.decaps(&ct, &sk);
        assert_eq!(k_recovered, ss);
        i += 1;
    }
}


#[test]
fn kat_kyber768() {
    let kem = kyber768kem();
    let f = File::open("./tests/PQCkemKAT_2400.ron").expect("Failed opening file");
    let kat: KAT = from_reader(f).expect("Failed parsing file");

    println!("Running KAT for {}", kat.kem_type);

    for test in kat.tests {

        let pk = ByteArray::from_bytes(decode(test.pk).expect("failed to decode").as_slice());
        let sk = ByteArray::from_bytes(decode(test.sk).expect("failed to decode").as_slice());
        let ss = ByteArray::from_bytes(decode(test.ss).expect("failed to decode").as_slice());
        let ct = ByteArray::from_bytes(decode(test.ct).expect("failed to decode").as_slice());
        let (c, k) = kem.encaps(&pk);
        let k_recovered = kem.decaps(&c, &sk);
        assert_eq!(k, ss);
        assert_eq!(k_recovered, ss);
        assert_eq!(c, ct)
    }
}

#[test]
fn kat_kyber1024() {
    let kem = kyber1024kem();
    let f = File::open("./tests/PQCkemKAT_3168.ron").expect("Failed opening file");
    let kat: KAT = from_reader(f).expect("Failed parsing file");

    println!("Running KAT for {}", kat.kem_type);

    for test in kat.tests {

        let pk = ByteArray::from_bytes(decode(test.pk).expect("failed to decode").as_slice());
        let sk = ByteArray::from_bytes(decode(test.sk).expect("failed to decode").as_slice());
        let ss = ByteArray::from_bytes(decode(test.ss).expect("failed to decode").as_slice());
        let ct = ByteArray::from_bytes(decode(test.ct).expect("failed to decode").as_slice());
        let (c, k) = kem.encaps(&pk);
        let k_recovered = kem.decaps(&c, &sk);
        assert_eq!(k, ss);
        assert_eq!(k_recovered, ss);
        assert_eq!(c, ct)
    }
}