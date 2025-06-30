# zk-MLP 🧮🔒  
*A Halo 2 proof of a (2 → 2 → 1) multilayer-perceptron*

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange.svg)](https://www.rust-lang.org)  
Builds, proves & verifies with **`halo2-proofs` 0.3** (IPA commitment ― no forks).



---

## ✨ Why this repo?

| Curious about … | …you’ll see |
|-----------------|------------|
| Writing a **custom gate** in Halo 2 | `Affine2Chip` (dot-product + bias + selector) |
| Building an **NN circuit** end-to-end | `src/circuit.rs` wires three chips into an MLP |
| Full **prove → verify** loop | reusable `prove_and_verify()` helper |
| **Robust tests** / fuzzing | 5 unit-tests (zeros, negatives, large values, random, etc.) |

---

## 📂 Directory layout

```text
src/
 ├ chips.rs      # reusable Affine2Chip gadget
 ├ circuit.rs    # 3-row MLP circuit (h₀, h₁, out)
 ├ params.rs     # lazy universal-parameter helper (Vesta / Fp, 2^12 rows)
 ├ verifier.rs   # thin wrapper around halo2::verify_proof
 └ main.rs       # demo + prove_and_verify() + tests
Cargo.toml
README.md
