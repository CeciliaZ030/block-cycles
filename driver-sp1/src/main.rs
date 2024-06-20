use std::cell::UnsafeCell;

use kzg::{
    eip_4844::{evaluate_polynomial_in_evaluation_form, hash_to_bls_field, Blob, BYTES_PER_BLOB},
    Poly,
};
use raiko_lib::primitives::keccak::keccak;
use rust_kzg_arkworks::{
    eip_4844::deserialize_blob_rust, kzg_proofs::KZGSettings, kzg_types::ArkFr, utils::PolyData,
};
use serde::{Deserialize, Serialize};
use serde_json;

fn main() {
    println!("Hello, world!");

    let kzg_setting =
        rust_kzg_arkworks::eip_4844::load_trusted_setup_filename_rust("./trusted_setup.txt")
            .unwrap();

    let s = serde_json::to_vec(&kzg_setting).unwrap();
    let kzg_setting: KZGSettings = serde_json::from_slice(&s).unwrap();

    let blob = [6u8; kzg::eip_4844::BYTES_PER_BLOB];

    let blob_fields = deserialize_blob_rust(&Blob { bytes: blob })
        .expect("Failed to deserialize blob to field elements");
    let poly = PolyData::from_coeffs(&blob_fields);

    let blob_hash = keccak(blob);
    let x: ArkFr = hash_to_bls_field(&blob_hash);

    let y = evaluate_polynomial_in_evaluation_form(&poly, &x, &kzg_setting)
        .expect("Failed to evaluate polynomial at hashed point");
}
