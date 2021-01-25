
// Stdlib imports
use std::collections::HashMap;
// Local imports
use crate::data::spec::TransformNode;
use crate::unparse::helpers::{le_u32};
use crate::unparse::special::{dict,dict_ref,rotation_u8};


/// Writes the payload of the `nTRN` chunk. (See [`TransformNode`])
pub fn chunk_ntrn( dst: &mut Vec< u8 >, t: &TransformNode ) {
  le_u32( dst, t.node_id );

  let mut attributes: HashMap< &str, &str > = HashMap::new( );
  if let Some( name ) = &t.name {
    attributes.insert( "_name", name );
  }
  if t.is_hidden {
    attributes.insert( "_hidden", "1" );
  }
  dict_ref( dst, &attributes );

  le_u32( dst, t.child_node_id );
  le_u32( dst, 0xFFFFFFFF ); // reserved id
  le_u32( dst, t.layer_id.unwrap_or( 0xFFFFFFFF ) );
  le_u32( dst, 1 ); // num frames

  let mut frame_attributes = HashMap::new( );
  if !t.rotation.is_identity( ) {
    let v = rotation_u8( t.rotation );
    frame_attributes.insert( "_r", format!("{}", v) );
  }
  if t.translation != (0,0,0) {
    let (x,y,z) = t.translation;
    frame_attributes.insert( "_t", format!("{} {} {}", x, y, z) );
  }
  dict( dst, &frame_attributes );
}
