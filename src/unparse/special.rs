/// Unparsers for the special data types as described in:
/// * https://github.com/ephtracy/voxel-model/blob/master/MagicaVoxel-file-format-vox-extension.txt


// Stdlib imports
use std::collections::HashMap;
// Local imports
use crate::data::spec::MatRowCols;
use crate::unparse::helpers::{le_u32};


/// Writes the `STRING` type.
/// 
/// See the [`parse::string`](crate::parse::string) documentation for the
/// format.
pub fn string( dst: &mut Vec< u8 >, v: &str ) {
  let bs = v.as_bytes( );
  le_u32( dst, bs.len( ) as u32 );
  dst.extend( bs );
}

/// Writes the `DICT` type. (See also [`dict_ref`])
/// 
/// See the [`parse::dict`](crate::parse::dict) documentation for the
/// format.
pub fn dict( dst: &mut Vec< u8 >, m: &HashMap< &str, String > ) {
  le_u32( dst, m.len( ) as u32 );

  for (k,v) in m {
    string( dst, k );
    string( dst, v );
  }
}

/// Writes the `DICT` type, where dictionary values are _string references_.
/// 
/// See also [`dict`].
pub fn dict_ref( dst: &mut Vec< u8 >, m: &HashMap< &str, &str > ) {
  le_u32( dst, m.len( ) as u32 );

  for (k,v) in m {
    string( dst, k );
    string( dst, v );
  }
}

/// Converts the `ROTATION` structure [`MatRowCols`] into a byte.
pub fn rotation_u8( m: MatRowCols ) -> u8 {
  let (r1, r2, is_neg1, is_neg2, is_neg3) =
    match m {
      MatRowCols::OneTwoThree( a, b, c ) => (0, 1, a, b, c),
      MatRowCols::OneThreeTwo( a, b, c ) => (0, 2, a, b, c),
      MatRowCols::TwoOneThree( a, b, c ) => (1, 0, a, b, c),
      MatRowCols::TwoThreeOne( a, b, c ) => (1, 2, a, b, c),
      MatRowCols::ThreeOneTwo( a, b, c ) => (2, 0, a, b, c),
      MatRowCols::ThreeTwoOne( a, b, c ) => (2, 1, a, b, c)
    };
  
  r1 | ( r2 << 2 ) | b2i( is_neg1 ) << 4 | b2i( is_neg2 ) << 5 | b2i( is_neg3 ) << 6
}

/// Converts a boolean to an `u8` integer.
fn b2i( b: bool ) -> u8 {
  if b {
    1
  } else {
    0
  }
}
