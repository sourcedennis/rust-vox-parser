
// Stdlib imports
use std::str::FromStr;
use std::collections::HashMap;
// External library imports
use nom::combinator::all_consuming;
use nom::number::complete::le_i32;
// Local imports
use crate::data::spec::{Matl, MatlType};
use crate::parse::error::VoxErrorKind;
use crate::parse::helpers::{IResult, failure};
use crate::parse::special::dict;


/// Parses the payload of a MATL chunk, or fails if bytes are remaining.
/// See `chunk_matl`.
pub fn chunk_matl_all< 'a >( input: &'a [u8] ) -> IResult< &'a [u8], Matl > {
  all_consuming( chunk_matl )( input )
}

/// Parses the payload of the `MATL` chunk. (See [`Matl`])
pub fn chunk_matl< 'a >(input: &'a [u8]) -> IResult<&'a [u8], Matl> {
  let (input, id) = le_i32( input )?;
  // put remainder in `input2`, so errors may reference `input`.
  let (input2, properties) = dict( input )?;
  
  // Interestingly, files produced by MagicaVoxel may contain materials
  // referencing palette index 256. However, no such index exists. Or, at least,
  // it cannot be displayed in the MagicaVoxel GUI.

  if id < 0 || id > 255 {
    return failure( input, VoxErrorKind::InvalidMatlId( id ) )
  }

  let prop_type =
    match properties.get( "_type" ) {
      Some( &"_diffuse" ) => MatlType::Diffuse,
      Some( &"_metal" )   => MatlType::Metal,
      Some( &"_glass" )   => MatlType::Glass,
      Some( &"_emit" )    => MatlType::Emit,
      Some( &"_blend" )   => MatlType::Blend,
      Some( &"_media" )   => MatlType::Media,
      _ => {
        return failure( input, VoxErrorKind::InvalidMatlType );
      }
    };
  // println!( "MATL {} {:?}", id, properties );

  let (_, prop_weight)  = prop_f32( &properties,  "_weight", |v| v >= 0.0 && v <= 1.0, input )?;
  let (_, prop_rough)   = prop_f32( &properties,  "_rough",  |_| true, input )?;
  let (_, prop_spec)    = prop_f32( &properties,  "_spec",   |_| true, input )?;
  let (_, prop_ior)     = prop_f32( &properties,  "_ior",    |_| true, input )?;
  let (_, prop_att)     = prop_f32( &properties,  "_att",    |_| true, input )?;
  let (_, prop_flux)    = prop_u32( &properties,  "_flux", input )?;
  let (_, prop_density) = prop_f32( &properties,  "_d",      |_| true, input )?;
  let (_, prop_alpha)   = prop_f32( &properties,  "_alpha",  |_| true, input )?;
  let (_, prop_emit)    = prop_f32( &properties,  "_emit",   |_| true, input )?;
  let (_, prop_ldr)     = prop_f32( &properties,  "_ldr",    |_| true, input )?;
  let (_, prop_metal)   = prop_f32( &properties,  "_metal",  |_| true, input )?;
  let (_, prop_plastic) = prop_bool( &properties, "_plastic", input )?;

  let matl =
    Matl {
      id: id as u8,
      
      prop_type,
      prop_weight,
      prop_rough,
      prop_spec,
      prop_ior,
      prop_att,
      prop_flux,
      prop_density,
      prop_alpha,
      prop_emit,
      prop_ldr,
      prop_metal,
      prop_plastic
    };

  nom::IResult::Ok((input2, matl))
}


// Helpers

/// Parses a f32 string into a f32.
/// 
/// Nothing is parsed from the `input`; that parameter is only relevant in case
/// of error (where it is passed to the caller).
fn prop_f32< 'a, F >(
  props: &HashMap< &'a str, &'a str >,
  key: &'static str,
  f_check: F,
  input: &'a [u8] )
  -> IResult< &'a [u8], Option< f32 > >
  where F : Fn( f32 ) -> bool {

  if let Some( val ) = props.get( key ) {
    if let Ok( val_f32 ) = f32::from_str( val ) {
      if f_check( val_f32 ) {
        return nom::IResult::Ok((input, Some(val_f32)));
      } else {
        return failure( input, VoxErrorKind::InvalidMatlProperty );
      }
    } else {
      return failure( input, VoxErrorKind::InvalidMatlProperty );
    }
  } else {
    return nom::IResult::Ok((input, None));
  }
}

/// Parses a f32 string into a f32.
/// 
/// Nothing is parsed from the `input`; that parameter is only relevant in case
/// of error (where it is passed to the caller).
fn prop_u32< 'a >(
  props: &HashMap< &'a str, &'a str >,
  key: &'static str,
  input: &'a [u8] )
  -> IResult< &'a [u8], Option< u32 > >  {

  if let Some( val ) = props.get( key ) {
    if let Ok( val_u32 ) = u32::from_str( val ) {
      return nom::IResult::Ok((input, Some(val_u32)));
    } else {
      return failure( input, VoxErrorKind::InvalidMatlProperty );
    }
  } else {
    return nom::IResult::Ok((input, None));
  }
}

fn prop_bool< 'a >(
  props: &HashMap< &'a str, &'a str >,
  key: &'static str,
  input: &'a [u8] )
  -> IResult< &'a [u8], bool > {

  match props.get( key ) {
    Some( &"0" ) => nom::IResult::Ok((input, false)),
    Some( &"1" ) => nom::IResult::Ok((input, true)),
    None => nom::IResult::Ok((input, false)),
    _ => failure( input, VoxErrorKind::InvalidMatlProperty )
  }
}
