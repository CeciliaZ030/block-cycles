use risc0_zkvm::{default_prover, ExecutorEnv};
mod guest;

use cycle_counter::{exec, CycleCounter, Metrics};

pub struct Job {}

impl CycleCounter for Job {
    const NAME: &'static str = "hello-world";
    const METHOD_ELF: &'static [u8] = guest::GUEST_ELF;

    fn run() -> Metrics {
        let env = ExecutorEnv::builder().build().unwrap();

        exec(Self::NAME, Self::METHOD_ELF, env)
    }
}

fn main() {
    cycle_counter::init_logging();
    Job::run();
}

//  RISC0_DEV_MODE=1 RISC0_PROVER=local RUST_LOG=info  cargo run --bin driver --release
