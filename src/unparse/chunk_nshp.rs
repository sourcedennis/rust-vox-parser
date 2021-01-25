
// Local imports
use crate::data::spec::ShapeNode;
use crate::unparse::helpers::le_u32;
use crate::unparse::special::dict_ref;


/// Writes the payload of the `nSHP` chunk. (See [`ShapeNode`])
pub fn chunk_nshp< 'a >( dst: &mut Vec< u8 >, s: &ShapeNode< 'a > ) {
  le_u32( dst, s.node_id );
  dict_ref( dst, &s.attributes );
  le_u32( dst, 1 ); // num models
  
  le_u32( dst, s.model_id );
  dict_ref( dst, &s.model_attributes );
}
