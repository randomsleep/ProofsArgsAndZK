use crate::{
    polynomial::{
        univariate_polynomial::{Polynomial, PolynomialTrait},
        MultivariatePolynomialOracle,
    },
    randomness_oracle::RandomnessOracle,
};
use ark_ff::fields::Field;

pub struct Verifier<F: Field> {
    l: u64,
    function_oracle: Box<dyn MultivariatePolynomialOracle<F>>,
    randomness_oracle: Box<dyn RandomnessOracle<F>>,

    // the claimed value in each round ci = gi(0) + gi(1) = g_{i-1}(r_{i-1})
    c: Vec<F>,
    // The generated challenges.
    r: Vec<F>,
}

impl<F: Field> Verifier<F> {
    pub fn new(
        l: u64,
        function_oracle: Box<dyn MultivariatePolynomialOracle<F>>,
        randomness_oracle: Box<dyn RandomnessOracle<F>>,
    ) -> Self {
        Self {
            l,
            function_oracle,
            randomness_oracle,
            c: Vec::new(),
            r: Vec::new(),
        }
    }

    pub fn verify(&mut self, round: u64, gi: Polynomial<F>) -> bool {
        assert!(round >= 1 && round <= self.l);
        assert!(round == self.current_round().checked_add(1).unwrap());
        // ensure deg(gi) == deg(f[X_i])
        if gi.deg() != self.function_oracle.deg(round) {
            return false;
        }
        let ci = gi.evaluate(F::zero()) + gi.evaluate(F::one());
        if round == 1 {
            // first round, store the claimed value as H
            self.c.push(ci);
        } else {
            // check gi(0) + gi(1) = g_{i-1}(r_{i-1})
            if ci != *self.c.last().unwrap() {
                return false;
            }
        }

        // generate the challenge
        let ri = self.randomness_oracle.next_random(gi.coeff());
        self.r.push(ri);
        let next_ci = gi.evaluate(ri);
        self.c.push(next_ci);

        // the last round, check g(r1, r2, ..., rl) = f(r)
        if round == self.l {
            let f_r = self.function_oracle.evaluate(&self.r);
            if f_r != *self.c.last().unwrap() {
                return false;
            }
        }

        true
    }

    pub fn challenge(&self, round: u64) -> F {
        assert!(round >= 1 && round <= self.r.len() as u64);
        *self.r.get(round as usize - 1).unwrap()
    }

    pub fn current_round(&self) -> u64 {
        self.r.len() as u64
    }
}
