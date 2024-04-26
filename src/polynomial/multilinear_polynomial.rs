use ark_ff::fields::Field;

use super::MultivariatePolynomialOracle;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MultilinearPolynomial<F: Field> {
    l: u64,    // number of variables
    f: Vec<F>, // f[0] = f(0,0,0), f[1] = f(0,0,1), f[2] = f(0,1,0), f[3] = f(0,1,1), ...
}

impl<F: Field> MultilinearPolynomial<F> {
    pub fn new(l: u64, f: Vec<F>) -> Self {
        assert!(f.len() == 1 << l, "f length must be 2^l");
        Self { l, f }
    }

    // Evaluate the multilinear polynomial at point r, using cty11.
    pub fn evaluate_cty11(&self, r: &[F]) -> F {
        assert!(r.len() as u64 == self.l, "r length must be l");
        let mut fr = F::zero(); // f(r)
        for (w, &fw) in self.f.iter().enumerate() {
            let mut term = F::one();
            for (i, &ri) in r.iter().enumerate() {
                let wi = F::from((w as u64) >> (self.l - 1 - i as u64) & 1); // w's i-th bit
                term *= wi * ri + (F::one() - wi) * (F::one() - ri);
            }
            fr += fw * term;
        }
        fr
    }

    // f.fix_variable(r2, r3, ..., rl) = f(r1, r2, r3, ..., rl)
    // This is not optimal. The complexity is O(l*2^l).
    // pub fn fix_variable(&self, x: F) -> Self {
    //     assert!(self.l >= 1, "l must be at least 1");
    //     let new_l = self.l - 1;
    //     let mut new_f = vec![F::zero(); 1 << new_l];
    //     for (i, nf) in new_f.iter_mut().enumerate() {
    //         // i to bit array of length l
    //         let mut r = vec![F::zero(); self.l as usize];
    //         r[0] = x;
    //         let mut item = i as u64;
    //         for j in (0..new_l).rev() {
    //             r[j as usize + 1] = F::from(item & 1);
    //             item >>= 1;
    //         }
    //         *nf = self.evaluate(r.as_slice());
    //     }

    //     Self { l: new_l, f: new_f }
    // }

    // Fix the first variable x1 = x and return the new multilinear polynomial.
    // The complexity is O(2^l).
    pub fn fix_variable(&self, x: F) -> Self {
        assert!(self.l >= 1, "l must be at least 1");
        let new_l = self.l - 1;
        let mut new_f = vec![F::zero(); 1 << new_l];

        for (w, &fw) in self.f.iter().enumerate() {
            let wi = F::from((w as u64) >> new_l); // w's 0-th bit
            let term = wi * x + (F::one() - wi) * (F::one() - x);
            new_f[w & ((1 << new_l) - 1)] += term * fw;
        }

        Self { l: new_l, f: new_f }
    }

    // The sum over binary cube of f.
    pub fn sum(&self) -> F {
        self.f.iter().sum()
    }
}

impl<F: Field> MultivariatePolynomialOracle<F> for MultilinearPolynomial<F> {
    // Evaluate the multilinear polynomial at point r, using VSBW13.
    // The complexity is O(2^l).
    fn evaluate(&self, r: &[F]) -> F {
        assert!(r.len() as u64 == self.l, "r length must be l");
        let mut b = vec![F::one(); 1 << self.l];
        for (i, ri) in r.iter().enumerate() {
            let current_length = 1 << (i + 1);
            for j in (0..current_length).rev().step_by(2) {
                // j/2 == (j-1)/2
                b[j] = b[j / 2] * ri;
                b[j - 1] = b[j / 2] * (F::one() - ri);
            }
        }
        // dot product of self.f and b
        self.f.iter().zip(b.iter()).map(|(a, b)| (*a) * (*b)).sum()
    }

    fn deg(&self, i: u64) -> u64 {
        assert!(i >= 1 && i <= self.l, "i must be between 1 and l");
        // the degree of the x_i term is always 1 for a multilinear polynomial
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::{Fp64, MontBackend, MontConfig};
    use rand::prelude::*;

    #[derive(MontConfig)]
    #[modulus = "5"]
    #[generator = "3"]
    pub struct FqConfig;
    pub type Fq = Fp64<MontBackend<FqConfig, 1>>;

    #[test]
    fn multi_variate_evaluate() {
        let f = vec![Fq::from(1), Fq::from(2), Fq::from(1), Fq::from(4)];
        let polynomial = MultilinearPolynomial::new(2, f);

        let r = vec![Fq::from(1), Fq::from(1)];
        let value = Fq::from(4);
        assert_eq!(polynomial.evaluate(r.as_slice()), value);
        assert_eq!(polynomial.evaluate_cty11(r.as_slice()), value);

        let r = vec![Fq::from(4), Fq::from(4)];
        let value = Fq::from(2);
        assert_eq!(polynomial.evaluate(r.as_slice()), value);
        assert_eq!(polynomial.evaluate_cty11(r.as_slice()), value);

        let r = vec![Fq::from(4), Fq::from(3)];
        let value = Fq::from(3);
        assert_eq!(polynomial.evaluate(r.as_slice()), value);
        assert_eq!(polynomial.evaluate_cty11(r.as_slice()), value);

        let r = vec![Fq::from(0), Fq::from(4)];
        let value = Fq::from(0);
        assert_eq!(polynomial.evaluate(r.as_slice()), value);
        assert_eq!(polynomial.evaluate_cty11(r.as_slice()), value);

        let r = vec![Fq::from(4), Fq::from(2)];
        let value = Fq::from(4);
        assert_eq!(polynomial.evaluate(r.as_slice()), value);
        assert_eq!(polynomial.evaluate_cty11(r.as_slice()), value);

        let mut rng = StdRng::from_seed([0; 32]);
        for _ in 0..100 {
            let r = vec![Fq::from(rng.gen::<u64>()), Fq::from(rng.gen::<u64>())];
            assert_eq!(
                polynomial.evaluate(r.as_slice()),
                polynomial.evaluate_cty11(r.as_slice())
            );
        }
    }

    #[test]
    fn fix_variable() {
        let f = vec![Fq::from(1), Fq::from(2), Fq::from(1), Fq::from(4)];
        let polynomial = MultilinearPolynomial::new(2, f);

        let mut rng = StdRng::from_seed([0; 32]);
        for _ in 0..100 {
            let r1 = Fq::from(rng.gen::<u64>());
            let r2 = Fq::from(rng.gen::<u64>());
            let fixed_polynomial = polynomial.fix_variable(r1);
            assert_eq!(
                fixed_polynomial.evaluate(&[r2]),
                polynomial.evaluate(&[r1, r2])
            );
        }
    }
}
