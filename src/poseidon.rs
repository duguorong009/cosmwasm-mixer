pub use self::poseidon::{Poseidon};

mod hasher {
    use ark_crypto_primitives::{Error, CRH as CRHTrait};
    use ark_ff::{BigInteger, PrimeField};
    use arkworks_gadgets::poseidon::CRH;
    use ark_std::{marker::PhantomData, vec::Vec};
    use arkworks_utils::poseidon::PoseidonParameters;
    pub struct ArkworksPoseidonHasher<F: PrimeField>(PhantomData<F>);

    impl<F: PrimeField> ArkworksPoseidonHasher<F> {
        pub fn hash(input: &[u8], param_bytes: &[u8]) -> Result<Vec<u8>, Error> {
            let params = PoseidonParameters::<F>::from_bytes(param_bytes)?;
            let output: F = <CRH<F> as CRHTrait>::evaluate(&params, input)?;
            let value = output.into_repr().to_bytes_le();
            Ok(value)
        }
    }

    use ark_bn254::Fr as Bn254;
    pub type ArkworksPoseidonHasherBn254 = ArkworksPoseidonHasher<Bn254>;
}

pub mod poseidon {
    use crate::poseidon::hasher::{ArkworksPoseidonHasherBn254};

    #[derive(Debug, Clone)]
    pub struct Poseidon {
        hasher_params_width_3_bytes: Vec<u8>,
        hasher_params_width_4_bytes: Vec<u8>,
        hasher_params_width_5_bytes: Vec<u8>,
    }

    /// The hash error types.
    #[derive(Debug)]
    pub enum Error {
        /// Returned if there is an error hashing
        HashError,
        /// Invalid hash width
        InvalidHashInputWidth,
    }

    /// The Hash result type.
    pub type Result<T> = core::result::Result<T, Error>;

    impl Poseidon {
        pub fn hash(&self, inputs: Vec<[u8; 32]>) -> Result<[u8; 32]> {
            let num_inputs = inputs.len();
            let mut packed_inputs = Vec::new();
            for inp in inputs {
                packed_inputs.extend_from_slice(&inp);
            }

            let hash_result = match num_inputs {
                3 => ArkworksPoseidonHasherBn254::hash(&packed_inputs, &self.hasher_params_width_3_bytes),
                4 => ArkworksPoseidonHasherBn254::hash(&packed_inputs, &self.hasher_params_width_4_bytes),
                5 => ArkworksPoseidonHasherBn254::hash(&packed_inputs, &self.hasher_params_width_5_bytes),
                _ => return Err(Error::InvalidHashInputWidth),
            };

            hash_result.map(|h| {
                let mut hash_result = [0u8; 32];
                hash_result.copy_from_slice(&h);
                hash_result
            })
            .map_err(|_| Error::HashError)
        }
    }
}