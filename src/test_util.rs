use ark_bn254::Bn254;
use arkworks_circuits::setup::{
    common::{Leaf, MixerProof},
    mixer::{
        setup_leaf_with_privates_raw_x5_5, setup_leaf_x5_5, setup_proof_x5_5, MixerProverSetup,
    },
};
use arkworks_utils::utils::common::Curve;

// wasm-utils dependencies
use wasm_utils::{
    proof::{generate_proof_js, JsProofInput, MixerProofInput, ProofInput},
    types::{Backend, Curve as WasmCurve},
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Element(pub [u8; 32]);

impl Element {
    fn to_bytes(&self) -> &[u8] {
        &self.0
    }

    fn from_bytes(input: &[u8]) -> Self {
        let mut buf = [0u8; 32];
        buf.copy_from_slice(input);
        Self(buf)
    }
}

type Bn254Fr = ark_bn254::Fr;

pub fn setup_environment(curve: Curve) -> (Vec<u8>, Vec<u8>) {
    match curve {
        Curve::Bn254 => {
            let pk_bytes = include_bytes!(
                "../protocol-substrate-fixtures/mixer/bn254/x5/proving_key_uncompressed.bin"
            );
            let vk_bytes =
                include_bytes!("../protocol-substrate-fixtures/mixer/bn254/x5/verifying_key.bin");

            (pk_bytes.to_vec(), vk_bytes.to_vec())
        }
        Curve::Bls381 => {
            unimplemented!()
        }
    }
}

pub fn setup_zk_circuit(
    curve: Curve,
    recipient_bytes: Vec<u8>,
    relayer_bytes: Vec<u8>,
    pk_bytes: Vec<u8>,
    fee_value: u128,
    refund_value: u128,
) -> (
    Vec<u8>, // proof bytes
    Element, // root
    Element, // nullifier_hash
    Element, // leaf
) {
    let rng = &mut ark_std::test_rng();

    match curve {
        Curve::Bn254 => {
            // fit inputs to the curve.
            let Leaf {
                secret_bytes,
                nullifier_bytes,
                leaf_bytes,
                nullifier_hash_bytes,
            } = setup_leaf_x5_5::<Bn254Fr, _>(curve, rng).unwrap();

            let leaves = vec![leaf_bytes.clone()];
            let index = 0;
            let proof = setup_proof_x5_5::<Bn254, _>(
                curve,
                secret_bytes,
                nullifier_bytes,
                leaves,
                index,
                recipient_bytes.clone(),
                relayer_bytes.clone(),
                fee_value,
                refund_value,
                pk_bytes,
                rng,
            )
            .unwrap();

            println!("nullifier_hash: {:?}", proof.nullifier_hash_raw);
            println!("root: {:?}", proof.root_raw);
            println!("recipient_bytes: {:?}", recipient_bytes);
            println!("relayer_bytes: {:?}", relayer_bytes);
            println!("fee_bytes: {:?}", fee_value);
            println!("refund_bytes: {:?}", refund_value);

            let leaf_element = Element::from_bytes(&leaf_bytes);
            let nullifier_hash_element = Element::from_bytes(&nullifier_hash_bytes);
            let root_element = Element::from_bytes(&proof.root_raw);

            (
                proof.proof,
                root_element,
                nullifier_hash_element,
                leaf_element,
            )
        }
        Curve::Bls381 => {
            unimplemented!()
        }
    }
}

pub fn setup_wasm_utils_zk_circuit(
    curve: Curve,
    recipient_bytes: Vec<u8>,
    relayer_bytes: Vec<u8>,
    pk_bytes: Vec<u8>,
    fee_value: u128,
    refund_value: u128,
) -> (
    Vec<u8>, // proof bytes
    Element, // root
    Element, // nullifier_hash
    Element, // leaf
) {
    match curve {
        Curve::Bn254 => {
            let note_secret = "7e0f4bfa263d8b93854772c94851c04b3a9aba38ab808a8d081f6f5be9758110b7147c395ee9bf495734e4703b1f622009c81712520de0bbd5e7a10237c7d829bf6bd6d0729cca778ed9b6fb172bbb12b01927258aca7e0a66fd5691548f8717";
            let raw = hex::decode(&note_secret).unwrap();
            let secret = &raw[0..32];
            let nullifier = &raw[32..64];
            let leaf = setup_leaf_with_privates_raw_x5_5::<Bn254Fr>(
                curve,
                secret.to_vec(),
                nullifier.to_vec(),
            )
            .unwrap();

            let leaves = vec![leaf.leaf_bytes];

            let mixer_proof_input = MixerProofInput {
                exponentiation: 5,
                width: 5,
                curve: WasmCurve::Bn254,
                backend: Backend::Arkworks,
                secrets: secret.to_vec(),
                nullifier: nullifier.to_vec(),
                recipient: recipient_bytes.clone(),
                relayer: relayer_bytes.clone(),
                pk: pk_bytes,
                refund: refund_value,
                fee: fee_value,
                chain_id: 0,
                leaves,
                leaf_index: 0,
            };
            let js_proof_inputs = JsProofInput {
                inner: ProofInput::Mixer(mixer_proof_input),
            };
            let proof = generate_proof_js(js_proof_inputs).unwrap();

            println!("nullifier_hash: {:?}", proof.nullifier_hash);
            println!("root: {:?}", proof.root);
            println!("recipient_bytes: {:?}", recipient_bytes);
            println!("relayer_bytes: {:?}", relayer_bytes);
            println!("fee_bytes: {:?}", fee_value);
            println!("refund_bytes: {:?}", refund_value);

            let root_element = Element::from_bytes(&proof.root);
            let nullifier_hash_element = Element::from_bytes(&proof.nullifier_hash);
            let leaf_element = Element::from_bytes(&proof.leaf);

            (
                proof.proof,
                root_element,
                nullifier_hash_element,
                leaf_element,
            )
        }
        Curve::Bls381 => {
            unimplemented!()
        }
    }
}

/// Truncate and pad 256 bit slice in reverse
pub fn truncate_and_pad_reverse(t: &[u8]) -> Vec<u8> {
    let mut truncated_bytes = t[12..].to_vec();
    truncated_bytes.extend_from_slice(&[0u8; 12]);
    truncated_bytes
}
