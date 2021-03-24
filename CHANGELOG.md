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

### Improvements
- [\#42](https://github.com/arkworks-rs/curves/pull/42) Remove the dependency of `rand_xorshift`.

### Bug fixes
- [\#28](https://github.com/arkworks-rs/curves/pull/28), [\#49](https://github.com/arkworks-rs/curves/pull/49) Fix broken documentation links.
- [\#38](https://github.com/arkworks-rs/curves/pull/38) Compile with `panic='abort'` in release mode, for safety of the library across FFI boundaries.
- [\#45](https://github.com/arkworks-rs/curves/pull/45) Fix `ark-ed-on-mnt4-753`.

## v0.1.0

Initial Release