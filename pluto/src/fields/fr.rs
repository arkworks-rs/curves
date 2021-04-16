use ark_ff::{
    biginteger::BigInteger448 as BigInteger,
    fields::{FftParameters, Fp448, Fp448Parameters, FpParameters},
};

pub struct FrParameters;

pub type Fr = Fp448<FrParameters>;

impl Fp448Parameters for FrParameters {}
impl FftParameters for FrParameters {
    type BigInt = BigInteger;

    const TWO_ADICITY: u32 = 32;

    // TWO_ADIC_ROOT_OF_UNITY = GENERATOR^T
    // Encoded in Montgomery form, so the value here is (5^T)R mod r.
    const TWO_ADIC_ROOT_OF_UNITY: BigInteger = BigInteger([
        0x1f8e3833929b5b76,
        0xefc4b78255f90eb7,
        0xb20ca09993e96198,
        0x8c7b1ccbfc9af2d0,
        0xa064b21566c8da3a,
        0x3af9ec0596cee365,
        0x2319f8cba754ae2d,
    ]);
}

impl FpParameters for FrParameters {
    // 0x24000000000024000130e0000d7f70e4a803ca76f439266f443f9a5c7a8a6c7be4a775fe8e177fd69ca7e85d60050af41ffffcd300000001
    const MODULUS: BigInteger = BigInteger([
        0x1ffffcd300000001,
        0x9ca7e85d60050af4,
        0xe4a775fe8e177fd6,
        0x443f9a5c7a8a6c7b,
        0xa803ca76f439266f,
        0x0130e0000d7f70e4,
        0x2400000000002400,
    ]);

    // R = 2^448 mod r
    const R: BigInteger = BigInteger([
        0x2000163afffffff9,
        0xb768a5725fdcb353,
        0xbf6bc60a1d5b8121,
        0x2242c778a637089c,
        0x67e576bf526ff2f5,
        0xf7a9dfffa183e9bf,
        0x03ffffffffff03ff,
    ]);

    // R2 = (2^448)^2 mod r
    const R2: BigInteger = BigInteger([
        0x740808c831022522,
        0xbc64e865fe4552ad,
        0x19bd905e6e4ff6c2,
        0x51da4da1c97f7164,
        0x44d51e923f646956,
        0xe436895a5a630ff5,
        0x050d7c998f46144e,
    ]);

    const MODULUS_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0x0ffffe6980000000,
        0x4e53f42eb002857a,
        0xf253baff470bbfeb,
        0xa21fcd2e3d45363d,
        0x5401e53b7a1c9337,
        0x0098700006bfb872,
        0x1200000000001200,
    ]);

    // T and T_MINUS_ONE_DIV_TWO, where MODULUS - 1 = 2^S * T

    const T: BigInteger = BigInteger([
        0x60050af41ffffcd3,
        0x8e177fd69ca7e85d,
        0x7a8a6c7be4a775fe,
        0xf439266f443f9a5c,
        0x0d7f70e4a803ca76,
        0x000024000130e000,
        0x0000000024000000,
    ]);

    const T_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0xb002857a0ffffe69,
        0x470bbfeb4e53f42e,
        0x3d45363df253baff,
        0x7a1c9337a21fcd2e,
        0x06bfb8725401e53b,
        0x0000120000987000,
        0x0000000012000000,
    ]);

    // GENERATOR = 19
    // Encoded in Montgomery form, so the value here is 19R mod r.
    const GENERATOR: BigInteger = BigInteger([
        0x2001acbaffffff79,
        0x637476c25d573943,
        0x6bb0c6c3119b95d3,
        0x0275993b6100caaa,
        0x66003b4535dcbb53,
        0x5f39dff8e1cb776b,
        0x03ffffffffed03ff,
    ]);

    const MODULUS_BITS: u32 = 446;

    const CAPACITY: u32 = Self::MODULUS_BITS - 1;

    const REPR_SHAVE_BITS: u32 = 2;

    // INV = -r^{-1} (mod 2^64)
    const INV: u64 = 2305839517405282303;
}
