use ark_ff::{
    biginteger::BigInteger256 as BigInteger,
    fields::{FftParameters, Fp256, Fp256Parameters, FpParameters},
};

pub type Fr = Fp256<FrParameters>;

pub struct FrParameters;

impl Fp256Parameters for FrParameters {}
impl FftParameters for FrParameters {
    type BigInt = BigInteger;

    /// Let `N` be the size of the multiplicative group defined by the field.
    /// Then `TWO_ADICITY` is the two-adicity of `N`, i.e. the integer `s`
    /// such that `N = 2^s * t` for some odd integer `t`.
    const TWO_ADICITY: u32 = 5;

    /// 2^s root of unity computed by GENERATOR^t
    /// 4740934665446857387895054948191089665295030226009829406950782728666658007874
    const TWO_ADIC_ROOT_OF_UNITY: BigInteger = BigInteger([
        0xa4dcdba087826b42,
        0x6e4ab162f57f862a,
        0xabc5492749348d6a,
        0xa7b462035f8c169,
    ]);
}
impl FpParameters for FrParameters {
    /// The modulus of the field.
    /// MODULUS = 13108968793781547619861935127046491459309155893440570251786403306729687672801.
    const MODULUS: BigInteger = BigInteger([
        0x74fd06b52876e7e1,
        0xff8f870074190471,
        0x0cce760202687600,
        0x1cfb69d4ca675f52,
    ]);

    /// The number of bits needed to represent the `Self::MODULUS`.
    const MODULUS_BITS: u32 = 253;

    /// The number of bits that can be reliably stored.
    /// (Should equal `SELF::MODULUS_BITS - 1`)
    const CAPACITY: u32 = Self::MODULUS_BITS - 1;

    /// The number of bits that must be shaved from the beginning of
    /// the representation when randomly sampling.
    const REPR_SHAVE_BITS: u32 = 4;

    /// Let `M` be the power of 2^64 nearest to `Self::MODULUS_BITS`. Then
    /// `R = M % Self::MODULUS`.
    /// R = 10920338887063814464675503992315976178796737518116002025166357554075628257528
    const R: BigInteger = BigInteger([
        0x5817ca56bc48c0f8,
        0x0383c7fc5f37dc74,
        0x998c4fefecbc4ff8,
        0x1824b159acc5056f,
    ]);

    /// R2 = R^2 % Self::MODULUS
    /// R2 = 4932290691328759802879919559207542894238895193980447506221046538067943049163
    const R2: BigInteger = BigInteger([
        0xdbb4f5d658db47cb,
        0x40fa7ca27fecb938,
        0xaa9e6daec0055cea,
        0xae793ddb14aec7d,
    ]);

    /// INV = -MODULUS^{-1} mod 2^64
    /// INV = 17410672245482742751
    const INV: u64 = 0xf19f22295cc063df;

    /// A multiplicative generator of the field.
    /// `Self::GENERATOR` is an element having multiplicative order
    /// `Self::MODULUS - 1`.
    /// n = 9962557815892774795293348142308860067333132192265356416788884706064406244838
    const GENERATOR: BigInteger = BigInteger([
        0x56b6f3ab7b616de6,
        0x114f419d6c9083e5,
        0xbf518d217780c4b9,
        0x16069b9f45dbce7f,
    ]);

    /// (Self::MODULUS - 1) / 2
    /// 6554484396890773809930967563523245729654577946720285125893201653364843836400
    const MODULUS_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0xba7e835a943b73f0,
        0x7fc7c3803a0c8238,
        0x06673b0101343b00,
        0xe7db4ea6533afa9,
    ]);

    /// t for 2^s * t = MODULUS - 1, and t coprime to 2.
    /// t = 409655274805673363120685472720202858103411121670017820368325103335302739775
    ///   = (modulus-1)/2^5
    const T: BigInteger = BigInteger([
        0x8ba7e835a943b73f,
        0x07fc7c3803a0c823,
        0x906673b0101343b0,
        0xe7db4ea6533afa,
    ]);

    /// (t - 1) / 2
    ///  = 204827637402836681560342736360101429051705560835008910184162551667651369887
    const T_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0xc5d3f41ad4a1db9f,
        0x03fe3e1c01d06411,
        0x483339d80809a1d8,
        0x73eda753299d7d,
    ]);
}
