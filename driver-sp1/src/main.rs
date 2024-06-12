use kzg::eip_4844::{Blob, BYTES_PER_BLOB};
use rust_kzg_zkcrypto::eip_4844::deserialize_blob_rust;

fn main() {
    println!("Hello, world!");

    let blob = deserialize_blob_rust(&Blob {
        bytes: [6u8; kzg::eip_4844::BYTES_PER_BLOB],
    })
    .unwrap();
}
