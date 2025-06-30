//! src/main.rs  – works with halo2_proofs 0.3 and the TinyCircuit demo

mod params;     // universal parameters helper
mod circuit;    // TinyCircuit
mod verifier;   // proof verification helper

use circuit::TinyCircuit;
use halo2_proofs::{
    plonk::{keygen_pk, keygen_vk, create_proof},
    transcript::{Blake2bWrite, Challenge255},
};
use halo2curves::pasta::Fp;
use params::Curve;                     // Curve = vesta::Affine (see params.rs)

fn main() {
    // 1. create a witness for the tiny circuit --------------------------
    let circuit = TinyCircuit { witness_val: Some(Fp::from(42)) };

    // 2. load (or create) universal parameters & generate keys ----------
    let params = params::load_or_create();
    let empty  = TinyCircuit::default();             // empty version for key-gen
    let vk = keygen_vk(&params, &empty).expect("vk");
    let pk = keygen_pk(&params, vk.clone(), &empty).expect("pk");

    /* -- 3. generate a proof ------------------------------------------- */

    let mut transcript = Blake2bWrite::<_, _, Challenge255<_>>::init(Vec::new());

    create_proof::<Curve, _, _, _, _>(
        &params,
        &pk,
        &[circuit],          //  ← value, not &value
        &[&[]],              //  one (empty) public-input slice
        rand::rngs::OsRng,
        &mut transcript,
    )
    .expect("proof");

    let proof = transcript.finalize();


    // 4. verify the proof -----------------------------------------------
    verifier::verify(&params, &vk, &proof).expect("verify");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prove_and_verify_rnn() {
        main(); // just run main – panics if verification fails
    }
}
