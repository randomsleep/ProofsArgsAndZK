use crate::polynomial::{
    multilinear_polynomial::MultilinearPolynomial, univariate_polynomial::Polynomial,
};
use ark_ff::fields::Field;

/// The prover in the sum-check protocol.
pub struct Prover<F: Field> {
    // the number of variables
    l: u64,
    // the sum of which to prove
    multilinear_polynomial: MultilinearPolynomial<F>,
    // in round i, the f(r_1, r_2, ..., r_{i-1}, xi, x_{i+1}, x_l)
    current_f: MultilinearPolynomial<F>,
    // the gi that prover sends in each round
    g: Vec<Polynomial<F>>,
}

impl<F: Field> Prover<F> {
    pub fn new(l: u64, f: MultilinearPolynomial<F>) -> Self {
        Self {
            l,
            multilinear_polynomial: f.clone(),
            current_f: f.clone(),
            g: Vec::new(),
        }
    }

    pub fn sum(&self) -> F {
        self.multilinear_polynomial.sum()
    }

    pub fn current_round(&self) -> u64 {
        self.g.len() as u64
    }

    // Generate the polynomial gi(Xi) = Sum_x_{f(r_1, r_2, ..., r_{i-1}, Xi, x_{i+1}, x_l)}
    pub fn prove(&mut self, round: u64, r: Option<F>) -> Polynomial<F> {
        assert!(round >= 1 && round <= self.l);
        assert!(round == self.current_round().checked_add(1).unwrap());

        if round > 1 {
            self.current_f = self.current_f.fix_variable(r.unwrap());
        }

        let gi = Self::gen_g(&self.current_f);
        self.g.push(gi.clone());
        gi
    }

    // generate the polynomial g(X) = Sum_x_{f(X, x_2, , x_3, ..., x_l)}
    // The complexity is O(l*2^l)
    pub fn gen_g(multi_linear_polynomial: &MultilinearPolynomial<F>) -> Polynomial<F> {
        let g0 = multi_linear_polynomial.fix_variable(F::zero()).sum();
        let g1 = multi_linear_polynomial.fix_variable(F::one()).sum();
        Polynomial::new(vec![g0, g1 - g0])
    }
}
