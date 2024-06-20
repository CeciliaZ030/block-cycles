extern crate rust_kzg_blst;
use kzg::{
    eip_4844::{
        blob_to_kzg_commitment_rust, evaluate_polynomial_in_evaluation_form, hash_to_bls_field,
        Blob,
    },
    Poly,
};
use risc0_zkvm::guest::env;
use rust_kzg_blst::{
    eip_4844::{blob_to_kzg_commitment, deserialize_blob_rust},
    types::kzg_settings::FsKZGSettings,
};

fn main() {
    println!("Hello, world!");
    let mut kzg_settings = env::read::<FsKZGSettings>();
    let mut blob = env::read::<Blob>();
    let blob_fields =
        deserialize_blob_rust(&blob).expect("Failed to deserialize blob to field elements");

    // println!("cycle-tracker-start: ? read blob fields directly");
    // let blob_fields = sp1_zkvm::io::read::<Vec<ArkFr>>();
    // println!("cycle-tracker-end: ? read blob fields directly");

    // println!("cycle-tracker-start: PolyData::from_coeffs");
    // let poly = PolyData::from_coeffs(&blob_fields);
    // println!("cycle-tracker-end: PolyData::from_coeffs");

    // println!("cycle-tracker-start: tiny-keccak");
    // // should be tiny-keccak
    // let blob_hash = keccak(&blob.bytes);
    // println!("cycle-tracker-end: tiny-keccak");

    // println!("cycle-tracker-start: hash_to_bls_field");
    // let x: ArkFr = hash_to_bls_field(&blob_hash);
    // println!("cycle-tracker-end: hash_to_bls_field");

    // println!("cycle-tracker-start: evaluate_polynomial_in_evaluation_form");
    // let y = evaluate_polynomial_in_evaluation_form(&poly, &x, &kzg_setting)
    //     .expect("Failed to evaluate polynomial at hashed point");
    // println!("cycle-tracker-end: evaluate_polynomial_in_evaluation_form");

    // // println!("cycle-tracker-start: blob_to_kzg_commitment_rust");
    blob_to_kzg_commitment_rust(&blob_fields, &kzg_settings).unwrap();
    // println!("cycle-tracker-end: blob_to_kzg_commitment_rust");
}

// fn keccak(data: &[u8]) -> [u8; 32] {
//     let mut hasher = Keccak::v256();
//     hasher.update(data);
//     let mut res = [0u8; 32];
//     hasher.finalize(&mut res);
//     res
// }
