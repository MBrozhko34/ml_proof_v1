use halo2_proofs::poly::commitment::Params;
use halo2curves::pasta::vesta;            // Vesta is the curve (affine)
use std::fs;

const K: u32   = 10;          // 2^K rows — tiny demo
const FILE: &str = "params.bin";

pub type Curve = vesta::Affine;          // CurveAffine type used everywhere

pub fn load_or_create() -> Params<Curve> {
    if let Ok(buf) = fs::read(FILE) {
        let mut rdr = std::io::Cursor::new(buf);
        return Params::<Curve>::read(&mut rdr).expect("deserialize params");
    }
    let params = Params::<Curve>::new(K);
    let mut v = Vec::new();
    params.write(&mut v).unwrap();
    fs::write(FILE, v).unwrap();
    params
}
