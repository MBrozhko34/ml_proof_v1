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

/// Produce a proof for `witness` and immediately verify it.
/// Returns `Ok(())` on success so callers can assert.
pub fn prove_and_verify(w: Witness) -> Result<(), Box<dyn std::error::Error>> {
    use halo2_proofs::plonk::{keygen_pk, keygen_vk, create_proof};
    use halo2_proofs::transcript::{Blake2bWrite, Challenge255};
    use rand::rngs::OsRng;

    let params = params::load_or_create();
    let empty  = MlpCircuit::default();
    let vk = keygen_vk(&params, &empty)?;
    let pk = keygen_pk(&params, vk.clone(), &empty)?;

    let circuit = MlpCircuit { wit: Some(w) };
    let mut tr  = Blake2bWrite::<_, _, Challenge255<_>>::init(Vec::new());

    create_proof::<Curve, _, _, _, _>(
        &params,
        &pk,
        &[circuit],
        &[&[]],
        OsRng,
        &mut tr,
    )?;
    let proof = tr.finalize();
    verifier::verify(&params, &vk, &proof)?;
    Ok(())
}

fn main() {
    // single demo run
    let demo_witness = Witness {
        x:  [Fp::from(3), Fp::from(5)],
        w1: [[Fp::from(2), Fp::from(1)],[Fp::from(4), Fp::from(3)]],
        b1: [Fp::from(7), Fp::from(11)],
        w2: [Fp::from(9), Fp::from(6)],
        b2: Fp::from(13),
    };
    prove_and_verify(demo_witness).unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;
    use halo2curves::pasta::Fp;
    use rand::Rng;

    /// Convert an `i64` (positive or negative) to the field element `Fp`.
    fn fp(i: i64) -> Fp {
        if i >= 0 {
            Fp::from(i as u64)
        } else {
            -Fp::from((-i) as u64)
        }
    }

    /// Build a `Witness` from plain signed integers (handy for unit-tests).
    fn w(
        x:  [i64; 2],
        w1: [[i64; 2]; 2],
        b1: [i64; 2],
        w2: [i64; 2],
        b2: i64,
    ) -> Witness {
        Witness {
            x:  x.map(fp),
            w1: w1.map(|row| row.map(fp)),
            b1: b1.map(fp),
            w2: w2.map(fp),
            b2: fp(b2),
        }
    }

    /* ------------------------------------------------------------------ */
    /* test cases                                                         */
    /* ------------------------------------------------------------------ */

    /// Original deterministic witness (all small positives).
    #[test]
    fn case_positive() {
        assert!(prove_and_verify(
            w([3, 5], [[2, 1], [4, 3]], [7, 11], [9, 6], 13)
        )
        .is_ok());
    }

    /// All zeros should satisfy the affine constraints trivially.
    #[test]
    fn case_zeros() {
        assert!(prove_and_verify(
            w([0, 0], [[0, 0], [0, 0]], [0, 0], [0, 0], 0)
        )
        .is_ok());
    }

    /// Negative numbers (represented modulo the field).
    #[test]
    fn case_negative() {
        assert!(prove_and_verify(
            w([-1, -2], [[-3, 4], [-5, 6]], [0, -7], [-8, 9], -10)
        )
        .is_ok());
    }

    /// Large unsigned values near the field modulus (stress fixed-point).
    #[test]
    fn case_large() {
        assert!(prove_and_verify(
            w(
                [1 << 30, 1 << 29],
                [[1 << 28, 1 << 27], [1 << 26, 1 << 25]],
                [1 << 24, 1 << 23],
                [1 << 22, 1 << 21],
                1 << 20
            )
        )
        .is_ok());
    }

    /// Random small values in the range [-20, 20].
    #[test]
    fn case_random() {
        let mut rng = rand::thread_rng();
        let mut rand_i = || rng.gen_range(-20..=20);
        let witness = w(
            [rand_i(), rand_i()],
            [[rand_i(), rand_i()], [rand_i(), rand_i()]],
            [rand_i(), rand_i()],
            [rand_i(), rand_i()],
            rand_i(),
        );
        assert!(prove_and_verify(witness).is_ok());
    }
}


