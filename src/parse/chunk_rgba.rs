
// External library imports
use nom::combinator::all_consuming;
use nom::bytes::complete::take;
use nom::number::complete::u8;
use nom::sequence::tuple;
// Local imports
use crate::parse::helpers::IResult;


/// Parses the payload of a RGBA chunk, or fails if bytes are remaining.
/// See `chunk_rgba`.
pub fn chunk_rgba_all< 'a >( input: &'a [u8] ) -> IResult< &'a [u8], [(u8,u8,u8,u8); 255] > {
  all_consuming( chunk_rgba )( input )
}

/// Parses the payload of the `RGBA` chunk.
/// 
/// This chunk describes the color palette. Only the 255 actual colors are extracted,
/// while the last stored quadruple is discarded (as mandated by the specification).
/// 
/// Note that this stores only the _colors_ within the palette. Material
/// properties are assigned externally (MATT or MATL).
///
/// | # Bytes | Type | Value                                                      |
/// | ------- | ---- | ---------------------------------------------------------- |
/// | 4 x 256 | int  | (R, G, B, A) : 1 byte for each component                   |
pub fn chunk_rgba< 'a >( mut input: &'a [u8] ) -> IResult<&'a [u8], [(u8,u8,u8,u8); 255] > {
  let mut rgba: [(u8,u8,u8,u8); 255] = [(0,0,0,0); 255];

  for i in 0..255 {
    let (input2, val) = tuple( (u8, u8, u8, u8) )( input )?;
    input = input2;
    rgba[ i ] = val;
  }
  // In the full palette, color 0 references the empty voxel. The stored palette
  // thus does not include the empty color, and only needs 255 stored colors.
  // The last remaining 4 bytes should thus be discarded.
  let (input, _) = take( 4usize )( input )?;

  nom::IResult::Ok( (input, rgba) )
}
