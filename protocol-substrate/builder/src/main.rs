use ark_ec::PairingEngine;
use arkworks_circuits::setup::anchor::AnchorProverSetup;
use arkworks_circuits::setup::mixer::MixerProverSetup;
use arkworks_circuits::setup::vanchor::VAnchorProverSetup;
use arkworks_utils::utils::common::{setup_params_x5_2, setup_params_x5_3, setup_params_x5_5, setup_params_x5_4, Curve};
use arkworks_circuits::prelude::ark_crypto_primitives::SNARK;
use ark_serialize::CanonicalSerialize;
use ark_groth16::{Groth16, ProvingKey, VerifyingKey};
use ark_std::test_rng;
use ark_bn254::Bn254;
use std::fs::write;
use std::env::current_dir;

pub const N: usize = 30;
pub const M: usize = 2;
type AnchorProverSetupBn254_30<F> = AnchorProverSetup<F, M, N>;
type MixerProverSetupBn254_30<F> = MixerProverSetup<F, N>;
type VAnchorProverSetupBn254_30_2x2<F> = VAnchorProverSetup<F, N, M, 2, 2>;

fn save_keys<E: PairingEngine>(proving_key: ProvingKey<E>, verifying_key: VerifyingKey<E>, path: &str) {
	let mut pk = Vec::new();
	let mut vk = Vec::new();
	proving_key.serialize(&mut pk).unwrap();
	verifying_key.serialize(&mut vk).unwrap();

	let mut pk_uncompressed = Vec::new();
	let mut vk_uncompressed = Vec::new();
	proving_key.serialize_uncompressed(&mut pk_uncompressed).unwrap();
	verifying_key.serialize_uncompressed(&mut vk_uncompressed).unwrap();

	let current_path = current_dir().unwrap();
	write(format!("{}/{}/proving_key.bin", current_path.display(), path), pk).unwrap();
	write(format!("{}/{}/verifying_key.bin", current_path.display(), path), vk).unwrap();
	write(format!("{}/{}/proving_key_uncompressed.bin", current_path.display(), path), pk_uncompressed).unwrap();
	write(format!("{}/{}/verifying_key_uncompressed.bin", current_path.display(), path), vk_uncompressed).unwrap();
}

fn generate_mixer_keys<E: PairingEngine>(curve: Curve) {
	let mut rng = test_rng();
	let params3 = setup_params_x5_3::<E::Fr>(curve);
	let params5 = setup_params_x5_5::<E::Fr>(curve);

	let prover = MixerProverSetupBn254_30::new(params3, params5);
	let (circuit, ..) = prover.setup_random_circuit(&mut rng).unwrap();

	let (proving_key, verifying_key) = Groth16::<E>::circuit_specific_setup(circuit, &mut rng).unwrap();

	save_keys(proving_key, verifying_key, "../mixer/bn254/x5");
}

fn generate_anchor_keys<E: PairingEngine>(curve: Curve) {
	let mut rng = test_rng();
	let params3 = setup_params_x5_3::<E::Fr>(curve);
	let params4 = setup_params_x5_4::<E::Fr>(curve);

	let prover = AnchorProverSetupBn254_30::new(params3, params4);
	let (circuit, ..) = prover.setup_random_circuit(&mut rng).unwrap();

	let (proving_key, verifying_key) = Groth16::<E>::circuit_specific_setup(circuit, &mut rng).unwrap();

	save_keys(proving_key, verifying_key, "../fixed-anchor/bn254/x5");
}

fn generate_vanchor_keys<E: PairingEngine>(curve: Curve) {
	let mut rng = test_rng();
	let params2 = setup_params_x5_2::<E::Fr>(curve);
	let params3 = setup_params_x5_3::<E::Fr>(curve);
	let params4 = setup_params_x5_4::<E::Fr>(curve);
	let params5 = setup_params_x5_5::<E::Fr>(curve);

	let prover = VAnchorProverSetupBn254_30_2x2::new(params2, params3, params4, params5);
	let circuit = prover.setup_random_circuit(&mut rng).unwrap();

	let (proving_key, verifying_key) = Groth16::<E>::circuit_specific_setup(circuit, &mut rng).unwrap();

	save_keys(proving_key, verifying_key, "../vanchor/bn254/x5");
}

fn main() {
	generate_mixer_keys::<Bn254>(Curve::Bn254);
	generate_anchor_keys::<Bn254>(Curve::Bn254);
	generate_vanchor_keys::<Bn254>(Curve::Bn254);
}
