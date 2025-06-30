use halo2_proofs::poly::commitment::Params;
use halo2curves::pasta::{Fp, vesta};
use std::fs;

pub type Curve = vesta::Affine;           // scalar field == Fp

const K: u32 = 12;        // 2¹² rows, plenty for the toy circuit
const FILE: &str = "params.bin";

pub fn load_or_create() -> Params<Curve> {
    if let Ok(buf) = fs::read(FILE) {
        let mut rdr = std::io::Cursor::new(buf);
        return Params::<Curve>::read(&mut rdr).unwrap();
    }
    let params = Params::<Curve>::new(K);
    let mut v = vec![];
    params.write(&mut v).unwrap();
    fs::write(FILE, v).unwrap();
    params
}
