// use c_kzg::{Blob, KzgSettings, BYTES_PER_BLOB};

use std::mem::MaybeUninit;

use kzg::{
    eip_4844::{
        Blob, CKZGSettings, BYTES_PER_BLOB, BYTES_PER_G1, BYTES_PER_G2, C_KZG_RET_OK,
        FIELD_ELEMENTS_PER_BLOB, TRUSTED_SETUP_NUM_G2_POINTS,
    },
    KZGSettings,
};
use risc0_zkvm::{default_prover, ExecutorEnv};
mod guest;
mod kzg_b;
mod trusted_setup_points;
use cycle_counter::{exec, CycleCounter, Metrics};

use rust_kzg_blst::{
    eip_4844::{kzg_settings_to_rust, load_trusted_setup},
    types::kzg_settings::FsKZGSettings,
};
use trusted_setup_points::{KzgErrors, NUM_G2_POINTS};
pub struct Job {}

impl CycleCounter for Job {
    const NAME: &'static str = "hello-world";
    const METHOD_ELF: &'static [u8] = kzg_b::KZG_B_ELF;

    fn run() -> Metrics {
        let (kzg_setting, blob) = gen_input();
        let env = ExecutorEnv::builder()
            .write(&kzg_setting)
            .unwrap()
            .write(&blob)
            .unwrap()
            .build()
            .unwrap();
        exec(Self::NAME, Self::METHOD_ELF, env)
    }
}

fn main() {
    cycle_counter::init_logging();
    Job::run();
}

// fn gen_input() -> (KzgSettings, Blob) {
//     let kzg_setting = KzgSettings::load_trusted_setup(
//         &trusted_setup_points::G1_POINTS.0,
//         &trusted_setup_points::G2_POINTS.0,
//     )
//     .unwrap();
//     let blob = Blob::new(&[1u8; BYTES_PER_BLOB]);
//     (kzg_setting, blob)
// }

fn gen_input() -> (FsKZGSettings, Blob) {
    let c_kzg_setting = load_trusted_setup_helper(
        &trusted_setup_points::G1_POINTS.0,
        &trusted_setup_points::G2_POINTS.0,
    )
    .unwrap();
    let kzg_setting = kzg_settings_to_rust(&c_kzg_setting).unwrap();
    let blob = Blob {
        bytes: [1u8; BYTES_PER_BLOB],
    };
    (kzg_setting, blob)
}

fn load_trusted_setup_helper(
    g1_bytes: &[[u8; BYTES_PER_G1]],
    g2_bytes: &[[u8; BYTES_PER_G2]],
) -> Result<CKZGSettings, KzgErrors> {
    if g1_bytes.len() != FIELD_ELEMENTS_PER_BLOB {
        return Err(KzgErrors::ParseError);
    }
    if g2_bytes.len() != NUM_G2_POINTS {
        return Err(KzgErrors::ParseError);
    }
    let mut kzg_settings = MaybeUninit::<CKZGSettings>::uninit();
    unsafe {
        if load_trusted_setup(
            kzg_settings.as_mut_ptr(),
            g1_bytes.as_ptr().cast(),
            g1_bytes.len(),
            g2_bytes.as_ptr().cast(),
            g2_bytes.len(),
        ) != C_KZG_RET_OK
        {
            return Err(KzgErrors::NotValidFile);
        }
        Ok(kzg_settings.assume_init())
    }
}

//  RISC0_DEV_MODE=1 RISC0_PROVER=local RUST_LOG=info  cargo run --bin driver --release
