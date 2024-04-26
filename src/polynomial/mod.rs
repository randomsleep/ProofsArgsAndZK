pub mod multilinear_polynomial;
pub mod univariate_polynomial;

pub trait MultivariatePolynomialOracle<F> {
    fn evaluate(&self, r: &[F]) -> F; // evaluate the multivariate polynomial at point r
    fn deg(&self, i: u64) -> u64; // return the degree of the x_i term
}
