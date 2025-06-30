//! Two small “chips” that we’ll reuse inside the RNN circuit:
//!   * AffineChip  – enforce y = W·x + b  (1 row per output neuron)
//!   * LookupTanh  – approximate tanh via 256-entry lookup table

use halo2_proofs::{
    circuit::{AssignedCell, Chip, Layouter, Value},
    plonk::{ConstraintSystem, Error},
    pasta::Fp,
    arithmetic::FieldExt,
    plonk::Column,
    plonk::Advice,
    plonk::TableColumn,
    plonk::Fixed,
};

/// -------- AffineChip -----------------------------------------------------
pub struct AffineConfig {
    pub input: Column<Advice>,
    pub weight: Column<Fixed>,
    pub bias: Column<Fixed>,
    pub output: Column<Advice>,
}

#[derive(Default)]
pub struct AffineChip<const N: usize>; // N inputs/outputs

impl<const N: usize> Chip<Fp> for AffineChip<N> {
    type Config = AffineConfig;
    type Loaded = ();

    fn configure(cs: &mut ConstraintSystem<Fp>) -> Self::Config {
        let input  = cs.advice_column();
        let weight = cs.fixed_column();
        let bias   = cs.fixed_column();
        let output = cs.advice_column();

        // y = Σ w_i * x_i + b
        cs.enable_equality(input);
        cs.enable_equality(output);
        for _ in 0..N {
            cs.create_gate("affine", |meta| {
                let x = meta.query_advice(input, 0);
                let w = meta.query_fixed(weight, 0);
                let b = meta.query_fixed(bias, 0);
                let y = meta.query_advice(output, 0);
                vec![y - (x * w + b)]
            });
        }
        AffineConfig { input, weight, bias, output }
    }

    fn load(&self, _layouter: &mut impl Layouter<Fp>) -> Result<(), Error> { Ok(()) }
}

/// Helper – lay out one affine row
impl<const N: usize> AffineChip<N> {
    pub fn assign_row(
        &self,
        cfg: &AffineConfig,
        mut layouter: impl Layouter<Fp>,
        x: [Value<Fp>; N],
        w: [Fp; N],
        b: Fp,
    ) -> Result<AssignedCell<Fp, Fp>, Error> {
        layouter.assign_region(
            || "affine row",
            |mut region| {
                let mut acc = Value::known(b);
                for i in 0..N {
                    let xi = region.assign_advice(
                        || format!("x_{i}"),
                        cfg.input,
                        i,
                        || x[i],
                    )?;
                    region.assign_fixed(
                        || format!("w_{i}"),
                        cfg.weight,
                        i,
                        || Value::known(w[i]),
                    )?;
                    acc = acc + x[i].map(|xi| xi * w[i]);
                }
                region.assign_fixed(|| "bias", cfg.bias, N, || Value::known(b))?;
                let out = region.assign_advice(|| "y", cfg.output, N, || acc)?;
                Ok(out)
            },
        )
    }
}

/// -------- Lookup tanh ----------------------------------------------------
pub struct TanhConfig {
    pub input: Column<Advice>,
    pub output: Column<Advice>,
    pub table_in: TableColumn,
    pub table_out: TableColumn,
}

pub struct LookupTanhChip;

impl Chip<Fp> for LookupTanhChip {
    type Config = TanhConfig;
    type Loaded = ();

    fn configure(cs: &mut ConstraintSystem<Fp>) -> Self::Config {
        let input = cs.advice_column();
        let output = cs.advice_column();
        let table_in = cs.lookup_table_column();
        let table_out = cs.lookup_table_column();

        cs.lookup("tanh", |meta| {
            let x = meta.query_advice(input, 0);
            let y = meta.query_advice(output, 0);
            vec![(x, table_in), (y, table_out)]
        });
        TanhConfig { input, output, table_in, table_out }
    }

    fn load(
        &self,
        layouter: &mut impl Layouter<Fp>,
    ) -> Result<(), Error> {
        // Hard-code 256-entry tanh(x/4) for demo.
        layouter.assign_table(
            || "tanh table",
            |mut tbl| {
                for i in 0..256 {
                    let x_f = Fp::from(i as u64);
                    let y_f = Fp::from(((f64::tanh((i as f64 - 128.0) / 32.0) * 10_000.0).round() as i64 + 10_000) as u64);
                    tbl.assign_cell(|| "x", self.0.table_in, i, || Value::known(x_f))?;
                    tbl.assign_cell(|| "y", self.0.table_out, i, || Value::known(y_f))?;
                }
                Ok(())
            },
        )
    }
}
