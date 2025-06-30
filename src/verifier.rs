use halo2_proofs::{
    plonk::{verify_proof, SingleVerifier},
    transcript::{Blake2bRead, Challenge255},
    poly::commitment::Params,
};
use crate::params::Curve;

pub fn verify(
    params: &Params<Curve>,
    vk:     &halo2_proofs::plonk::VerifyingKey<Curve>,
    proof:  &[u8],
) -> Result<(), halo2_proofs::plonk::Error> {
    let mut tr = Blake2bRead::<_, _, Challenge255<_>>::init(proof);
    verify_proof::<Curve, _, _, _>(
        params,
        vk,
        SingleVerifier::new(params),
        &[&[]],            // still no public inputs
        &mut tr,
    )
}
