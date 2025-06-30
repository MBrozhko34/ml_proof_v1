mod params;
mod chips;
mod circuit;
mod verifier;

use circuit::{MlpCircuit, Witness, IN, HID};
use halo2_proofs::{
    plonk::{keygen_pk, keygen_vk, create_proof},
    transcript::{Blake2bWrite, Challenge255},
};
use halo2curves::pasta::Fp;
use params::Curve;

fn main() {
    // --- build a small deterministic witness ---------------------------
    let w = Witness {
        x:  [Fp::from(3), Fp::from(5)],
        w1: [
            [Fp::from(2), Fp::from(1)],   // neuron 0 weights
            [Fp::from(4), Fp::from(3)],   // neuron 1 weights
        ],
        b1: [Fp::from(7), Fp::from(11)],
        w2: [Fp::from(9), Fp::from(6)],
        b2: Fp::from(13),
    };

    // --- keys -----------------------------------------------------------
    let params = params::load_or_create();
    let empty  = MlpCircuit::default();
    let vk = keygen_vk(&params, &empty).unwrap();
    let pk = keygen_pk(&params, vk.clone(), &empty).unwrap();

    // --- proof ----------------------------------------------------------
    let circuit = MlpCircuit { wit: Some(w) };
    let mut tr = Blake2bWrite::<_, _, Challenge255<_>>::init(Vec::new());
    create_proof::<Curve, _, _, _, _>(
        &params,
        &pk,
        &[circuit],     // slice of circuits (by value)
        &[&[]],         // no public inputs
        rand::rngs::OsRng,
        &mut tr,
    ).unwrap();
    let proof = tr.finalize();

    // --- verify ---------------------------------------------------------
    verifier::verify(&params, &vk, &proof).unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn prove_and_verify_mlp() {
        super::main();
    }
}
