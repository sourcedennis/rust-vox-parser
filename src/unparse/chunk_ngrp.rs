
// Local imports
use crate::data::spec::GroupNode;
use crate::unparse::helpers::{le_u32};
use crate::unparse::special::{dict_ref};

/// Writes the payload of the `nGRP` chunk. (See [`GroupNode`])
pub fn chunk_ngrp< 'a >( dst: &mut Vec< u8 >, g: &GroupNode< 'a > ) {
  le_u32( dst, g.node_id );
  dict_ref( dst, &g.attributes );

  le_u32( dst, g.child_nodes.len( ) as u32 );
  for cn in &g.child_nodes {
    le_u32( dst, *cn );
  }
}
