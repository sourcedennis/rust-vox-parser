
// Local imports
use crate::unparse::helpers::le_u32;


/// Writes the payload of the `PACK` chunk.
/// 
/// See the [`parse::chunk_pack`](crate::parse::chunk_pack) documentation for
/// the format.
pub fn chunk_pack( dst: &mut Vec< u8 >, num_models: &u32 ) {
  le_u32( dst, *num_models );
}
