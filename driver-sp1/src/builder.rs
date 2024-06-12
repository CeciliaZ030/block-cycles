use raiko_pipeline::{
    parse_metadata, rerun_if_changed, CommandBuilder, GuestMetadata, Metadata, Pipeline,
};
use std::path::PathBuf;

fn main() {
    let pipeline = Sp1Pipeline::new("../guest-sp1", "release");
    // pipeline.bins(&["guest", "keccak", "secp256k1"], "../guest-sp1/elf");
    pipeline.bins(&["kzg-z"], "../guest-sp1/elf");

    pipeline.tests(&["guest"], "../guest-sp1/elf");
}

pub struct Sp1Pipeline {
    pub meta: Metadata,
    pub profile: String,
}

impl Pipeline for Sp1Pipeline {
    fn new(root: &str, profile: &str) -> Self {
        raiko_pipeline::ROOT_DIR.get_or_init(|| PathBuf::from(root));
        Sp1Pipeline {
            meta: parse_metadata(root),
            profile: profile.to_string(),
        }
    }

    fn builder(&self) -> CommandBuilder {
        CommandBuilder::new(&self.meta, "riscv32im-succinct-zkvm-elf", "succinct")
            .rust_flags(&[
                "passes=loweratomic",
                "link-arg=-Ttext=0x00200800",
                "panic=abort",
            ])
            .cc_compiler("gcc".into())
            .c_flags(&[
                "/opt/riscv/bin/riscv32-unknown-elf-gcc",
                "-march=rv32im",
                "-mstrict-align",
                "-falign-functions=2",
            ])
            .custom_args(&["--ignore-rust-version"])
    }

    fn bins(&self, names: &[&str], dest: &str) {
        rerun_if_changed(&[]);
        let bins = self.meta.get_bins(names);
        let builder = self.builder();
        let executor = builder.build_command(&self.profile, &bins);
        println!(
            "executor: \n   ${:?}\ntargets: \n   {:?}",
            executor.cmd, executor.artifacts
        );
        if executor.artifacts.is_empty() {
            panic!("No artifacts to build");
        }
        executor
            .execute()
            .expect("Execution failed")
            .sp1_placement(dest)
            .expect("Failed to export Sp1 artifacts");
    }

    fn tests(&self, names: &[&str], dest: &str) {
        rerun_if_changed(&[]);
        let tests = self.meta.get_tests(names);
        let builder = self.builder();
        let executor = builder.test_command(&self.profile, &tests);
        println!(
            "executor: \n   ${:?}\ntargets: \n   {:?}",
            executor.cmd, executor.artifacts
        );
        if executor.artifacts.is_empty() {
            panic!("No artifacts to build");
        }
        executor
            .execute()
            .expect("Execution failed")
            .sp1_placement(dest)
            .expect("Failed to export Sp1 artifacts");
    }
}
