
// Stdlib imports
use std::collections::HashMap;
// Local imports
use crate::data::spec::{Matl, MatlType};
use crate::unparse::helpers::{le_u32};
use crate::unparse::special::{dict};


/// Writes the payload of the `MATL` chunk. (See [`Matl`])
pub fn chunk_matl< 'a >( dst: &mut Vec< u8 >, m: &Matl ) {
  le_u32( dst, m.id as u32 );

  let mut properties = HashMap::new( );

  let prop_type_str =
    match m.prop_type {
      MatlType::Diffuse => "_diffuse",
      MatlType::Metal   => "_metal",
      MatlType::Glass   => "_glass",
      MatlType::Emit    => "_emit",
      MatlType::Blend   => "_blend",
      MatlType::Media   => "_media"
    };
  properties.insert( "_type", prop_type_str.to_string( ) );

  prop_f32( &mut properties,  "_weight",  m.prop_weight );
  prop_f32( &mut properties,  "_rough",   m.prop_rough );
  prop_f32( &mut properties,  "_spec",    m.prop_spec );
  prop_f32( &mut properties,  "_ior",     m.prop_ior );
  prop_f32( &mut properties,  "_att",     m.prop_att );
  prop_u32( &mut properties,  "_flux",    m.prop_flux );
  prop_f32( &mut properties,  "_d",       m.prop_density );
  prop_f32( &mut properties,  "_alpha",   m.prop_alpha );
  prop_f32( &mut properties,  "_emit",    m.prop_emit );
  prop_f32( &mut properties,  "_ldr",     m.prop_ldr );
  prop_f32( &mut properties,  "_metal",   m.prop_metal );
  prop_bool( &mut properties, "_plastic", m.prop_plastic );

  dict( dst, &properties );
}


// Helpers

/// Writes the a f32 value to the dictionary, if it is present. When it is
/// `None`, nothing is changed.
fn prop_f32< 'a >( dst: &mut HashMap< &'a str, String >, k: &'a str, v: Option< f32 > ) {
  if let Some( v ) = v {
    dst.insert( k, v.to_string( ) );
  }
}

/// Writes the a u32 value to the dictionary, if it is present. When it is
/// `None`, nothing is changed.
fn prop_u32< 'a >( dst: &mut HashMap< &'a str, String >, k: &'a str, v: Option< u32 > ) {
  if let Some( v ) = v {
    dst.insert( k, v.to_string( ) );
  }
}

/// Writes the a boolean to the dictionary.
fn prop_bool< 'a >( dst: &mut HashMap< &'a str, String >, k: &'a str, v: bool ) {
  if v {
    dst.insert( k, "1".to_string( ) );
  } else {
    dst.insert( k, "0".to_string( ) );
  }
}
