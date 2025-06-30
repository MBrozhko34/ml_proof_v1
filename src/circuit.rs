//! Tiny 1-cell demo circuit that works with halo2-proofs v0.3

use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    pasta::Fp,
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error},
};

#[derive(Default, Clone)]
pub struct TinyCircuit {
    pub witness_val: Option<Fp>,
}

impl Circuit<Fp> for TinyCircuit {
    // One advice column is the entire “config”.
    type Config       = Column<Advice>;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self { witness_val: None }
    }

    /// Declare the single advice column.
    fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
        meta.advice_column()
    }

    /// Lay out one value in that column at row-0.
    fn synthesize(
        &self,
        col: Self::Config,
        mut layouter: impl Layouter<Fp>,
    ) -> Result<(), Error> {
        layouter.assign_region(
            || "place one advice cell",
            |mut region| {
                region.assign_advice(
                    || "dummy",
                    col,                     // ← real Column<Advice>
                    0,                       // row offset
                    || Value::known(self.witness_val.unwrap_or(Fp::zero())),
                )?;
                Ok(())
            },
        )
    }
}
