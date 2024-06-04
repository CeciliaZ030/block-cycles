use revm_precompile::{bn128::ADD_INPUT_LEN, utilities::right_pad, zk_op::ZkvmOperator, Error};
use revm_primitives::keccak256;
use sha2_v0_10_8 as sp1_sha2;
use sp1_zkvm::precompiles::{bn254::Bn254, utils::AffinePoint};

#[derive(Debug)]
pub struct Sp1Operator;

impl ZkvmOperator for Sp1Operator {
    fn bn128_run_add(&self, input: &[u8]) -> Result<[u8; 64], Error> {
        let input = right_pad::<ADD_INPUT_LEN>(input);
        let mut p = be_bytes_to_point(&input[..64]);
        let q = be_bytes_to_point(&input[64..]);
        p.add_assign(&q);
        Ok(point_to_be_bytes(p))
    }

    fn bn128_run_mul(&self, input: &[u8]) -> Result<[u8; 64], Error> {
        let input = right_pad::<96>(input);
        let _output = [0u8; 64];

        let mut p = be_bytes_to_point(&input[..64]);

        let k: [u32; 8] = input[64..]
            .to_owned()
            .chunks_exact(4)
            .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap()))
            .collect::<Vec<u32>>()
            .try_into()
            .map_err(|e| Error::ZkvmOperation("Input point processing failed".to_string()))?;

        p.mul_assign(&k);
        Ok(point_to_be_bytes(p))
    }

    fn bn128_run_pairing(&self, _input: &[u8]) -> Result<bool, Error> {
        unreachable!()
    }

    fn blake2_run(&self, _input: &[u8]) -> Result<[u8; 64], Error> {
        unreachable!()
    }

    fn sha256_run(&self, input: &[u8]) -> Result<[u8; 32], Error> {
        use sp1_sha2::Digest;
        Ok(sp1_sha2::Sha256::digest(input).into())
    }

    fn ripemd160_run(&self, _input: &[u8]) -> Result<[u8; 32], Error> {
        unreachable!()
    }

    fn modexp_run(&self, _base: &[u8], _exp: &[u8], _modulus: &[u8]) -> Result<Vec<u8>, Error> {
        unreachable!()
    }

    fn secp256k1_ecrecover(
        &self,
        sig: &[u8; 64],
        recid: u8,
        msg: &[u8; 32],
    ) -> Result<[u8; 32], Error> {
        let mut sig_id = [0u8; 65];
        sig_id[..64].copy_from_slice(sig);
        sig_id[64] = recid;
        let recovered_key = sp1_precompiles::secp256k1::ecrecover(&sig_id, msg)
            .map_err(|e| Error::ZkvmOperation(e.to_string()))?;

        let mut hash = keccak256(&recovered_key[1..]);

        // truncate to 20 bytes
        hash[..12].fill(0);
        Ok(*hash)
    }
}

#[inline]
fn be_bytes_to_point(input: &[u8]) -> AffinePoint<Bn254, 16> {
    assert!(input.len() == 64, "Input length must be 64 bytes");
    let mut x: [u8; 32] = input[..32].try_into().unwrap();
    let mut y: [u8; 32] = input[32..].try_into().unwrap();
    x.reverse();
    y.reverse();

    // Init AffinePoint for sp1
    AffinePoint::<Bn254, 16>::from(x, y)
}

#[inline]
fn point_to_be_bytes(p: AffinePoint<Bn254, 16>) -> [u8; 64] {
    let p = p.to_le_bytes();
    let mut x = [0u8; 32];
    let mut y = [0u8; 32];

    x.copy_from_slice(&p[..32].iter().rev().copied().collect::<Vec<_>>());
    y.copy_from_slice(&p[32..].iter().rev().copied().collect::<Vec<_>>());

    ([x, y]).concat().try_into().unwrap()
}
