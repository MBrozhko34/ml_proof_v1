use halo2_proofs::{
    circuit::{Region, Value},
    pasta::Fp,
    plonk::{Advice, Column, ConstraintSystem, Error, Fixed, Selector},
    poly::Rotation,
};

#[derive(Debug, Clone)]
pub struct Affine2Config {
    pub q:    Selector,           // ← NEW
    pub x0:   Column<Advice>,
    pub x1:   Column<Advice>,
    pub y:    Column<Advice>,
    pub w0:   Column<Fixed>,
    pub w1:   Column<Fixed>,
    pub bias: Column<Fixed>,
}

pub struct Affine2Chip;

impl Affine2Chip {
    pub fn configure(meta: &mut ConstraintSystem<Fp>) -> Affine2Config {
        let q    = meta.selector();          // ← allocate selector
        let x0   = meta.advice_column();
        let x1   = meta.advice_column();
        let y    = meta.advice_column();
        let w0   = meta.fixed_column();
        let w1   = meta.fixed_column();
        let bias = meta.fixed_column();

        meta.create_gate("affine-2", |meta| {
            let q  = meta.query_selector(q);                 // ← selector
            let x0 = meta.query_advice(x0, Rotation::cur());
            let x1 = meta.query_advice(x1, Rotation::cur());
            let y  = meta.query_advice(y,  Rotation::cur());
            let w0 = meta.query_fixed(w0);
            let w1 = meta.query_fixed(w1);
            let b  = meta.query_fixed(bias);

            // apply only when q == 1
            vec![ q * (y - (x0 * w0 + x1 * w1 + b)) ]
        });

        Affine2Config { q, x0, x1, y, w0, w1, bias }
    }

    pub fn assign(
        cfg: &Affine2Config,
        region: &mut Region<'_, Fp>,
        offset: usize,
        x0: Value<Fp>,
        x1: Value<Fp>,
        w0: Fp,
        w1: Fp,
        bias: Fp,
    ) -> Result<Value<Fp>, Error> {
        // enable selector
        cfg.q.enable(region, offset)?;

        // ----------- debug log --------------------------------------------
        if cfg!(debug_assertions) {
            println!(
                "[DEBUG] row {}: x=({:?},{:?})  w=({:?},{:?})  b={:?}",
                offset, x0, x1, w0, w1, bias
            );
        }
        // ------------------------------------------------------------------

        region.assign_advice(|| "x0", cfg.x0, offset, || x0)?;
        region.assign_advice(|| "x1", cfg.x1, offset, || x1)?;
        region.assign_fixed(|| "w0", cfg.w0, offset, || Value::known(w0))?;
        region.assign_fixed(|| "w1", cfg.w1, offset, || Value::known(w1))?;
        region.assign_fixed(|| "b",  cfg.bias, offset, || Value::known(bias))?;

        let y_val = x0
            .zip(x1)
            .map(|(a, b)| a * w0 + b * w1 + bias);

        // ---------- debug log ---------------------------------------------
        if cfg!(debug_assertions) {
            println!("[DEBUG] row {offset}: y={:?}", y_val);
        }
        // ------------------------------------------------------------------

        region.assign_advice(|| "y", cfg.y, offset, || y_val)?;
        Ok(y_val)
    }

}
