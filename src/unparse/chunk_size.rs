
// Local imports
use crate::unparse::helpers::{le_u32};


/// Writes the payload of the `SIZE` chunk.
/// 
/// See the [`parse::chunk_size`](crate::parse::chunk_size) documentation for
/// the format.
pub fn chunk_size( dst: &mut Vec< u8 >, size: &(u32,u32,u32) ) {
  let (x_size, y_size, z_size) = size;
  le_u32( dst, *x_size );
  le_u32( dst, *y_size );
  le_u32( dst, *z_size );
}
