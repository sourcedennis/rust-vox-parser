
// Stdlib imports
use std::collections::HashMap;
// Local imports
use crate::data::spec::Layr;
use crate::unparse::helpers::le_u32;
use crate::unparse::special::dict;


/// Writes the payload of the `LAYR` chunk. (See [`Layr`])
/// 
/// ```
/// int32	: layer id
/// DICT	: layer attributes
///         (_name : string)
///         (_hidden : 0/1)
/// int32	: reserved id, must be -1
/// ```
pub fn chunk_layr< 'a >( dst: &mut Vec< u8 >, l: &Layr< 'a > ) {
  le_u32( dst, l.id );

  let mut attributes = HashMap::new( );

  if let Some( name ) = l.name {
    attributes.insert( "_name", name.to_string( ) );
  }

  if l.is_hidden {
    attributes.insert( "_hidden", "1".to_string( ) );
  }
  dict( dst, &attributes );
  le_u32( dst, 0xFFFFFFFF ); // reserved -1
}
