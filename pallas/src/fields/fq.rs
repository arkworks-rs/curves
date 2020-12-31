use ark_ff::{
    biginteger::BigInteger256 as BigInteger,
    fields::{FftParameters, Fp256, Fp256Parameters},
};

pub type Fq = Fp256<FqParameters>;

pub struct FqParameters;

impl Fp256Parameters for FqParameters {}
impl FftParameters for FqParameters {
    type BigInt = BigInteger;

    const TWO_ADICITY: u32 = 32;

    // TWO_ADIC_ROOT_OF_UNITY = GENERATOR^T
    // Encoded in Montgomery form, so the value here is (5^T)R mod p.
    const TWO_ADIC_ROOT_OF_UNITY: BigInteger = BigInteger([
        0xa28db849bad6dbf0,
        0x9083cd03d3b539df,
        0xfba6b9ca9dc8448e,
        0x3ec928747b89c6da,
    ]);
}

impl ark_ff::fields::FpParameters for FqParameters {
    // 28948022309329048855892746252171976963363056481941560715954676764349967630337
    const MODULUS: BigInteger = BigInteger([
        0x992d30ed00000001,
        0x224698fc094cf91b,
        0x0000000000000000,
        0x4000000000000000,
    ]);

    // R = 2^256 mod p
    const R: BigInteger = BigInteger([
        0x34786d38fffffffd,
        0x992c350be41914ad,
        0xffffffffffffffff,
        0x3fffffffffffffff,
    ]);

    // R2 = (2^256)^2 mod p
    const R2: BigInteger = BigInteger([
        0x8c78ecb30000000f,
        0xd7d30dbd8b0de0e7,
        0x7797a99bc3c95d18,
        0x096d41af7b9cb714,
    ]);

    const MODULUS_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0xcc96987680000000,
        0x11234c7e04a67c8d,
        0x0000000000000000,
        0x2000000000000000,
    ]);

    // T and T_MINUS_ONE_DIV_TWO, where MODULUS - 1 = 2^S * T
    const T: BigInteger = BigInteger([
        0x094cf91b992d30ed,
        0x00000000224698fc,
        0x0000000000000000,
        0x0000000040000000,
    ]);

    const T_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0x04a67c8dcc969876,
        0x0000000011234c7e,
        0x0000000000000000,
        0x0000000020000000,
    ]);

    // GENERATOR = 5
    // Encoded in Montgomery form, so the value here is 5R mod p.
    const GENERATOR: BigInteger = BigInteger([
        0xa1a55e68ffffffed,
        0x74c2a54b4f4982f3,
        0xfffffffffffffffd,
        0x3fffffffffffffff,
    ]);

    const MODULUS_BITS: u32 = 255;

    const CAPACITY: u32 = Self::MODULUS_BITS - 1;

    const REPR_SHAVE_BITS: u32 = 1;

    // INV = -p^{-1} (mod 2^64)
    const INV: u64 = 11037532056220336127;
}
