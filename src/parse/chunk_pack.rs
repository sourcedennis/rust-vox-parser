
// External library imports
use nom::combinator::all_consuming;
use nom::number::complete::le_u32;
// Local imports
use crate::parse::helpers::IResult;


/// Parses the payload of a PACK chunk, or fails if bytes are remaining.
/// See `chunk_pack`.
pub fn chunk_pack_all< 'a >( input: &'a [u8] ) -> IResult< &'a [u8], u32 > {
  all_consuming( chunk_pack )( input )
}

/// Parses the payload of the `PACK` chunk.
/// 
/// This chunk describes the number of models in the file. (This chunk seems
/// deprecated, as it is unused in recent MagicaVoxel versions)
///
/// | # Bytes | Type    | Value                                    |
/// | ------- | ------- | ---------------------------------------- |
/// | 4       | int     | numModels : num of SIZE and XYZI chunks  |
pub fn chunk_pack< 'a >( input: &'a [u8] ) -> IResult< &'a [u8], u32 > {
  le_u32( input )
}
