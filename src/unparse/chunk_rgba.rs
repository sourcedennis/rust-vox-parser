
/// Writes the payload of the `RGBA` chunk.
/// 
/// See the [`parse::chunk_rgba`](crate::parse::chunk_rgba) documentation for
/// the format.
pub fn chunk_rgba( dst: &mut Vec< u8 >, rgba: &[(u8,u8,u8,u8); 255] ) {
  for (r,g,b,a) in rgba {
    dst.push( *r );
    dst.push( *g );
    dst.push( *b );
    dst.push( *a );
  }
  // The palette contains 1 more meaningless color
  dst.extend( &[0,0,0,0] );
}
