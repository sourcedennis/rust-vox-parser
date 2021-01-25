
// Local imports
use crate::unparse::chunk_pack::chunk_pack;
use crate::unparse::chunk_size::chunk_size;
use crate::unparse::chunk_xyzi::chunk_xyzi;
use crate::unparse::chunk_rgba::chunk_rgba;
use crate::unparse::chunk_matt::chunk_matt;
use crate::unparse::chunk_ntrn::chunk_ntrn;
use crate::unparse::chunk_ngrp::chunk_ngrp;
use crate::unparse::chunk_nshp::chunk_nshp;
use crate::unparse::chunk_matl::chunk_matl;
use crate::unparse::chunk_layr::chunk_layr;
use crate::data::spec::Chunk;


/// Writes the _payload_ of the given chunk as bytes. Returns `false` when this
/// fails.
pub fn chunk< 'a >( dst: &mut Vec< u8 >, c: &Chunk< 'a > ) {
  match c {
    Chunk::PACK( c ) => chunk_pack( dst, c ),
    Chunk::SIZE( c ) => chunk_size( dst, c ),
    Chunk::XYZI( c ) => chunk_xyzi( dst, c ),
    Chunk::RGBA( c ) => chunk_rgba( dst, c ),
    Chunk::MATT( c ) => chunk_matt( dst, c ),
    Chunk::NTRN( c ) => chunk_ntrn( dst, c ),
    Chunk::NGRP( c ) => chunk_ngrp( dst, c ),
    Chunk::NSHP( c ) => chunk_nshp( dst, c ),
    Chunk::MATL( c ) => chunk_matl( dst, c ),
    Chunk::LAYR( c ) => chunk_layr( dst, c )
  };
}
