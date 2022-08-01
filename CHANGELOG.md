# CHANGELOG

## Pending

- [\#76](https://github.com/arkworks-rs/curves/pull/76) twisted Edwards parameters for bls12-377
- Fixed curve benches

### Breaking changes

- [\#104](https://github.com/arkworks-rs/curves/pull/104) Remove `QUADRATIC_NONRESIDUE` parameter from implementors of `Fp2Config`.
 
### Features

### Improvements

- [\#74](https://github.com/arkworks-rs/curves/pull/74) Use Scott's subgroup membership tests for `G1` and `G2` of BLS12-381.

### Bug fixes

## v0.3.0 

### Breaking changes

- [\#60](https://github.com/arkworks-rs/curves/pull/60) Change the scalar group generator of `Fr` of `bls12_377` Fr from `11` to `22`.
- [\#61](https://github.com/arkworks-rs/curves/pull/61) Remove `ATE_LOOP_COUNT_IS_NEGATIVE` from BN254 curve parameter.

### Features

- [\#64](https://github.com/arkworks-rs/curves/pull/64) Implement the Bandersnatch curve, another twisted Edwards curve for BLS12-381.

### Improvements

### Bug fixes

## v0.2.0

### Breaking changes

- Requires all crates from `arkworks-rs/algebra` to have version `v0.2.0` or greater.

### Features

- [\#3](https://github.com/arkworks-rs/curves/pull/3) Add constraints for
        `ark-bls12-377`,
        `ark-ed-on-bls12-377`,
        `ark-ed-on-bls12-381`,
        `ark-ed-on-bn254`,
        `ark-ed-on-cp6-782`,
        `ark-ed-on-bw6-761`,
        `ark-ed-on-mnt4-298`,
        `ark-ed-on-mnt4-753`,
        `ark-mnt4-298`,
        `ark-mnt6-298`,
        `ark-mnt4-753`,
        `ark-mnt6-753`.
- [\#7](https://github.com/arkworks-rs/curves/pull/7) Add benchmarks for Edwards curves.
- [\#19](https://github.com/arkworks-rs/curves/pull/19) Change field constants to be provided as normal strings, instead of in Montgomery form.
- [\#53](https://github.com/arkworks-rs/curves/pull/53) Add benchmarks for Pallas and Vesta curves.

### Improvements

- [\#42](https://github.com/arkworks-rs/curves/pull/42) Remove the dependency of `rand_xorshift`.

### Bug fixes

- [\#28](https://github.com/arkworks-rs/curves/pull/28), [\#49](https://github.com/arkworks-rs/curves/pull/49) Fix broken documentation links.
- [\#38](https://github.com/arkworks-rs/curves/pull/38) Compile with `panic='abort'` in release mode, for safety of the library across FFI boundaries.
- [\#45](https://github.com/arkworks-rs/curves/pull/45) Fix `ark-ed-on-mnt4-753`.

## v0.1.0

Initial Release
