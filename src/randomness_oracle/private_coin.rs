use ark_ff::fields::Field;
use rand::prelude::*;

use super::RandomnessOracle;

#[derive(Default)]
pub struct PrivateCoinRandomnessOracle<R: RngCore + CryptoRng + Default> {
    rng: R,
}

impl<F: Field, R: RngCore + CryptoRng + Default> RandomnessOracle<F>
    for PrivateCoinRandomnessOracle<R>
{
    fn next_random(&mut self, _input: &[F]) -> F {
        F::rand(&mut self.rng)
    }
}
