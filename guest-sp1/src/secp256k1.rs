// use alloy_primitives::{keccak256, ChainId, TxKind, B256, U256};
#![no_main]
harness::entrypoint!(main);
fn main() {
    sign_recover();
}

#[sp1_derive::cycle_tracker]
pub fn sign_recover() {
    let secp: Secp256k1<secp256k1::All> = Secp256k1::new();

    let seckey = [
        59, 148, 11, 85, 134, 130, 61, 253, 2, 174, 59, 70, 27, 180, 51, 107, 94, 203, 174, 253,
        102, 39, 170, 146, 46, 252, 4, 143, 236, 12, 136, 28,
    ];
    let pubkey = PublicKey::from_slice(&[
        2, 29, 21, 35, 7, 198, 183, 43, 14, 208, 65, 139, 14, 112, 205, 128, 231, 245, 41, 91, 141,
        134, 245, 114, 45, 63, 82, 19, 251, 210, 57, 79, 54,
    ])
    .unwrap();
    let msg = b"This is some message";

    println!("cycle-tracker-start: sign");
    let signature = sign_recovery(&secp, msg, seckey).unwrap();
    println!("cycle-tracker-end: sign");

    let (recovery_id, serialize_sig) = signature.serialize_compact();

    println!("cycle-tracker-start: recover");
    let pubkey = recover(&secp, msg, serialize_sig, recovery_id.to_i32() as u8);
    println!("cycle-tracker-end: recover");

    // assert_eq!(recover(&secp, msg, serialize_sig, recovery_id.to_i32() as u8), Ok(pubkey));
}

use hashes::{sha256, Hash};
use secp256k1::{ecdsa, Error, Message, PublicKey, Secp256k1, SecretKey, Signing, Verification};

fn recover<C: Verification>(secp: &Secp256k1<C>, msg: &[u8], sig: [u8; 64], recovery_id: u8)
/* -> Result<PublicKey, Error> */
{
    let msg = sha256::Hash::hash(msg);
    let msg = Message::from_digest_slice(msg.as_ref()).unwrap();
    let id = ecdsa::RecoveryId::from_i32(recovery_id as i32).unwrap();
    let sig = ecdsa::RecoverableSignature::from_compact(&sig, id).unwrap();

    let _ = secp.recover_ecdsa(&msg, &sig);
}

fn sign_recovery<C: Signing>(
    secp: &Secp256k1<C>,
    msg: &[u8],
    seckey: [u8; 32],
) -> Result<ecdsa::RecoverableSignature, Error> {
    let msg = sha256::Hash::hash(msg);
    let msg = Message::from_digest_slice(msg.as_ref())?;
    let seckey = SecretKey::from_slice(&seckey)?;
    Ok(secp.sign_ecdsa_recoverable(&msg, &seckey))
}
