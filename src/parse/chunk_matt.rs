
// External library imports
use nom::combinator::all_consuming;
use nom::number::complete::{le_u32, le_f32};
// Local imports
use crate::data::spec::{Matt, MattType};
use crate::parse::error::VoxErrorKind;
use crate::parse::helpers::{IResult, parse_if, check, vox_assert, failure};


/// Parses the payload of a MATT chunk, or fails if bytes are remaining.
/// See `chunk_matt`.
pub fn chunk_matt_all< 'a >( input: &'a [u8] ) -> IResult< &'a [u8], Matt > {
  all_consuming( chunk_matt )( input )
}

/// Parses the payload of the `MATT` chunk. (See [`Matt`])
pub fn chunk_matt<'a>(input: &'a [u8]) -> IResult<&'a [u8], Matt> {
    let (input, id) = le_u32(input)?;

    // A material replaces an entry in the palette, of which there are 256.
    // (Entry 0 is the empty voxel)
    if id < 1 || id > 255 {
      return failure( input, VoxErrorKind::InvalidMattId( id ) )
    }

    let (input, mat_type_id) = le_u32(input)?;
    let (input, mat_weight) = le_f32(input)?;

    let matt_type = match mat_type_id {
        0 => {
            vox_assert(VoxErrorKind::InvalidMattType, mat_weight == 1.0, input)?;
            MattType::Diffuse
        }
        1 => {
            vox_assert(
                VoxErrorKind::InvalidMattType,
                mat_weight > 0.0 && mat_weight <= 1.0,
                input,
            )?;
            MattType::Metal(mat_weight)
        }
        2 => {
            vox_assert(
                VoxErrorKind::InvalidMattType,
                mat_weight > 0.0 && mat_weight <= 1.0,
                input,
            )?;
            MattType::Glass(mat_weight)
        }
        3 => {
            vox_assert(
                VoxErrorKind::InvalidMattType,
                mat_weight > 0.0 && mat_weight <= 1.0,
                input,
            )?;
            MattType::Emissive(mat_weight)
        }
        _ => return failure(input, VoxErrorKind::InvalidMattType),
    };

    let (input, property_bits) = le_u32(input)?;

    let (input, prop_plastic) = parse_if(
        property_bits & 0x01 != 0,
        |i| f32_prop(|v| v == 0.0 || v == 1.0, i),
        input,
    )?;
    let (input, prop_roughness) = parse_if(
        property_bits & 0x02 != 0,
        |i| f32_prop(|v| v > 0.0 && v <= 1.0, i),
        input,
    )?;
    let (input, prop_specular) = parse_if(
        property_bits & 0x04 != 0,
        |i| f32_prop(|v| v > 0.0 && v <= 1.0, i),
        input,
    )?;
    let (input, prop_ior) = parse_if(
        property_bits & 0x08 != 0,
        |i| f32_prop(|v| v > 0.0 && v <= 1.0, i),
        input,
    )?;
    let (input, prop_attenuation) = parse_if(
        property_bits & 0x10 != 0,
        |i| f32_prop(|v| v > 0.0 && v <= 1.0, i),
        input,
    )?;
    let (input, prop_power) = parse_if(
        property_bits & 0x20 != 0,
        |i| f32_prop(|v| v > 0.0 && v <= 1.0, i),
        input,
    )?;
    let (input, prop_glow) = parse_if(
        property_bits & 0x40 != 0,
        |i| f32_prop(|v| v > 0.0 && v <= 1.0, i),
        input,
    )?;
    let prop_is_total_power = property_bits & 0x80 != 0;

    let matt = Matt {
        id: id as u8,
        matt_type,
        prop_plastic,
        prop_roughness,
        prop_specular,
        prop_ior,
        prop_attenuation,
        prop_power,
        prop_glow,
        prop_is_total_power,
    };

    nom::IResult::Ok((input, matt))
}

fn f32_prop<'a, F>(f_check: F, input: &'a [u8]) -> IResult<&'a [u8], f32>
where
    F: Fn(f32) -> bool,
{
    check(
        VoxErrorKind::InvalidMattProperty,
        le_f32,
        f_check,
        input,
    )
}
