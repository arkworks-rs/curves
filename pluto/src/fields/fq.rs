use ark_ff::{
    biginteger::BigInteger448 as BigInteger,
    fields::{FftParameters, Fp448, Fp448Parameters},
};

pub type Fq = Fp448<FqParameters>;

pub struct FqParameters;

impl Fp448Parameters for FqParameters {}
impl FftParameters for FqParameters {
    type BigInt = BigInteger;

    const TWO_ADICITY: u32 = 32;

    // TWO_ADIC_ROOT_OF_UNITY = GENERATOR^T
    // Encoded in Montgomery form, so the value here is (19^T)R mod q.
    const TWO_ADIC_ROOT_OF_UNITY: BigInteger = BigInteger([
        0xebda7b4bdd4ba274,
        0x2e31416b79bde3fd,
        0x86c37b96cdca3d4b,
        0x9c352819e72e0970,
        0xec2b878c57869a49,
        0xbb47401eb1ff6cf2,
        0x054f245ce7ac1bcd,
    ]);
}

impl ark_ff::fields::FpParameters for FqParameters {
    // 0x24000000000024000130e0000d7f70e4a803ca76f439266f443f9a5cda8a6c7be4a7a5fe8fadffd6a2a7e8c30006b9459ffffcd300000001
    const MODULUS: BigInteger = BigInteger([
        0x9ffffcd300000001,
        0xa2a7e8c30006b945,
        0xe4a7a5fe8fadffd6,
        0x443f9a5cda8a6c7b,
        0xa803ca76f439266f,
        0x0130e0000d7f70e4,
        0x2400000000002400,
    ]);

    // R = 2^448 mod q
    const R: BigInteger = BigInteger([
        0xa000163afffffff9,
        0x8d68a2aaffd0ef18,
        0xbf6a760a123e0121,
        0x2242c7760637089c,
        0x67e576bf526ff2f5,
        0xf7a9dfffa183e9bf,
        0x03ffffffffff03ff,
    ]);

    // R2 = (2^448)^2 mod q
    const R2: BigInteger = BigInteger([
        0xd9702c6d54dc0598,
        0x4b20c07277ae01f1,
        0x7a42067a8ccd154b,
        0x734fd363b575c23e,
        0x20b6db3d7481a84c,
        0x8bcb0f20758aec85,
        0x1a4b16581f66e3cc,
    ]);

    const MODULUS_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0xcffffe6980000000,
        0x5153f46180035ca2,
        0xf253d2ff47d6ffeb,
        0xa21fcd2e6d45363d,
        0x5401e53b7a1c9337,
        0x0098700006bfb872,
        0x1200000000001200,
    ]);

    // T and T_MINUS_ONE_DIV_TWO, where MODULUS - 1 = 2^S * T

    const T: BigInteger = BigInteger([
        0x0006b9459ffffcd3,
        0x8fadffd6a2a7e8c3,
        0xda8a6c7be4a7a5fe,
        0xf439266f443f9a5c,
        0x0d7f70e4a803ca76,
        0x000024000130e000,
        0x0000000024000000,
    ]);

    const T_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0x80035ca2cffffe69,
        0x47d6ffeb5153f461,
        0x6d45363df253d2ff,
        0x7a1c9337a21fcd2e,
        0x06bfb8725401e53b,
        0x0000120000987000,
        0x0000000012000000,
    ]);

    // GENERATOR = 19
    // Encoded in Montgomery form, so the value here is 19R mod q.
    const GENERATOR: BigInteger = BigInteger([
        0xa001acbaffffff79,
        0x3974412afc744c48,
        0x6b9776c23b3e15d0,
        0x02759908c100caaa,
        0x66003b4535dcbb53,
        0x5f39dff8e1cb776b,
        0x03ffffffffed03ff,
    ]);

    const MODULUS_BITS: u32 = 446;

    const CAPACITY: u32 = Self::MODULUS_BITS - 1;

    const REPR_SHAVE_BITS: u32 = 2;

    // INV = -q^{-1} (mod 2^64)
    const INV: u64 = 11529211554260058111;
}
