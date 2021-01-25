
// External library imports
use nom::combinator::{opt, all_consuming, eof};
use nom::number::complete::{le_i32, le_u32};
use nom::character::complete::{char, digit1};
// Local imports
use crate::data::spec::{TransformNode, MatRowCols};
use crate::parse::helpers::{IResult, failure};
use crate::parse::error::VoxErrorKind;
use crate::parse::special::{dict, rotation_u8};


/// Parses the payload of a nTRN chunk, or fails if bytes are remaining.
/// See `chunk_ntrn`.
pub fn chunk_ntrn_all< 'a >( input: &'a [u8] ) -> IResult< &'a [u8], TransformNode > {
  all_consuming( chunk_ntrn )( input )
}

/// Parses the payload of the `nTRN` chunk. (See [`TransformNode`])
pub fn chunk_ntrn< 'a >( input: &'a [u8] ) -> IResult< &'a [u8], TransformNode > {
  let (input, node_id) = le_u32( input )?;
  let (input, attributes) = dict( input )?;

  let name =
    if let Some( s ) = attributes.get( "_name" ) {
      Some( s.to_string( ) )
    } else {
      None
    };
  let is_hidden =
    match attributes.get( "_hidden" ) {
      Some( &"0" ) => false,
      Some( &"1" ) => true,
      Some( _ ) => { return failure( input, VoxErrorKind::InvalidTRNHidden ); },
      None => {
        // println!( "_is_hidden attribute missing. {:?}", attributes );
        false // by default, assume it's not hidden
      }
    };
  
  let (input, child_id) = le_u32( input )?;
  let (input, reserved_id) = le_i32( input )?;

  if reserved_id != -1 {
    return failure( input, VoxErrorKind::InvalidTRNReserved( reserved_id ) );
  }

  let (input, layer_id)   = le_i32( input )?;

  let layer_id =
    if layer_id == -1 {
      None
    } else if layer_id < 0 {
      return failure( input, VoxErrorKind::InvalidLayrId );
    } else {
      Some( layer_id as u32 )
    };

  let (input, num_frames) = le_i32( input )?;

  if num_frames != 1 {
    return failure( input, VoxErrorKind::InvalidTRNFrames( num_frames ) );
  }

  let (input, frame_attributes) = dict( input )?;

  let translation =
    if let Some( val ) = frame_attributes.get( "_t" ) {
      if let Ok( (_, t) ) = translation_text( val ) {
        t
      } else {
        return failure( input, VoxErrorKind::InvalidTRNProperty );
      }
    } else {
      (0,0,0)
    };
  
  let rotation =
    if let Some( val ) = frame_attributes.get( "_r" ) {
      if let Ok( r ) = val.parse::<u8>( ) {
        if let Some( rv ) = rotation_u8( r ) {
          rv
        } else {
          return failure( input, VoxErrorKind::InvalidTRNProperty );
        }
      } else {
        return failure( input, VoxErrorKind::InvalidTRNProperty );
      }
    } else {
      MatRowCols::OneTwoThree( false, false, false )
    };

  let node =
    TransformNode {
      node_id,
      name,
      is_hidden,
      child_node_id: child_id,
      layer_id,
      translation,
      rotation
    };

  nom::IResult::Ok((input, node))
}


// Helpers

fn translation_text< 'a >( t: &'a str ) -> IResult< &'a str, (i32,i32,i32) > {
  let (t, x) = number_text( VoxErrorKind::InvalidTRNProperty, t )?;
  let (t, _) = char( ' ' )( t )?;
  let (t, y) = number_text( VoxErrorKind::InvalidTRNProperty, t )?;
  let (t, _) = char( ' ' )( t )?;
  let (t, z) = number_text( VoxErrorKind::InvalidTRNProperty, t )?;
  let (t, _) = eof( t )?;

  nom::IResult::Ok((t, (x, y, z)))
}

fn number_text< 'a >( err: VoxErrorKind, t: &'a str ) -> IResult< &'a str, i32 > {
  let (t, is_neg) = opt(char('-'))( t )?;
  let (t, num_str) = digit1( t )?;

  if let Ok( i ) = num_str.parse::<i32>( ) {
    let neg_i =
      if is_neg.is_some( ) {
        -i
      } else {
        i
      };
    nom::IResult::Ok((t, neg_i))
  } else {
    failure( t, err )
  }
}
