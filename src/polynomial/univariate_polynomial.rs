use ark_ff::fields::Field;

pub trait PolynomialTrait<F: Field> {
    fn evaluate(&self, r: F) -> F;
    fn deg(&self) -> u64;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Polynomial<F: Field> {
    coeff: Vec<F>, // coefficients of the polynomial: f(X) = cofeff[0] + coeff[1]X + coeff[2]X^2 + ...
}

impl<F: Field> Polynomial<F> {
    pub fn new(coeff: Vec<F>) -> Self {
        assert!(!coeff.is_empty(), "f must be non-empty");
        Self { coeff }
    }

    pub fn coeff(&self) -> &[F] {
        &self.coeff
    }
}

impl<F: Field> std::fmt::Display for Polynomial<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut first = true;
        for (i, c) in self.coeff.iter().enumerate() {
            if c.is_zero() {
                continue;
            }
            if first {
                first = false;
            } else {
                write!(f, " + ")?;
            }
            if i == 0 {
                write!(f, "{}", c)?;
            } else if i == 1 {
                write!(f, "{}X", c)?;
            } else {
                write!(f, "{}X^{}", c, i)?;
            }
        }
        Ok(())
    }
}

impl<F: Field> PolynomialTrait<F> for Polynomial<F> {
    /// Evaluate the polynomial at point r.
    fn evaluate(&self, r: F) -> F {
        let mut result = F::zero();
        let mut r_pow = F::one();
        for &c in &self.coeff {
            result += c * r_pow;
            r_pow *= r;
        }
        result
    }

    fn deg(&self) -> u64 {
        self.coeff.len() as u64 - 1
    }
}
