
// External library imports
use nom::combinator::all_consuming;
use nom::number::complete::{u8, le_u32};
use nom::sequence::tuple;
use nom::multi::length_count;
// Local imports
use crate::parse::helpers::IResult;


/// Parses the payload of a XYZI chunk, or fails if bytes are remaining.
/// See `chunk_xyzi`.
pub fn chunk_xyzi_all< 'a >( input: &'a [u8] ) -> IResult< &'a [u8], Vec< (u8,u8,u8,u8) > > {
  all_consuming( chunk_xyzi )( input )
}

/// Parses the payload of the `XYZI` chunk.
/// 
/// This chunk describes the voxels within a single voxel model.
///
/// | # Bytes | Type | Value                                             |
/// | ------- | ---- | ------------------------------------------------- |
/// | 4       | int  | numVoxels (N)                                     |
/// | 4 x N   | int  | (x, y, z, colorIndex) : 1 byte for each component |
pub fn chunk_xyzi< 'a >( input: &'a [u8] ) -> IResult<&'a [u8], Vec< (u8,u8,u8,u8) > > {
  length_count( le_u32, tuple( (u8, u8, u8, u8) ) )( input )
}
