
// External library imports
use nom::combinator::all_consuming;
use nom::number::complete::le_u32;
use nom::multi::length_count;
// Local imports
use crate::data::spec::GroupNode;
use crate::parse::helpers::IResult;
use crate::parse::special::dict;


/// Parses the payload of a nGRP chunk, or fails if bytes are remaining.
/// See `chunk_ngrp`.
pub fn chunk_ngrp_all< 'a >( input: &'a [u8] ) -> IResult< &'a [u8], GroupNode< 'a > > {
  all_consuming( chunk_ngrp )( input )
}

/// Parses the payload of the `nGRP` chunk. (See [`GroupNode`])
pub fn chunk_ngrp< 'a >( input: &'a [u8] ) -> IResult< &'a [u8], GroupNode< 'a > > {
  let (input, node_id)      = le_u32( input )?;
  let (input, attributes)   = dict( input )?;

  let (input, child_nodes) = length_count( le_u32, le_u32 )( input )?;

  let node =
    GroupNode {
      node_id,
      attributes,
      child_nodes
    };

  nom::IResult::Ok((input, node))
}
