use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    pasta::Fp,
    plonk::{Circuit, ConstraintSystem, Error},
};
use crate::chips::{Affine2Chip, Affine2Config};

pub const IN: usize = 2;
pub const HID: usize = 2;

#[derive(Clone)]
pub struct Witness {
    pub x:  [Fp; IN],
    pub w1: [[Fp; IN]; HID],   // input -> hidden
    pub b1: [Fp; HID],
    pub w2: [Fp; HID],         // hidden -> output
    pub b2: Fp,
}

#[derive(Default, Clone)]
pub struct MlpCircuit {
    pub wit: Option<Witness>,
}

#[derive(Clone)]
pub struct MlpConfig {
    h0: Affine2Config,
    h1: Affine2Config,
    out: Affine2Config,
}

impl Circuit<Fp> for MlpCircuit {
    type Config = MlpConfig;
    type FloorPlanner = SimpleFloorPlanner;
    fn without_witnesses(&self) -> Self { Self { wit: None } }

    fn configure(cs: &mut ConstraintSystem<Fp>) -> Self::Config {
        let h0  = Affine2Chip::configure(cs);
        let h1  = Affine2Chip::configure(cs);
        let out = Affine2Chip::configure(cs);
        MlpConfig { h0, h1, out }
    }

    fn synthesize(
        &self,
        cfg: Self::Config,
        mut layouter: impl Layouter<Fp>,
    ) -> Result<(), Error> {
        let Some(w) = &self.wit else {
            return Ok(());            // key-gen path
        };


        layouter.assign_region(
            || "MLP",
            |mut region| {
                // ----- hidden neuron 0 (row 0)
                let h0 = Affine2Chip::assign(
                    &cfg.h0, &mut region, 0,
                    Value::known(w.x[0]), Value::known(w.x[1]),
                    w.w1[0][0], w.w1[0][1], w.b1[0],
                )?;

                // ----- hidden neuron 1 (row 1)
                let h1 = Affine2Chip::assign(
                    &cfg.h1, &mut region, 1,
                    Value::known(w.x[0]), Value::known(w.x[1]),
                    w.w1[1][0], w.w1[1][1], w.b1[1],
                )?;

                // ----- output neuron (row 2)  – uses h0, h1 as inputs
                Affine2Chip::assign(
                    &cfg.out, &mut region, 2,
                    h0, h1,                // copy values from rows 0 & 1
                    w.w2[0], w.w2[1], w.b2,
                )?;

                Ok(())
            },
        )
    }
}
