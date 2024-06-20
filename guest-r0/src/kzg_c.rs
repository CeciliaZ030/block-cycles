use c_kzg::{bindings::blob_to_kzg_commitment, Blob, KzgCommitment, KzgSettings, BYTES_PER_BLOB};
use risc0_zkvm::guest::env;
pub mod mem;
pub use mem::*;

fn main() {
    println!("Hello, world!");
    let mut kzg_settings = KzgSettings::from_u8_slice(env::read::<Vec<u8>>().as_mut_slice());
    let mut blob = [1u8; BYTES_PER_BLOB];
    env::read_slice(blob.as_mut());

    let commitment = KzgCommitment::blob_to_kzg_commitment(&Blob::new(blob), &kzg_settings);
    // println!("cycle-tracker-end: blob_to_kzg_commitment_rust");
}

// fn keccak(data: &[u8]) -> [u8; 32] {
//     let mut hasher = Keccak::v256();
//     hasher.update(data);
//     let mut res = [0u8; 32];
//     hasher.finalize(&mut res);
//     res
// }
