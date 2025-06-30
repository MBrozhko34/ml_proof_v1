//! src/verifier.rs   –  corrected for halo2-proofs 0.3  (Bundle A)

use halo2_proofs::{
    plonk::{verify_proof, SingleVerifier},
    transcript::{Blake2bRead, Challenge255},
    poly::commitment::Params,
};
use crate::params::Curve;                // Curve = vesta::Affine

pub fn verify(
    params: &Params<Curve>,
    vk:     &halo2_proofs::plonk::VerifyingKey<Curve>,
    proof:  &[u8],
) -> Result<(), halo2_proofs::plonk::Error> {
    // 1. Build the transcript reader over the proof bytes
    let mut tr = Blake2bRead::<_, _, Challenge255<_>>::init(proof);

    // 2. Call verify_proof  — note the argument order
    verify_proof::<Curve, _, _, _>(
        params,
        vk,
        SingleVerifier::new(params),  // ③ verification strategy
        &[],                          // ④ no public inputs
        &mut tr,                      // ⑤ transcript reader
    )
}
