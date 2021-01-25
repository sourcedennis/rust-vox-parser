//! Parsers for the `.vox` binary format.
//! 
//! The parsers for sub-components within the specification are also exposed.
//! This may be convenient when writing your own parser outer loop. *Most users,
//! however, likely want to use [`parse::file_raw`](file_raw) or
//! [`parse::file_custom`](file_custom).*
//! 
//! This parser uses [`nom`] (v6)
//! 
//! # Example: Parse full scene
//! 
//! See the examples in the [crate root](crate).
//! 
//! # Example: Parse raw chunks
//! 
//! In this example, raw chunks are extracted from the `.vox` file. These raw
//! chunks are then individually parsed with the chunk parser. As not all chunks
//! are described in the specification, parsing some chunks may fail.
//! 
//! ```
//! let (_, raw_chunks) = vox_parser::parse::file_raw( &content ).unwrap( );
//! 
//! for raw_chunk in raw_chunks {
//!   match vox_parser::parse::chunk( raw_chunk.tag, raw_chunk.payload_data ) {
//!     Ok( (_, _parsed_chunk) ) => { println!( "Chunk parsed!" ); },
//!     // Note that unknown chunks will fail to parse. It may be advisable to
//!     // pattern match on the contained `VoxErrorKind::UnknownChunk`.
//!     Err( _err ) => { println!( "Chunk failed to parse" ); }
//!   }
//! }
//! ```


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
mod error;
mod helpers;
mod special;

pub use chunk::{chunk, raw_chunk};
pub use chunk_layr::chunk_layr;
pub use chunk_matl::chunk_matl;
pub use chunk_matt::chunk_matt;
pub use chunk_ntrn::chunk_ntrn;
pub use chunk_nshp::chunk_nshp;
pub use chunk_ngrp::chunk_ngrp;
pub use chunk_pack::chunk_pack;
pub use chunk_rgba::chunk_rgba;
pub use chunk_size::chunk_size;
pub use chunk_xyzi::chunk_xyzi;
pub use special::{string, dict, rotation_u8};
pub use helpers::IResult;
pub use error::{VoxErrorKind, VoxError};

// External library imports
use nom::combinator::{all_consuming};
use nom::bytes::complete::{tag};
use nom::number::complete::{le_u32};
// Local imports
use crate::data::spec::RawChunk;
use crate::data::custom::VoxScene;
use crate::convert::to_custom;
use helpers::failure;


/// Parses a `.vox` file into raw chunks.
/// 
/// See the examples in [`parse`](crate::parse).
pub fn file_raw( input: &[u8] ) -> IResult<&[u8], Vec< RawChunk > > {
  let (input, _) = tag( "VOX " )( input )?;
  let (input, version) = le_u32( input )?;

  if version != 150 {
    return failure( input, VoxErrorKind::FileVersionUnknown( version ) );
  }

  // ## MAIN chunk ##
  let (input, main_chunk) = raw_chunk( Some( b"MAIN" ), input )?;

  if main_chunk.payload_data.len( ) > 0 {
    // The main chunk has no data of itself, only child chunks
    return failure( input, VoxErrorKind::InvalidMainChunk );
  }

  nom::IResult::Ok( (input, main_chunk.children) )
}

/// Parses a `.vox` file into a scene structure.
/// 
/// See the examples in the [crate root](crate).
pub fn file_custom( input: &[u8] ) -> Result< VoxScene, VoxErrorKind > {
  match all_consuming( file_raw )( &input ) {
    nom::IResult::Ok( (_, chunks) ) => to_custom( &chunks ),
    nom::IResult::Err( nom::Err::Error( err ) ) => Err( err.code ),
    nom::IResult::Err( nom::Err::Failure( err ) ) => Err( err.code ),
    nom::IResult::Err( nom::Err::Incomplete( _ ) ) =>
      Err( VoxErrorKind::Nom( nom::error::ErrorKind::Eof ) )
  }
}
