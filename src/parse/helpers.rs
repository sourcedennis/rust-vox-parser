//! Some project-specific helper functions for the _nom_ parser.


// Local imports
use crate::parse::error::{VoxError, VoxErrorKind};


pub type IResult< I, O > = nom::IResult< I, O, VoxError< I > >;

/// Parses with the given parser iff the boolean is `true`. Otherwise, nothing
/// is parsed and `None` is returned.
pub fn parse_if< 'a, F, T >( b: bool, f: F, input: &'a [u8] ) -> IResult< &'a [u8], Option< T > >
    where
      F : Fn( &'a [u8] ) -> IResult< &'a [u8], T > {
  
  if b {
    let (input, v) = f( input )?;
    nom::IResult::Ok( (input, Some( v ) ) )
  } else {
    nom::IResult::Ok( (input, None) )
  }
}

/// After parsing the value, checks whether the predicate is satisfied. If the
/// predicate is not satisfied, the given error is returned over the `IResult`.
pub fn check< 'a, F, G, T: Copy >( err: VoxErrorKind, f_parse: F, f_check: G, input: &'a [u8] ) -> IResult< &'a [u8], T >
    where
      F : Fn( &'a [u8] ) -> IResult< &'a [u8], T >,
      G : Fn( T ) -> bool {
  
  let (input2, v) = f_parse( input )?;
  vox_assert( err, f_check( v ), input )?;
  nom::IResult::Ok( (input2, v) )
}

/// Assertion. If the assertion is false, the given error is returned over the
/// `IResult`.
/// 
/// Nothing is parsed from the input, which is only necessary for error
/// diagnostics.
pub fn vox_assert< 'a, I >( err: VoxErrorKind, b: bool, input: I ) -> IResult< I, () > {
  if b {
    nom::IResult::Ok( (input, ()) )
  } else {
    failure( input, err )
  }
}

/// Fail parsing with the given error kind.
pub fn failure< I, O >( input: I, kind: VoxErrorKind ) -> IResult<I, O> {
  nom::IResult::Err( nom::Err::Failure( VoxError::new( input, kind ) ) )
}
