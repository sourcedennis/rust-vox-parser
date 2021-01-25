//! Unparsers for the `.vox` binary format.
//! 
//! The chunk data structures are specified in [`crate::data::spec`].


// Alphabetic order
mod chunk_layr;
mod chunk_matl;
mod chunk_matt;
mod chunk_ntrn;
mod chunk_nshp;
mod chunk_ngrp;
mod chunk_pack;
mod chunk_rgba;
mod chunk_size;
mod chunk_xyzi;
mod chunk;
mod helpers;
mod special;

// Specification order
pub use chunk_pack::chunk_pack;
pub use chunk_size::chunk_size;
pub use chunk_xyzi::chunk_xyzi;
pub use chunk_rgba::chunk_rgba;
pub use chunk_matt::chunk_matt;
pub use chunk_ntrn::chunk_ntrn;
pub use chunk_ngrp::chunk_ngrp;
pub use chunk_nshp::chunk_nshp;
pub use chunk_matl::chunk_matl;
pub use chunk_layr::chunk_layr;
pub use special::{string, dict, dict_ref, rotation_u8};

use crate::data::spec::Chunk;
use crate::data::custom::VoxScene;
use crate::convert::from_custom;
use chunk::chunk;
use helpers::le_u32;


/// Writes a complete `.vox` file to the byte vector.
/// 
/// The `.vox` file is entirely described by the chunks. No validation is
/// performed on these chunks, or their order. Note that this order must satisfy
/// the order as given by the specification.
pub fn file_raw( chunks: &[Chunk] ) -> Vec< u8 >{
  let mut dst = Vec::new( );

  dst.extend( b"VOX " ); // "VOX "
  le_u32( &mut dst, 150 ); // version

  let mut main_children = Vec::new( );
  for c in chunks {
    let mut child_payload = Vec::new( );
    chunk( &mut child_payload, c ); // unparse the chunk

    // wrap it in a raw chunk container
    raw_chunk( &mut main_children, &c.tag( ), &child_payload, &[] );
  }

  // wrap it all in the MAIN chunk
  raw_chunk( &mut dst, b"MAIN", &[], &main_children );

  dst
}

/// Writes a [`VoxScene`] in `.vox` format to the byte vector.
/// 
/// The scene is converted into its corresponding `Chunk`s, which are written to
/// the vector.
pub fn file_custom( scene: &VoxScene ) {
  let out_chunks = from_custom( &scene );
  file_raw( &out_chunks );
}

/// Writes a raw chunk to the byte vector.
fn raw_chunk(
    dst: &mut Vec< u8 >,
    tag: &[u8; 4],
    payload: &[u8],
    children: &[u8] ) {

  dst.extend( tag );
  le_u32( dst, payload.len( ) as u32 );
  le_u32( dst, children.len( ) as u32 );

  dst.extend( payload );
  dst.extend( children );
}
