
// Local imports
use crate::unparse::helpers::{le_u32};


/// Writes the payload of the XYZI chunk.
///
/// See the [`parse::chunk_xyzi`](crate::parse::chunk_xyzi) documentation for
/// the format.
pub fn chunk_xyzi( dst: &mut Vec< u8 >, voxels: &[(u8,u8,u8,u8)] ) {
  le_u32( dst, voxels.len( ) as u32 );

  for (x,y,z,i) in voxels {
    dst.push( *x );
    dst.push( *y );
    dst.push( *z );
    dst.push( *i );
  }
}
