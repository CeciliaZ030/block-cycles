#![no_main]
// use c_kzg::{Blob, KzgCommitment, KzgSettings, BYTES_PER_BLOB};
use risc0_zkvm::guest::env;
harness::entrypoint!(main, tests);
pub mod mem;
pub use mem::*;

// #[sp1_derive::cycle_tracker]
fn main() {
    // println!("Hello, world!");
    // let mut ks = env::read::<Vec<u8>>();
    // let kzg_settings = KzgSettings::from_u8_slice(ks.as_mut_slice());
    // let mut blob = [0u8; BYTES_PER_BLOB];
    // env::read_slice(blob.as_mut());

    // let kzg_commit =
    //     KzgCommitment::blob_to_kzg_commitment(&Blob::from_bytes(&blob).unwrap(), &kzg_settings);
}

// harness::zk_suits!(
//     pub mod tests {
//         #[test]
//         pub fn test_build_from_mock_input() {
//             // Todo: impl mock input for static unit test
//             assert_eq!(1, 1);
//         }
//     }
// );
