use ark_ff::fields::Field;

use crate::{
    polynomial::{
        multilinear_polynomial::MultilinearPolynomial, univariate_polynomial::Polynomial,
    },
    randomness_oracle::{fiat_shamir::FiatShamir, RandomnessOracle},
};

use super::{prover::Prover, verifier::Verifier};

pub struct SumCheck<F: Field> {
    l: u64,
    f: MultilinearPolynomial<F>,
}

pub struct Proof<F: Field>(Vec<Polynomial<F>>);

/// SumCheck protocol with Fiat-Shamir heuristic.
impl<F: Field> SumCheck<F> {
    pub fn new(l: u64, f: MultilinearPolynomial<F>) -> Self {
        Self { l, f }
    }

    pub fn sum(&self) -> F {
        self.f.sum()
    }

    // Generate proof for the sum of the multilinear polynomial.
    pub fn prove(&self) -> Proof<F> {
        let mut prover = Prover::new(self.l, self.f.clone());
        let mut proof = vec![];
        let mut r = None;
        let mut randomness_oracle = FiatShamir::new();
        for i in 1..=self.l {
            let gi = prover.prove(i, r);
            r = Some(randomness_oracle.next_random(gi.coeff()));
            proof.push(gi);
        }

        Proof(proof)
    }

    // Verify the proof.
    pub fn verify(&self, proof: Proof<F>) -> bool {
        let fiat_shamir = Box::new(FiatShamir::new());
        let mut verifier = Verifier::new(self.l, Box::new(self.f.clone()), fiat_shamir);
        for i in 1..=self.l {
            let gi = proof.0[i as usize - 1].clone();
            if !verifier.verify(i, gi) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::randomness_oracle::private_coin::PrivateCoinRandomnessOracle;

    use super::*;
    use ark_ff::{Fp64, MontBackend, MontConfig};
    use rand::rngs::ThreadRng;

    #[derive(MontConfig)]
    #[modulus = "5"]
    #[generator = "3"]
    pub struct FqConfig;
    pub type Fq = Fp64<MontBackend<FqConfig, 1>>;

    #[test]
    fn sum_check_interactive() {
        let l = 2;
        let f = vec![Fq::from(1), Fq::from(2), Fq::from(1), Fq::from(4)];
        let polynomial = MultilinearPolynomial::new(2, f.clone());

        let mut prover = Prover::new(l, polynomial.clone());
        assert!(prover.sum() == f.iter().sum());

        let rand_oracle = Box::<PrivateCoinRandomnessOracle<ThreadRng>>::default();
        let mut verifier = Verifier::new(l, Box::new(polynomial.clone()), rand_oracle);

        let mut r: Option<_> = None;
        for round in 1..=l {
            println!("Round {}", round);
            let gi = prover.prove(round, r);
            println!("Prover: g_{} = {}", round, gi);
            assert!(verifier.verify(round, gi));
            r = Some(verifier.challenge(round));
            println!("Verifier: r_{} = {}", round, r.unwrap());
        }
        assert!(prover.current_round() == l);
        assert!(verifier.current_round() == l);
    }

    #[test]
    fn sum_check() {
        let l = 2;
        let f = vec![Fq::from(1), Fq::from(2), Fq::from(1), Fq::from(4)];
        let polynomial = MultilinearPolynomial::new(2, f.clone());

        let sum_check = SumCheck::new(l, polynomial.clone());
        assert!(sum_check.sum() == f.iter().sum());

        let proof = sum_check.prove();
        assert!(sum_check.verify(proof));
    }
}
