use raiko_lib::input::GuestInput;
use sp1_sdk::{utils, ProverClient, SP1Stdin};

/// The ELF we want to execute inside the zkVM.
const ELF: &[u8] = include_bytes!("../../guest-sp1/elf/guest");

const KECCAK: &[u8] = include_bytes!("../../guest-sp1/elf/keccak");

const SECP256K1: &[u8] = include_bytes!("../../guest-sp1/elf/secp256k1");

const KZG_Z: &[u8] = include_bytes!("../../guest-sp1/elf/kzg-z");

const TEST: &[u8] = include_bytes!("../../guest-sp1/elf/test-guest");

// #[cfg(not(feature = "kzg"))]
fn main_() {
    // let args: Vec<String> = std::env::args().collect();
    // let input = PathBuf::from(format!("./{}.json", &args.get(1).expect("Input.json?")));
    // println!("{:?}", input);

    // Setup a tracer for logging.
    utils::setup_logger();

    // Create an input stream.
    let mut stdin = SP1Stdin::new();

    // let json = std::fs::read_to_string(input).unwrap();
    // let input: GuestInput = serde_json::from_str(&json).unwrap();
    // stdin.write(&input);

    let start = std::time::Instant::now();

    // Generate the proof for the given program.
    let client = ProverClient::new();
    let (pk, vk) = client.setup(KECCAK);
    let proof = client.prove(&pk, stdin).unwrap();

    let end = start.elapsed();
    println!("{:?}", end);

    // Verify proof.
    // client.verify(&proof, &vk).expect("verification failed");

    // // Save the proof.
    // proof
    //     .save("proof-with-pis.json")
    //     .expect("saving proof failed");

    // println!("successfully generated and verified proof for the program!")
}

//  SP1_PROVER=mock RUST_LOG=info cargo run --bin driver --release

// #[cfg(feature = "kzg")]
fn main() {
    use kzg::eip_4844::{Blob, BYTES_PER_BLOB};
    use rust_kzg_zkcrypto::eip_4844::deserialize_blob_rust;

    // Setup a tracer for logging.
    utils::setup_logger();

    let start = std::time::Instant::now();

    let kzg_setting =
        rust_kzg_zkcrypto::eip_4844::load_trusted_setup_filename_rust("./trusted_setup.txt")
            .unwrap();
    let end = start.elapsed();
    println!("Driver: load kzg data and precomputate setup {:?}", end);
    let blob = deserialize_blob_rust(&Blob {
        bytes: [6u8; BYTES_PER_BLOB],
    })
    .unwrap();

    // Create an input stream.
    let mut stdin = SP1Stdin::new();
    stdin.write(&kzg_setting);
    stdin.write(&blob);

    let start = std::time::Instant::now();

    // Generate the proof for the given program.
    let client = ProverClient::new();
    let (pk, vk) = client.setup(KZG_Z);
    let proof = client.prove(&pk, stdin).unwrap();

    let end = start.elapsed();
    println!("{:?}", end);
}
