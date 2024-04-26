use proofsargs::polynomial::univariate_polynomial::PolynomialTrait;
use proofsargs::randomness_oracle::private_coin::PrivateCoinRandomnessOracle;
use proofsargs::sum_check_protocol::verifier::Verifier;
use proofsargs::{
    polynomial::multilinear_polynomial::MultilinearPolynomial, sum_check_protocol::prover::Prover,
};

use ark_ff::{Fp64, MontBackend, MontConfig};
use rand::rngs::ThreadRng;

#[derive(MontConfig)]
#[modulus = "5"]
#[generator = "3"]
pub struct FqConfig;
pub type Fq = Fp64<MontBackend<FqConfig, 1>>;

fn main() {
    let l = 2;
    let f = vec![Fq::from(1), Fq::from(2), Fq::from(1), Fq::from(4)];
    let polynomial = MultilinearPolynomial::new(2, f.clone());

    let mut prover = Prover::new(l, polynomial.clone());
    assert!(prover.sum() == f.iter().sum());

    let rand_oracle = Box::<PrivateCoinRandomnessOracle<ThreadRng>>::default();
    let mut verifier = Verifier::new(l, Box::new(polynomial.clone()), rand_oracle);

    println!("Sum Check Start");
    println!("l = {}", l);
    let mut r: Option<_> = None;
    for round in 1..=l {
        println!("==== Round {}", round);
        let gi = prover.prove(round, r);
        println!("Prover send: g_{} = {}", round, gi);
        assert!(verifier.verify(round, gi.clone()));
        println!(
            "Verifier accept g(0)+g(1) = {}",
            gi.evaluate(Fq::from(0)) + gi.evaluate(Fq::from(1))
        );
        r = Some(verifier.challenge(round));
        println!(
            "Verifier send: r_{} = {}. Write down g(r_{}) = {}",
            round,
            r.unwrap(),
            round,
            gi.evaluate(r.unwrap())
        );
    }
    assert!(prover.current_round() == l);
    assert!(verifier.current_round() == l);
}
