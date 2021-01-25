
// Local imports
use crate::data::spec::{Matt, MattType};
use crate::unparse::helpers::{le_u32, le_f32};


/// Writes the payload of the `MATT` chunk. (See [`Matt`])
pub fn chunk_matt< 'a >( dst: &mut Vec< u8 >, m: &Matt ) {
  le_u32( dst, m.id as u32 );

  match m.matt_type {
    MattType::Diffuse => {
      le_u32( dst, 0 ); // diffuse
      le_f32( dst, 1.0 );
    },
    MattType::Metal( w ) => {
      le_u32( dst, 1 ); // metal
      le_f32( dst, w );
    },
    MattType::Glass( w ) => {
      le_u32( dst, 2 ); // glass
      le_f32( dst, w );
    },
    MattType::Emissive( w ) => {
      le_u32( dst, 3 ); // emissive
      le_f32( dst, w );
    }
  };

  let mut property_bits = 0;
  property_bits |= if m.prop_plastic.is_some( )     { 0x01 } else { 0 };
  property_bits |= if m.prop_roughness.is_some( )   { 0x02 } else { 0 };
  property_bits |= if m.prop_specular.is_some( )    { 0x04 } else { 0 };
  property_bits |= if m.prop_ior.is_some( )         { 0x08 } else { 0 };
  property_bits |= if m.prop_attenuation.is_some( ) { 0x10 } else { 0 };
  property_bits |= if m.prop_power.is_some( )       { 0x20 } else { 0 };
  property_bits |= if m.prop_glow.is_some( )        { 0x40 } else { 0 };
  property_bits |= if m.prop_is_total_power         { 0x40 } else { 0 };

  le_u32( dst, property_bits );

  prop_f32( dst, m.prop_plastic );
  prop_f32( dst, m.prop_roughness );
  prop_f32( dst, m.prop_specular );
  prop_f32( dst, m.prop_ior );
  prop_f32( dst, m.prop_attenuation );
  prop_f32( dst, m.prop_power );
  prop_f32( dst, m.prop_glow );
}

fn prop_f32( dst: &mut Vec< u8 >, m_v: Option< f32 > ) {
  if let Some( v ) = m_v {
    le_f32( dst, v );
  }
}
