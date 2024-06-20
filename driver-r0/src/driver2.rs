use c_kzg::{Blob, KzgSettings, BYTES_PER_BLOB};

use risc0_zkvm::{default_prover, ExecutorEnv};
use std::mem::MaybeUninit;
mod kzg_c;
mod trusted_setup_points2;
use cycle_counter::{exec, CycleCounter, Metrics};

use trusted_setup_points2::{KzgErrors, NUM_G2_POINTS};
pub struct Job {}

impl CycleCounter for Job {
    const NAME: &'static str = "hello-world";
    const METHOD_ELF: &'static [u8] = kzg_c::KZG_C_ELF;

    fn run() -> Metrics {
        let (kzg_setting, blob) = gen_input();
        let env = ExecutorEnv::builder()
            .write(&kzg_setting.to_bytes())
            .unwrap()
            .write_slice(&[1u8; BYTES_PER_BLOB])
            .build()
            .unwrap();
        exec(Self::NAME, Self::METHOD_ELF, env)
    }
}

fn main() {
    cycle_counter::init_logging();
    Job::run();
}

fn gen_input() -> (KzgSettings, Blob) {
    let kzg_setting = KzgSettings::load_trusted_setup(
        &trusted_setup_points2::G1_POINTS.0,
        &trusted_setup_points2::G2_POINTS.0,
    )
    .unwrap();
    let blob = Blob::new([1u8; BYTES_PER_BLOB]);
    (kzg_setting, blob)
}
