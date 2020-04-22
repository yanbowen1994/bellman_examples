#![allow(unused_imports)]
#![allow(unused_variables)]
extern crate bellperson as bellman;
extern crate paired;
extern crate rand;
extern crate ff;
extern crate log;

use ff::{Field, PrimeField};
use bellman::{Circuit, ConstraintSystem, SynthesisError};
use paired::{
    Engine,
    bls12_381::{Bls12, Fr}
};
use rand::thread_rng;
use bellman::groth16::{
    self, create_random_proof_batch, generate_random_parameters, prepare_verifying_key, verify_proof, Proof,
};
use log::*;

mod cube; 

fn main(){
    fil_logger::init();

    println!("Prove that I know x such that x^3 + x + 5 == 35.");
    
    let rng = &mut rand_core::OsRng;
    
    println!("Creating parameters...");
    
    // Create parameters for our circuit
    let params = {
        let c = cube::CubeDemo::<Bls12> {
            x: None
        };

        generate_random_parameters(c, rng).unwrap()
    };
    
    // Prepare the verification key (for proof verification)
    let pvk = prepare_verifying_key(&params.vk);

    println!("Creating proofs...");
    
    // Create an instance of circuit
    let c = cube::CubeDemo::<Bls12> {
        x: Fr::from_str("3")
    };
    
    // Create a groth16 proof with our parameters.
    let groth_proofs = create_random_proof_batch(vec![c], &params, rng).unwrap();

    let proof:Vec<groth16::Proof<Bls12>> = groth_proofs
        .into_iter()
        .map(|groth_proof| {
            let mut proof_vec = vec![];
            groth_proof.write(&mut proof_vec)?;
            let gp = groth16::Proof::<Bls12>::read(&proof_vec[..])?;
            Ok(gp)
        })
        .collect::<Result<Vec<_>, std::io::Error>>().unwrap();

    info!("verify");

    assert!(verify_proof(
        &pvk,
        &proof[0],
        &[Fr::from_str("35").unwrap()]
    ).unwrap());
}