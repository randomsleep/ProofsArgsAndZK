pub mod fiat_shamir;
pub mod private_coin;

pub trait RandomnessOracle<F> {
    fn next_random(&mut self, input: &[F]) -> F; // return a random field element based on the input (and possibly internal state)
}
