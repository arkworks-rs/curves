use ark_ec::{
    bls12,
    bls12::Bls12Parameters,
    models::CurveConfig,
    short_weierstrass::{Affine, SWCurveConfig},
    AffineRepr, Group,
};
use ark_ff::{Field, MontFp, Zero};
use ark_serialize::{Compress, SerializationError};
use ark_std::ops::Neg;

use super::util::{read_fq_with_offset, serialise_fq, EncodingFlags, G1_SERIALISED_SIZE};
use crate::*;

pub type G1Affine = bls12::G1Affine<crate::Parameters>;
pub type G1Projective = bls12::G1Projective<crate::Parameters>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Parameters;

impl CurveConfig for Parameters {
    type BaseField = Fq;
    type ScalarField = Fr;

    /// COFACTOR = (x - 1)^2 / 3  = 76329603384216526031706109802092473003
    const COFACTOR: &'static [u64] = &[0x8c00aaab0000aaab, 0x396c8c005555e156];

    /// COFACTOR_INV = COFACTOR^{-1} mod r
    /// = 52435875175126190458656871551744051925719901746859129887267498875565241663483
    const COFACTOR_INV: Fr =
        MontFp!("52435875175126190458656871551744051925719901746859129887267498875565241663483");
}

impl SWCurveConfig for Parameters {
    /// COEFF_A = 0
    const COEFF_A: Fq = Fq::ZERO;

    /// COEFF_B = 4
    const COEFF_B: Fq = MontFp!("4");

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const GENERATOR: G1Affine = G1Affine::new_unchecked(G1_GENERATOR_X, G1_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(_: Self::BaseField) -> Self::BaseField {
        Self::BaseField::zero()
    }

    #[inline]
    fn is_in_correct_subgroup_assuming_on_curve(p: &G1Affine) -> bool {
        // Algorithm from Section 6 of https://eprint.iacr.org/2021/1130.
        //
        // Check that endomorphism_p(P) == -[X^2]P

        // An early-out optimization described in Section 6.
        // If uP == P but P != point of infinity, then the point is not in the right
        // subgroup.
        let x_times_p = p.mul_bigint(crate::Parameters::X);
        if x_times_p.eq(p) && !p.infinity {
            return false;
        }

        let minus_x_squared_times_p = x_times_p.mul_bigint(crate::Parameters::X).neg();
        let endomorphism_p = endomorphism(p);
        minus_x_squared_times_p.eq(&endomorphism_p)
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
            p = G1Affine::zero();
        }
        // need to access the field struct `x` directly, otherwise we get None from xy()
        // method
        let x_bytes = serialise_fq(p.x);
        if encoding.is_compressed {
            let mut bytes: [u8; G1_SERIALISED_SIZE] = x_bytes;

            encoding.encode_flags(&mut bytes);
            writer.write_all(&bytes)?;
        } else {
            let mut bytes = [0u8; 2 * G1_SERIALISED_SIZE];
            bytes[0..G1_SERIALISED_SIZE].copy_from_slice(&x_bytes[..]);
            bytes[G1_SERIALISED_SIZE..].copy_from_slice(&serialise_fq(p.y)[..]);

            encoding.encode_flags(&mut bytes);
            writer.write_all(&bytes)?;
        };

        Ok(())
    }

    fn serialized_size(compress: Compress) -> usize {
        if compress == Compress::Yes {
            G1_SERIALISED_SIZE
        } else {
            G1_SERIALISED_SIZE * 2
        }
    }
}

/// G1_GENERATOR_X =
/// 3685416753713387016781088315183077757961620795782546409894578378688607592378376318836054947676345821548104185464507
pub const G1_GENERATOR_X: Fq = MontFp!("3685416753713387016781088315183077757961620795782546409894578378688607592378376318836054947676345821548104185464507");

/// G1_GENERATOR_Y =
/// 1339506544944476473020471379941921221584933875938349620426543736416511423956333506472724655353366534992391756441569
pub const G1_GENERATOR_Y: Fq = MontFp!("1339506544944476473020471379941921221584933875938349620426543736416511423956333506472724655353366534992391756441569");

/// BETA is a non-trivial cubic root of unity in Fq.
pub const BETA: Fq = MontFp!("793479390729215512621379701633421447060886740281060493010456487427281649075476305620758731620350");

pub fn endomorphism(p: &Affine<Parameters>) -> Affine<Parameters> {
    // Endomorphism of the points on the curve.
    // endomorphism_p(x,y) = (BETA * x, y)
    // where BETA is a non-trivial cubic root of unity in Fq.
    let mut res = (*p).clone();
    res.x *= BETA;
    res
}

fn read_compressed<R: ark_serialize::Read>(
    mut reader: R,
) -> Result<Affine<Parameters>, ark_serialize::SerializationError> {
    let mut bytes = [0u8; G1_SERIALISED_SIZE];
    reader
        .read_exact(&mut bytes)
        .ok()
        .ok_or(SerializationError::InvalidData)?;

    // Obtain the three flags from the start of the byte sequence
    let flags = EncodingFlags::get_flags(&bytes[..]);

    // we expect to be deserializing a compressed point
    if !flags.is_compressed {
        return Err(SerializationError::UnexpectedFlags);
    }

    if flags.is_infinity {
        return Ok(G1Affine::default());
    }

    // Attempt to obtain the x-coordinate
    let x = read_fq_with_offset(&bytes, 0, true)?;

    let p = G1Affine::get_point_from_x_unchecked(x, flags.is_lexographically_largest)
        .ok_or(SerializationError::InvalidData)?;

    Ok(p)
}

fn read_uncompressed<R: ark_serialize::Read>(
    mut reader: R,
) -> Result<Affine<Parameters>, ark_serialize::SerializationError> {
    let mut bytes = [0u8; 2 * G1_SERIALISED_SIZE];
    reader
        .read_exact(&mut bytes)
        .ok()
        .ok_or(SerializationError::InvalidData)?;

    // Obtain the three flags from the start of the byte sequence
    let flags = EncodingFlags::get_flags(&bytes[..]);

    // we expect to be deserializing an uncompressed point
    if flags.is_compressed {
        return Err(SerializationError::UnexpectedFlags);
    }

    if flags.is_infinity {
        return Ok(G1Affine::default());
    }

    // Attempt to obtain the x-coordinate
    let x = read_fq_with_offset(&bytes, 0, true)?;
    // Attempt to obtain the y-coordinate
    let y = read_fq_with_offset(&bytes, 1, false)?;

    let p = G1Affine::new_unchecked(x, y);

    Ok(p)
}

#[cfg(test)]
mod tests {
    use ark_ec::AffineRepr;
    use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress};

    use crate::G1Affine;
    extern crate alloc;
    use alloc::vec;

    #[test]
    fn g1_standard_serialization() {
        let pairs = [
            (G1Affine::generator(), "97f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb"),
            (G1Affine::zero(), "c00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000")
        ];

        for (p, s) in pairs {
            // serialize
            let mut serialized = vec![0; p.serialized_size(Compress::Yes)];
            p.serialize_with_mode(&mut serialized[..], Compress::Yes)
                .unwrap();
            assert_eq!(hex::encode(&serialized), s);

            // deserialize, should get the same point
            let p2 = G1Affine::deserialize_with_mode(
                &serialized[..],
                Compress::Yes,
                ark_serialize::Validate::Yes,
            )
            .unwrap();
            assert_eq!(p, p2);
        }
    }
}
