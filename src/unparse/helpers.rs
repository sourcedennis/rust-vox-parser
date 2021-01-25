
// Stdlib imports
use std::mem::transmute;


/// Writes a `u32` as Little-Endian to the byte vector.
pub fn le_u32( dst: &mut Vec< u8 >, v: u32 ) {
  dst.push( ( v & 0xFF ) as u8 );
  dst.push( ( ( v >> 8 ) & 0xFF ) as u8 );
  dst.push( ( ( v >> 16 ) & 0xFF ) as u8 );
  dst.push( ( ( v >> 24 ) & 0xFF ) as u8 );
}

/// Writes a `f32` as Little-Endian to the byte vector.
pub fn le_f32( dst: &mut Vec< u8 >, v: f32 ) {
  let v_u32 = unsafe { transmute::<f32, u32>( v ) };
  le_u32( dst, v_u32 );
}
