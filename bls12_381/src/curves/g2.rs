use ark_ec::{
    bls12,
    bls12::Bls12Parameters,
    models::CurveConfig,
    short_weierstrass::{Affine, SWCurveConfig},
    AffineRepr,
};
use ark_ff::{Field, MontFp, Zero};
use ark_serialize::{Compress, SerializationError};

use crate::*;

pub type G2Affine = bls12::G2Affine<crate::Parameters>;
pub type G2Projective = bls12::G2Projective<crate::Parameters>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Parameters;

impl CurveConfig for Parameters {
    type BaseField = Fq2;
    type ScalarField = Fr;

    /// COFACTOR = (x^8 - 4 x^7 + 5 x^6) - (4 x^4 + 6 x^3 - 4 x^2 - 4 x + 13) //
    /// 9
    /// = 305502333931268344200999753193121504214466019254188142667664032982267604182971884026507427359259977847832272839041616661285803823378372096355777062779109
    #[rustfmt::skip]
    const COFACTOR: &'static [u64] = &[
        0xcf1c38e31c7238e5,
        0x1616ec6e786f0c70,
        0x21537e293a6691ae,
        0xa628f1cb4d9e82ef,
        0xa68a205b2e5a7ddf,
        0xcd91de4547085aba,
        0x91d50792876a202,
        0x5d543a95414e7f1,
    ];

    /// COFACTOR_INV = COFACTOR^{-1} mod r
    /// 26652489039290660355457965112010883481355318854675681319708643586776743290055
    const COFACTOR_INV: Fr =
        MontFp!("26652489039290660355457965112010883481355318854675681319708643586776743290055");
}

impl SWCurveConfig for Parameters {
    /// COEFF_A = [0, 0]
    const COEFF_A: Fq2 = Fq2::new(g1::Parameters::COEFF_A, g1::Parameters::COEFF_A);

    /// COEFF_B = [4, 4]
    const COEFF_B: Fq2 = Fq2::new(g1::Parameters::COEFF_B, g1::Parameters::COEFF_B);

    /// AFFINE_GENERATOR_COEFFS = (G2_GENERATOR_X, G2_GENERATOR_Y)
    const GENERATOR: G2Affine = G2Affine::new_unchecked(G2_GENERATOR_X, G2_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(_: Self::BaseField) -> Self::BaseField {
        Self::BaseField::zero()
    }

    fn is_in_correct_subgroup_assuming_on_curve(point: &G2Affine) -> bool {
        // Algorithm from Section 4 of https://eprint.iacr.org/2021/1130.
        //
        // Checks that [p]P = [X]P

        let mut x_times_point = point.mul_bigint(crate::Parameters::X);
        if crate::Parameters::X_IS_NEGATIVE {
            x_times_point = -x_times_point;
        }

        let p_times_point = p_power_endomorphism(point);

        x_times_point.eq(&p_times_point)
    }

    fn deserialize_with_mode<R: ark_serialize::Read>(
        mut reader: R,
        compress: ark_serialize::Compress,
        validate: ark_serialize::Validate,
    ) -> Result<Affine<Self>, ark_serialize::SerializationError> {
        let p = if compress == ark_serialize::Compress::Yes {
            read_compressed(&mut reader)?
        } else {
            read_uncompressed(&mut reader)?
        };

        if validate == ark_serialize::Validate::Yes {
            if !p.is_in_correct_subgroup_assuming_on_curve() {
                return Err(SerializationError::InvalidData);
            }
        }
        Ok(p)
    }

    fn serialize_with_mode<W: ark_serialize::Write>(
        item: &Affine<Self>,
        mut writer: W,
        compress: ark_serialize::Compress,
    ) -> Result<(), SerializationError> {
        let encoding = EncodingFlags {
            is_compressed: compress == ark_serialize::Compress::Yes,
            is_infinity: item.is_zero(),
            is_lexographically_largest: item.y > -item.y,
        };
        let mut p = *item;
        if encoding.is_infinity {
            p = G2Affine::default();
        }

        let mut x_bytes = [0u8; G2_SERIALISED_SIZE];
        let c1_bytes = serialise_fq(p.x.c1);
        let c0_bytes = serialise_fq(p.x.c0);
        (&mut x_bytes[0..48]).copy_from_slice(&c1_bytes[..]);
        (&mut x_bytes[48..96]).copy_from_slice(&c0_bytes[..]);
        if encoding.is_compressed {
            let mut bytes: [u8; G2_SERIALISED_SIZE] = x_bytes;

            encoding.encode_flags(&mut bytes);
            writer.write(&bytes)?;
        } else {
            let mut bytes = [0u8; 2 * G2_SERIALISED_SIZE];

            let mut y_bytes = [0u8; G2_SERIALISED_SIZE];
            let c1_bytes = serialise_fq(p.y.c1);
            let c0_bytes = serialise_fq(p.y.c0);
            (&mut y_bytes[0..48]).copy_from_slice(&c1_bytes[..]);
            (&mut y_bytes[48..96]).copy_from_slice(&c0_bytes[..]);
            bytes[0..G2_SERIALISED_SIZE].copy_from_slice(&x_bytes);
            bytes[G2_SERIALISED_SIZE..].copy_from_slice(&y_bytes);

            encoding.encode_flags(&mut x_bytes);
            writer.write(&x_bytes)?;
        };

        Ok(())
    }

    fn serialized_size(compress: ark_serialize::Compress) -> usize {
        if compress == Compress::Yes {
            G2_SERIALISED_SIZE
        } else {
            2 * G2_SERIALISED_SIZE
        }
    }
}

pub const G2_GENERATOR_X: Fq2 = Fq2::new(G2_GENERATOR_X_C0, G2_GENERATOR_X_C1);
pub const G2_GENERATOR_Y: Fq2 = Fq2::new(G2_GENERATOR_Y_C0, G2_GENERATOR_Y_C1);

/// G2_GENERATOR_X_C0 =
/// 352701069587466618187139116011060144890029952792775240219908644239793785735715026873347600343865175952761926303160
pub const G2_GENERATOR_X_C0: Fq = MontFp!("352701069587466618187139116011060144890029952792775240219908644239793785735715026873347600343865175952761926303160");

/// G2_GENERATOR_X_C1 =
/// 3059144344244213709971259814753781636986470325476647558659373206291635324768958432433509563104347017837885763365758
pub const G2_GENERATOR_X_C1: Fq = MontFp!("3059144344244213709971259814753781636986470325476647558659373206291635324768958432433509563104347017837885763365758");

/// G2_GENERATOR_Y_C0 =
/// 1985150602287291935568054521177171638300868978215655730859378665066344726373823718423869104263333984641494340347905
pub const G2_GENERATOR_Y_C0: Fq = MontFp!("1985150602287291935568054521177171638300868978215655730859378665066344726373823718423869104263333984641494340347905");

/// G2_GENERATOR_Y_C1 =
/// 927553665492332455747201965776037880757740193453592970025027978793976877002675564980949289727957565575433344219582
pub const G2_GENERATOR_Y_C1: Fq = MontFp!("927553665492332455747201965776037880757740193453592970025027978793976877002675564980949289727957565575433344219582");

// psi(x,y) = (x**p * PSI_X, y**p * PSI_Y) is the Frobenius composed
// with the quadratic twist and its inverse

// PSI_X = 1/(u+1)^((p-1)/3)
pub const P_POWER_ENDOMORPHISM_COEFF_0 : Fq2 = Fq2::new(
    Fq::ZERO,
    MontFp!(
                "4002409555221667392624310435006688643935503118305586438271171395842971157480381377015405980053539358417135540939437"
    )
);

// PSI_Y = 1/(u+1)^((p-1)/2)
pub const P_POWER_ENDOMORPHISM_COEFF_1: Fq2 = Fq2::new(
    MontFp!(
                "2973677408986561043442465346520108879172042883009249989176415018091420807192182638567116318576472649347015917690530"),
    MontFp!(
                "1028732146235106349975324479215795277384839936929757896155643118032610843298655225875571310552543014690878354869257")
);

pub fn p_power_endomorphism(p: &Affine<Parameters>) -> Affine<Parameters> {
    // The p-power endomorphism for G2 is defined as follows:
    // 1. Note that G2 is defined on curve E': y^2 = x^3 + 4(u+1).
    //    To map a point (x, y) in E' to (s, t) in E,
    //    one set s = x / ((u+1) ^ (1/3)), t = y / ((u+1) ^ (1/2)),
    //    because E: y^2 = x^3 + 4.
    // 2. Apply the Frobenius endomorphism (s, t) => (s', t'),
    //    another point on curve E, where s' = s^p, t' = t^p.
    // 3. Map the point from E back to E'; that is,
    //    one set x' = s' * ((u+1) ^ (1/3)), y' = t' * ((u+1) ^ (1/2)).
    //
    // To sum up, it maps
    // (x,y) -> (x^p / ((u+1)^((p-1)/3)), y^p / ((u+1)^((p-1)/2)))
    // as implemented in the code as follows.

    let mut res = *p;
    res.x.frobenius_map(1);
    res.y.frobenius_map(1);

    let tmp_x = res.x.clone();
    res.x.c0 = -P_POWER_ENDOMORPHISM_COEFF_0.c1 * &tmp_x.c1;
    res.x.c1 = P_POWER_ENDOMORPHISM_COEFF_0.c1 * &tmp_x.c0;
    res.y *= P_POWER_ENDOMORPHISM_COEFF_1;

    res
}

fn read_compressed<R: ark_serialize::Read>(
    mut reader: R,
) -> Result<Affine<Parameters>, ark_serialize::SerializationError> {
    let mut bytes = [0u8; G2_SERIALISED_SIZE];
    reader
        .read_exact(&mut bytes)
        .ok()
        .ok_or(SerializationError::InvalidData)?;

    // Obtain the three flags from the start of the byte sequence
    let flags = EncodingFlags::get_flags(&bytes);

    // we expect to be deserializing a compressed point
    if !flags.is_compressed {
        return Err(SerializationError::UnexpectedFlags);
    }

    if flags.is_infinity {
        return Ok(G2Affine::default());
    }

    // Attempt to obtain the x-coordinate
    let xc1 = read_fq_with_offset(&bytes, 0, true)?;
    let xc0 = read_fq_with_offset(&bytes, 1, false)?;

    let x = Fq2::new(xc0, xc1);

    let p = G2Affine::get_point_from_x_unchecked(x, flags.is_lexographically_largest)
        .ok_or(SerializationError::InvalidData)?;

    Ok(p)
}

fn read_uncompressed<R: ark_serialize::Read>(
    mut reader: R,
) -> Result<Affine<Parameters>, ark_serialize::SerializationError> {
    let mut bytes = [0u8; 2 * G2_SERIALISED_SIZE];
    reader
        .read_exact(&mut bytes)
        .ok()
        .ok_or(SerializationError::InvalidData)?;

    // Obtain the three flags from the start of the byte sequence
    let flags = EncodingFlags::get_flags(&bytes);

    // we expect to be deserializing an uncompressed point
    if flags.is_compressed {
        return Err(SerializationError::UnexpectedFlags);
    }

    if flags.is_infinity {
        return Ok(G2Affine::default());
    }

    // Attempt to obtain the x-coordinate
    let xc1 = read_fq_with_offset(&bytes, 0, true)?;
    let xc0 = read_fq_with_offset(&bytes, 1, false)?;
    let x = Fq2::new(xc0, xc1);

    // Attempt to obtain the y-coordinate
    let yc1 = read_fq_with_offset(&bytes, 2, false)?;
    let yc0 = read_fq_with_offset(&bytes, 3, false)?;
    let y = Fq2::new(yc0, yc1);

    let p = G2Affine::new_unchecked(x, y);

    Ok(p)
}

#[cfg(test)]
mod tests {
    use ark_ec::AffineRepr;
    use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress};

    use crate::G2Affine;
    extern crate alloc;
    use alloc::vec;

    #[test]
    fn g2_standard_serialization() {
        let pairs = [
            (G2Affine::generator(), "93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8"),
            (G2Affine::zero(), "c00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"),
        ];

        for (p, s) in pairs {
            // serialize
            let mut serialized = vec![0; p.serialized_size(Compress::Yes)];
            p.serialize_with_mode(&mut serialized[..], Compress::Yes)
                .unwrap();
            assert_eq!(hex::encode(&serialized), s);

            // deserialize, should get the same point
            let p2 = G2Affine::deserialize_with_mode(
                &serialized[..],
                Compress::Yes,
                ark_serialize::Validate::Yes,
            )
            .unwrap();
            assert_eq!(p, p2);
        }
    }
}
