
// External library imports
use nom::combinator::all_consuming;
use nom::number::complete::le_u32;
// Local imports
use crate::data::spec::ShapeNode;
use crate::parse::helpers::{IResult, failure};
use crate::parse::error::VoxErrorKind;
use crate::parse::special::dict;


/// Parses the payload of a nSHP chunk, or fails if bytes are remaining.
/// See `chunk_nshp`.
pub fn chunk_nshp_all< 'a >( input: &'a [u8] ) -> IResult< &'a [u8], ShapeNode< 'a > > {
  all_consuming( chunk_nshp )( input )
}

/// Parses the payload of the `nSHP` chunk. (See [`ShapeNode`])
pub fn chunk_nshp< 'a >( input: &'a [u8] ) -> IResult< &'a [u8], ShapeNode< 'a > > {
  let (input, node_id)      = le_u32( input )?;
  let (input, attributes)   = dict( input )?;

  let (input, num_models) = le_u32( input )?;

  if num_models != 1 {
    return failure( input, VoxErrorKind::InvalidSHPModelCount( num_models ) );
  }

  let (input, model_id) = le_u32( input )?;
  let (input, model_attributes) = dict( input )?;

  let node =
    ShapeNode {
      node_id,
      attributes,

      model_id,
      model_attributes
    };

  nom::IResult::Ok((input, node))
}
