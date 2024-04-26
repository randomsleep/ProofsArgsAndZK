use ark_ff::{
    field_hashers::{DefaultFieldHasher, HashToField},
    fields::Field,
};
use sha2::Sha256;

use super::RandomnessOracle;

/// Fiat-Shamir heuristic for generating randomness.
#[derive(Default, Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub struct FiatShamir<F> {
    current_state: F,
}

impl<F: Field> FiatShamir<F> {
    pub fn new() -> Self {
        Self {
            current_state: F::zero(),
        }
    }
}

impl<F: Field> RandomnessOracle<F> for FiatShamir<F> {
    fn next_random(&mut self, input: &[F]) -> F {
        let hasher = <DefaultFieldHasher<Sha256> as HashToField<F>>::new(&[]);
        let mut input_bytes = Vec::new();
        self.current_state
            .serialize_uncompressed(&mut input_bytes)
            .unwrap();
        for f in input.iter() {
            f.serialize_uncompressed(&mut input_bytes).unwrap();
        }
        self.current_state = *hasher
            .hash_to_field(input_bytes.as_slice(), 1)
            .first()
            .unwrap();

        self.current_state
    }
}
