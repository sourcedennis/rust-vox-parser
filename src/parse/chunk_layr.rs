
// External library imports
use nom::combinator::all_consuming;
use nom::number::complete::{le_u32, le_i32};
// Local imports
use crate::data::spec::Layr;
use crate::parse::error::VoxErrorKind;
use crate::parse::helpers::{IResult, failure};
use crate::parse::special::dict;


/// Parses the payload of a LAYR chunk, or fails if bytes are remaining.
/// See `chunk_layr`.
pub fn chunk_layr_all< 'a >( input: &'a [u8] ) -> IResult< &'a [u8], Layr< 'a > > {
  all_consuming( chunk_layr )( input )
}

/// Parses the payload of the `LAYR` chunk. (See [`Layr`])
pub fn chunk_layr<'a>(input: &'a [u8]) -> IResult<&'a [u8], Layr< 'a > > {
  let (input, layer_id)    = le_u32( input )?;

  let (input, attributes)  = dict( input )?;
  let (input, reserved_id) = le_i32( input )?;

  if reserved_id != -1 {
    return failure( input, VoxErrorKind::InvalidLayrReserved( reserved_id ) );
  }

  let name =
    if let Some( name ) = attributes.get( "_name" ) {
      Some( *name )
    } else {
      None
    };

  let is_hidden =
    match attributes.get( "_is_hidden" ) {
      Some( &"0" ) => false,
      Some( &"1" ) => true,
      None => false,
      _ => return failure( input, VoxErrorKind::InvalidLayrProperty )
    };

  let layr =
    Layr {
      id: layer_id,
      name,
      is_hidden
    };

  nom::IResult::Ok((input, layr))
}
