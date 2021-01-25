/// Parsers for the special data types as described in:
/// * https://github.com/ephtracy/voxel-model/blob/master/MagicaVoxel-file-format-vox-extension.txt


// Stdlib imports
use std::collections::HashMap;
use std::str;
// External library imports
use nom::bytes::complete::take;
use nom::number::complete::le_u32;
// Local imports
use crate::data::spec::MatRowCols;
use crate::parse::error::VoxErrorKind;
use crate::parse::helpers::{IResult, failure};


/// Parses the `STRING` type.
/// 
/// ```
/// int32  : buffer size (in bytes)
/// int8xN : buffer (without the ending "\0")
/// ```
pub fn string< 'a >( input: &'a [u8] ) -> IResult< &'a [u8], &'a str > {
  let (input, len) = le_u32( input )?;
  let (input2, xs)  = take( len as usize )( input )?;

  if let Ok( res ) = str::from_utf8( &xs ) {
    nom::IResult::Ok( ( input2, res ) )
  } else {
    failure( input, VoxErrorKind::InvalidUTF8String )
  }
}

/// Parses the `DICT` type.
///
/// ```
/// int32	: num of key-value pairs
/// {
/// STRING	: key
/// STRING	: value
/// }xN
/// ```
pub fn dict< 'a >( input: &'a [u8] ) -> IResult< &'a [u8], HashMap< &'a str, &'a str > > {
  let (mut input, n) = le_u32( input )?;
  let mut res = HashMap::with_capacity( n as usize );

  for _i in 0..n {
    let (input2, key)   = string( input )?;
    let (input2, value) = string( input2 )?;
    res.insert( key, value );
    input = input2;
  }

  nom::IResult::Ok( ( input, res ) )
}

/// Converts `ROTATION` byte into its corresponding structure ([`MatRowCols`]).
pub fn rotation_u8( b: u8 ) -> Option< MatRowCols > {
  let row1_col_index = b & 0x03;
  let row2_col_index = ( b >> 2 ) & 0x3;
  let is_neg1 = ( b >> 4 ) & 0x1 != 0;
  let is_neg2 = ( b >> 5 ) & 0x1 != 0;
  let is_neg3 = ( b >> 6 ) & 0x1 != 0;

  match (row1_col_index, row2_col_index) {
    (0, 1) => Some( MatRowCols::OneTwoThree( is_neg1, is_neg2, is_neg3 ) ),
    (0, 2) => Some( MatRowCols::OneThreeTwo( is_neg1, is_neg2, is_neg3 ) ),
    (1, 0) => Some( MatRowCols::TwoOneThree( is_neg1, is_neg2, is_neg3 ) ),
    (1, 2) => Some( MatRowCols::TwoThreeOne( is_neg1, is_neg2, is_neg3 ) ),
    (2, 0) => Some( MatRowCols::ThreeOneTwo( is_neg1, is_neg2, is_neg3 ) ),
    (2, 1) => Some( MatRowCols::ThreeTwoOne( is_neg1, is_neg2, is_neg3 ) ),
    _ => None
  }
}
