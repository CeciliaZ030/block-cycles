extern crate rust_kzg_zkcrypto;
use kzg::eip_4844::blob_to_kzg_commitment_rust;
use rust_kzg_zkcrypto::{kzg_proofs::KZGSettings, kzg_types::ZFr};

fn main() {
    println!("cycle-tracker-start: load kzg_setting");
    let kzg_setting = sp1_zkvm::io::read::<KZGSettings>();
    println!("cycle-tracker-end: load kzg_setting");

    println!("cycle-tracker-start: read blob");
    let blob = sp1_zkvm::io::read::<Vec<ZFr>>();
    println!("cycle-tracker-end: read blob");

    println!("cycle-tracker-start: blob_to_kzg_commitment_rust");
    blob_to_kzg_commitment_rust(&blob, &kzg_setting).unwrap();
    println!("cycle-tracker-end: blob_to_kzg_commitment_rust");
}
