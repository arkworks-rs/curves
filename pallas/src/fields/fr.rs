use ark_ff::{
    biginteger::BigInteger256 as BigInteger,
    fields::{FftParameters, Fp256, Fp256Parameters, FpParameters},
};

pub struct FrParameters;

pub type Fr = Fp256<FrParameters>;

impl Fp256Parameters for FrParameters {}
impl FftParameters for FrParameters {
    type BigInt = BigInteger;

    const TWO_ADICITY: u32 = 32;

    // TWO_ADIC_ROOT_OF_UNITY = GENERATOR^T
    // Encoded in Montgomery form, so the value here is (5^T)R mod q.
    const TWO_ADIC_ROOT_OF_UNITY: BigInteger = BigInteger([
        0x218077428c9942de,
        0xcc49578921b60494,
        0xac2e5d27b2efbee2,
        0x0b79fa897f2db056,
    ]);
}

impl FpParameters for FrParameters {
    // 28948022309329048855892746252171976963363056481941647379679742748393362948097
    const MODULUS: BigInteger = BigInteger([
        0x8c46eb2100000001,
        0x224698fc0994a8dd,
        0x0000000000000000,
        0x4000000000000000,
    ]);

    // R = 2^256 mod q
    const R: BigInteger = BigInteger([
        0x5b2b3e9cfffffffd,
        0x992c350be3420567,
        0xffffffffffffffff,
        0x3fffffffffffffff,
    ]);

    // R2 = (2^256)^2 mod q
    const R2: BigInteger = BigInteger([
        0xfc9678ff0000000f,
        0x67bb433d891a16e3,
        0x7fae231004ccf590,
        0x096d41af7ccfdaa9,
    ]);

    const MODULUS_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0xc623759080000000,
        0x11234c7e04ca546e,
        0x0000000000000000,
        0x2000000000000000,
    ]);

    // T and T_MINUS_ONE_DIV_TWO, where MODULUS - 1 = 2^S * T

    const T: BigInteger = BigInteger([
        0x0994a8dd8c46eb21,
        0x00000000224698fc,
        0x0000000000000000,
        0x0000000040000000,
    ]);

    const T_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0x04ca546ec6237590,
        0x0000000011234c7e,
        0x0000000000000000,
        0x0000000020000000,
    ]);

    // GENERATOR = 5
    // Encoded in Montgomery form, so the value here is 5R mod q.
    const GENERATOR: BigInteger = BigInteger([
        0x96bc8c8cffffffed,
        0x74c2a54b49f7778e,
        0xfffffffffffffffd,
        0x3fffffffffffffff,
    ]);

    const MODULUS_BITS: u32 = 255;

    const CAPACITY: u32 = Self::MODULUS_BITS - 1;

    const REPR_SHAVE_BITS: u32 = 1;

    // INV = -q^{-1} (mod 2^64)
    const INV: u64 = 10108024940646105087;
}
