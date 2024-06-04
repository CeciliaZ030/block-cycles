#![no_main]
harness::entrypoint!(main, tests);

use raiko_lib::{
    builder::{BlockBuilderStrategy, TaikoStrategy},
    input::GuestInput,
};
use revm_precompile::zk_op::ZkOperation;

pub mod zk_op;
use zk_op::Sp1Operator;

pub mod mem;
pub use mem::*;

fn main() {
    revm_precompile::zk_op::ZKVM_OPERATOR.get_or_init(|| Box::new(Sp1Operator {}));
    revm_precompile::zk_op::ZKVM_OPERATIONS
        .set(Box::new(vec![
            ZkOperation::Bn128Add,
            ZkOperation::Bn128Mul,
            ZkOperation::Sha256,
            // ZkOperation::Secp256k1,
        ]))
        .expect("Failed to set ZkvmOperations");

    let mut input: GuestInput = sp1_zkvm::io::read::<GuestInput>();
    input.taiko.skip_verify_blob = true;

    let build_result = TaikoStrategy::build_from(&input);
}

harness::zk_suits!(
    pub mod tests {
        #[test]
        pub fn test_build_from_mock_input() {
            // Todo: impl mock input for static unit test
            assert_eq!(1, 1);
        }
    }
);
