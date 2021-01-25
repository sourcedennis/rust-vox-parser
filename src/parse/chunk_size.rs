
// External library imports
use nom::combinator::all_consuming;
use nom::number::complete::le_u32;
use nom::sequence::tuple;
// Local imports
use crate::parse::helpers::IResult;


/// Parses the payload of a SIZE chunk, or fails if bytes are remaining.
/// See `chunk_size`.
pub fn chunk_size_all< 'a >( input: &'a [u8] ) -> IResult< &'a [u8], (u32,u32,u32) > {
  all_consuming( chunk_size )( input )
}

/// Parses the payload of the `SIZE` chunk.
/// 
/// This chunk describes the dimensions of a single voxel model.
///
/// | # Bytes | Type | Value                      |
/// | ------- | ---- | -------------------------- |
/// | 4       | int  | size x                     |
/// | 4       | int  | size y                     |
/// | 4       | int  | size z : gravity direction |
pub fn chunk_size< 'a >( input: &'a [u8] ) -> IResult<&'a [u8], (u32,u32,u32)> {
  tuple( (le_u32, le_u32, le_u32) )( input )
}
