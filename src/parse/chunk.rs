
// External library imports
use nom::do_parse;
use nom::number::complete::le_u32;
use nom::bytes::complete::{tag, take};
use nom::combinator::all_consuming;
use nom::multi::many0;
// Local imports
use crate::data::spec::{Chunk, RawChunk};
use crate::parse::error::VoxErrorKind;
use crate::parse::helpers::{IResult, failure};
use crate::parse::chunk_pack::chunk_pack_all;
use crate::parse::chunk_size::chunk_size_all;
use crate::parse::chunk_xyzi::chunk_xyzi_all;
use crate::parse::chunk_rgba::chunk_rgba_all;
use crate::parse::chunk_matt::chunk_matt_all;
use crate::parse::chunk_ntrn::chunk_ntrn_all;
use crate::parse::chunk_ngrp::chunk_ngrp_all;
use crate::parse::chunk_nshp::chunk_nshp_all;
use crate::parse::chunk_matl::chunk_matl_all;
use crate::parse::chunk_layr::chunk_layr_all;


/// Parses the payload of a chunk into the corresponding [`Chunk`] constructor.
pub fn chunk< 'a >( id: [u8; 4], input: &'a [u8] ) -> IResult< &'a [u8], Chunk > {
  match &id {
    b"PACK" => do_parse!( input, res: chunk_pack_all >> ( Chunk::PACK( res ) ) ),
    b"SIZE" => do_parse!( input, res: chunk_size_all >> ( Chunk::SIZE( res ) ) ),
    b"XYZI" => do_parse!( input, res: chunk_xyzi_all >> ( Chunk::XYZI( res ) ) ),
    b"RGBA" => do_parse!( input, res: chunk_rgba_all >> ( Chunk::RGBA( res ) ) ),
    b"MATT" => do_parse!( input, res: chunk_matt_all >> ( Chunk::MATT( res ) ) ),
    b"nTRN" => do_parse!( input, res: chunk_ntrn_all >> ( Chunk::NTRN( res ) ) ),
    b"nGRP" => do_parse!( input, res: chunk_ngrp_all >> ( Chunk::NGRP( res ) ) ),
    b"nSHP" => do_parse!( input, res: chunk_nshp_all >> ( Chunk::NSHP( res ) ) ),
    b"MATL" => do_parse!( input, res: chunk_matl_all >> ( Chunk::MATL( res ) ) ),
    b"LAYR" => do_parse!( input, res: chunk_layr_all >> ( Chunk::LAYR( res ) ) ),
    _ => failure( input, VoxErrorKind::UnknownChunk( id ) )
  }
}

/// Parses a raw chunk
/// 
/// If the tag is specified, the parser fails if the parsed tag mismatches.
///
/// | # Bytes | Type | Value                            |
/// | ------- | ---- | -------------------------------- |
/// | 1x4     | char | chunk id                         |
/// | 4       | int  | num bytes of chunk content (N)   |
/// | 4       | int  | num bytes of children chunks (M) |
/// | N       |      | chunk content                    |
/// | M       |      | children chunks                  |
pub fn raw_chunk< 'a >( known_tag: Option< &'static [u8;4] >
                      , input: &'a [u8]
                      ) -> IResult< &'a [u8], RawChunk< 'a > > {
  let (input, tag) =
    if let Some( t ) = known_tag {
      tag( t )( input )?
    } else {
      take( 4usize )( input )?
    };
  
  let (input, num_payload_bytes) = le_u32( input )?;
  let (input, num_child_bytes)   = le_u32( input )?;

  let (input, payload_data) = take( num_payload_bytes )( input )?;
  let (input, child_data)   = take( num_child_bytes )( input )?;

  let (_, children) = all_consuming( many0( |i| raw_chunk( None, i ) ) )( child_data )?;

  nom::IResult::Ok( (input, RawChunk { tag: copy4( tag ), payload_data, children } ) )
}

/// Forces the array of size 4 into an array of fixed size 4.
fn copy4( a: &[u8] ) -> [u8; 4] {
  [a[0], a[1], a[2], a[3]]
}
