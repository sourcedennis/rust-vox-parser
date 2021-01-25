//! Functions for conversion between raw data chunks and a custom data structure,
//! which contains the voxel scene represented by those chunks.
//! 
//! Public methods:
//! - [`to_custom`]
//! - [`from_custom`]


// Stdlib imports
use std::collections::HashMap;
// External library imports
use nom::combinator::all_consuming;
// Local imports
use crate::data::spec;
use crate::data::spec::{DEFAULT_PALETTE, RawChunk};
use crate::data::custom;
use crate::data::custom::VoxScene;
use crate::parse::{VoxErrorKind, chunk};


static DEFAULT_ALPHA:  f32 = 0.0;
static DEFAULT_ROUGH:  f32 = 0.0;
static DEFAULT_FLUX:   u32 = 1;
static DEFAULT_METAL:  f32 = 0.0;
static DEFAULT_WEIGHT: f32 = 0.0;
// Note that the offset from 1.0 is stored. So, the value 0.14 indicates an
// actual IOR of 1.14 .
static DEFAULT_IOR:   f32 = 0.0;
static DEFAULT_EMIT:  f32 = 0.0;
static DEFAULT_LDR:   f32 = 0.0;

/// Internal. Enum over the types of nodes in the scene graph.
#[derive(Debug)]
enum ParsedNode< 'a > {
  Transform( spec::TransformNode ),
  Group( spec::GroupNode< 'a > ),
  Shape( spec::ShapeNode< 'a > )
}

/// Internal. Default diffuse material within the palette.
/// 
/// The color is surely overwritten by either values in the RGBA chunk, or the
/// corresponding color in the default palette.
static DEFAULT_MATERIAL: custom::Material =
  custom::Material {
    rgba: (0,0,0,0),
    mat_type: custom::MaterialType::Diffuse
  };

/// Parses and converts [`RawChunk`]s into a [`VoxScene`].
/// 
/// As not all chunks are fully specified, this function ensures unknown chunks
/// are appropriately ignored. Parse errors of known chunks are still reported.
pub fn to_custom< 'a >( chunks: &'a [RawChunk] ) -> Result< VoxScene, VoxErrorKind > {
  // Keep a parsing state and update it while traversing the chunks.

  let mut palette: [custom::Material; 255] = [DEFAULT_MATERIAL; 255];
  for i in 0..255 {
    palette[ i ].rgba = DEFAULT_PALETTE[ i ];
  }

  let mut models: Vec< custom::Model > = Vec::new( );
  let mut latest_size: Option<(u32,u32,u32)> = None;
  let mut layers: Vec< custom::Layer > = Vec::new( );
  let mut parsed_scene: HashMap< u32, ParsedNode< 'a > > = HashMap::new( );

  // Traverse the chunks and update the state
  for raw_chunk in chunks {
    match all_consuming( |i| chunk( raw_chunk.tag, i ) )( raw_chunk.payload_data ) {
      Ok( ( _, spec::Chunk::PACK( _ ) ) ) => { },
      // The size of the next model
      Ok( ( _, spec::Chunk::SIZE( s ) ) ) =>
        if latest_size.is_some( ) {
          return Err( VoxErrorKind::NonAlternatingModel );
        } else {
          latest_size = Some( s );
        },
      // The voxels of a model. The size was given by the previous SIZE chunk.
      Ok( ( _, spec::Chunk::XYZI( xyzi ) ) ) => {
        if let Some( size ) = latest_size {
          models.push( custom::Model { size, xyzi } );
          latest_size = None;
        } else {
          return Err( VoxErrorKind::NonAlternatingModel );
        }
      },
      // Initialize the palette colors.
      Ok( ( _, spec::Chunk::RGBA( rgba ) ) ) => {
        for i in 0..255 {
          palette[ i ].rgba = rgba[ i ];
        }
      },
      // Deprecated MATT chunk, which is present in old files.
      Ok( ( _, spec::Chunk::MATT( m ) ) ) =>
        if m.id == 0 {
          // Technically, this is an error, as palette index 0 may not be used.
          // However, older MagicaVoxel versions produce materials at this
          // index. So ignore this error.
        } else {
          // Note that palette array index 0 is index 1 in the actual palette.
          // (Index 0 represents the "null"-material, which is not stored)
          palette[ m.id as usize - 1 ].mat_type = matt2material( &m );
        },
      Ok( ( _, spec::Chunk::MATL( m ) ) ) =>
        if m.id == 0 {
          // Technically, this is an error, as palette index 0 may not be used.
          // However, older MagicaVoxel versions produce materials at this
          // index. So ignore this error.
        } else {
          // Note that palette array index 0 is index 1 in the actual palette.
          // (Index 0 represents the "null"-material, which is not stored)
          palette[ m.id as usize - 1 ].mat_type = matl2material( &m );
        },
      Ok( ( _, spec::Chunk::LAYR( layr ) ) ) => {
        let uid = layr.id as usize;
        if layers.len( ) <= uid {
          layers.push( custom::Layer { name: "".to_string( ), is_hidden: false } );
        }
        layers[ uid ].name =
          if let Some( name ) = layr.name {
            name.to_string( )
          } else {
            "".to_string( )
          };
        layers[ uid ].is_hidden = layr.is_hidden;
      },
      Ok( ( _, spec::Chunk::NTRN( ntrn ) ) ) => {
        parsed_scene.insert( ntrn.node_id, ParsedNode::Transform( ntrn ) );
      },
      Ok( ( _, spec::Chunk::NGRP( ngrp ) ) ) => {
        parsed_scene.insert( ngrp.node_id, ParsedNode::Group( ngrp ) );
      },
      Ok( ( _, spec::Chunk::NSHP( nshp ) ) ) => {
        parsed_scene.insert( nshp.node_id, ParsedNode::Shape( nshp ) );
      },
      Err( err ) =>
        if let Some( err_code ) = err_code( err ) {
          match err_code.code {
            // Many chunks exported by MagicaVoxel are left out of the
            // specification. For now, just ignore unknown chunks.
            VoxErrorKind::UnknownChunk( _tag ) => { },
            VoxErrorKind::InvalidMatlId( 0 ) => { },
            // MagicaVoxel seems to produce files with a material at index 256
            // in the palette. Yet, this material is invisible in the GUI.
            // Ignore it.
            VoxErrorKind::InvalidMatlId( 256 ) => { },
            _ => { return Err( err_code.code ); }
          }
        } // Ignore `Incomplete`, because it does not happen
    }
  }

  // Compose the data structure from the obtained state

  let graph =
    if parsed_scene.len( ) == 0 {
      // Old MagicaVoxel files contain no scenes. Instead, they contain only a
      // single model. Introduce a simple scene for this kind of file.

      custom::SceneNode {
        rotation: spec::MatRowCols::identity( ),
        translation: (0,0,0),
        layer_id: Some( 0 ),
        node_type:
          if models.len( ) > 0 {
            custom::NodeType::Shape( 0 )
          } else {
            custom::NodeType::Group( Vec::new( ) )
          }
      }
    } else if let Some( sn ) = build_scene( &parsed_scene, 0, models.len( ) ) {
      // Note that node 0 is always the graph root. I think?
      sn
    } else {
      return Err( VoxErrorKind::InvalidScene )
    };

  Ok( 
    custom::VoxScene {
      palette, models, graph, layers
    }
  )
}

/// Converts a [`VoxScene`] back into a vector of [`Chunk`](spec::Chunk)s.
pub fn from_custom< 'a >( s: &'a VoxScene ) -> Vec< spec::Chunk< 'a > > {
  // The order of the chunks is made to correspond to those of files produces by
  // MagicaVoxel: models, palette, scene, layers, materials.

  let mut chunks = Vec::new( );

  // First write the models
  for m in &s.models {
    chunks.push( spec::Chunk::SIZE( m.size ) );
    chunks.push( spec::Chunk::XYZI( m.xyzi.clone( ) ) );
  }

  // Then write the palette
  let mut rgba: [(u8,u8,u8,u8); 255] = [(0,0,0,0); 255];
  for i in 0..255 {
    rgba[ i ] = s.palette[ i ].rgba;
  }
  chunks.push( spec::Chunk::RGBA( rgba ) );

  // Then write the scene
  let mut i = 0;
  export_scene( &mut chunks, &mut i, &s.graph );

  // Layers
  for i in 0..s.layers.len( ) {
    chunks.push(
      spec::Chunk::LAYR(
        spec::Layr {
          id: i as u32,
          name: Some( &s.layers[ i ].name ),
          is_hidden: s.layers[ i ].is_hidden
        }
      )
    )
  }

  // Materials
  for i in 0..255 {
    chunks.push(
      spec::Chunk::MATL( material2matl( i + 1, &s.palette[ i as usize ].mat_type ) )
    )
  }

  chunks
}

/// Extracts the contained error structure from the nom error object.
/// 
/// If the object contains no error specification, `None` is returned.
fn err_code< E >( err: nom::Err< E > ) -> Option< E > {
  match err {
    nom::Err::Error( err )    => Some( err ),
    nom::Err::Failure( err )  => Some( err ),
    nom::Err::Incomplete( _ ) => None
  }
}

/// Builds a scene tree representation from the scene chunks (nTRN/nSHP/nGRP).
/// 
/// Returns `None` if a non-existing scene node or model is referenced.
fn build_scene< 'a >(
    scene_nodes: &HashMap< u32, ParsedNode< 'a > >,
    node_id: u32,
    num_models:  usize
  ) -> Option< custom::SceneNode > {

  match scene_nodes.get( &(node_id as u32) ) {
    Some( ParsedNode::Transform( n ) ) => {
      let node_type =
        match scene_nodes.get( &n.child_node_id ) {
          Some( ParsedNode::Group( group_node ) ) => {
            let mut children = Vec::with_capacity( group_node.child_nodes.len( ) );
            for child_index in &group_node.child_nodes {
              if let Some( c ) = build_scene( scene_nodes, *child_index, num_models ) {
                children.push( c );
              } else {
                return None;
              }
            }
            custom::NodeType::Group( children )
          },
          Some( ParsedNode::Shape( s ) ) => {
            if s.model_id < num_models as u32 {
              custom::NodeType::Shape( s.model_id )
            } else {
              // Referenced model does not exist
              return None;
            }
          },
          _ => { return None; }
        };

      Some(
        custom::SceneNode {
          rotation:    n.rotation,
          translation: n.translation,
          layer_id:    n.layer_id,
          node_type
        }
      )
    },
    _ => None
  }
}

/// Traverses the scene graph downward, and writes the encountered nodes as
/// chunks to the `dst` vector.
/// 
/// `i` is an increasing value which represents the number of written graph
/// nodes. The function returns the node index of the written `scene` node.
fn export_scene< 'a >( dst: &mut Vec< spec::Chunk< 'a > >, i: &mut u32, scene: &custom::SceneNode ) -> u32 {
  let node_id = *i;

  let transform_node =
    spec::TransformNode {
      node_id:       node_id,
      name:          None,
      is_hidden:     false,
      child_node_id: node_id + 1,
      layer_id:      scene.layer_id,
      rotation:      scene.rotation,
      translation:   scene.translation
    };
  dst.push( spec::Chunk::NTRN( transform_node ) );
  
  *i = *i + 1;

  match &scene.node_type {
    custom::NodeType::Shape( model_id ) => {
      dst.push(
        spec::Chunk::NSHP(
          spec::ShapeNode {
            node_id: *i,
            attributes: HashMap::new( ),
            model_id: *model_id,
            model_attributes: HashMap::new( )
          }
        )
      );
      *i = *i + 1;
    },
    custom::NodeType::Group( children ) => {
      let chunk_id = dst.len( );

      let mut group =
        spec::GroupNode {
          node_id:     *i,
          attributes:  HashMap::new( ),
          child_nodes: Vec::with_capacity( children.len( ) )
        };

      // Reserve the spot for the group. Children are yet empty
      dst.push( spec::Chunk::NGRP( group.clone( ) ) );

      *i = *i + 1;

      for c in children {
        group.child_nodes.push( export_scene( dst, i, &c ) );
      }

      // Actually store it with the contents
      dst[ chunk_id ] = spec::Chunk::NGRP( group );
    }
  }

  node_id
}

/// Converts the `MATL` chunk to a material type in the custom structure
/// ([`custom::Material`]).
fn matl2material( m: &spec::Matl ) -> custom::MaterialType {
  match m.prop_type {
    spec::MatlType::Diffuse => custom::MaterialType::Diffuse,
    spec::MatlType::Metal =>
      custom::MaterialType::Metal(
        custom::MetalMaterial {
          prop_rough: to_val( DEFAULT_ROUGH, m.prop_rough ),
          prop_ior:   to_val( DEFAULT_IOR,   m.prop_ior ),
          prop_metal: to_val( DEFAULT_METAL, m.prop_metal )
        }
      ),
    spec::MatlType::Glass =>
      custom::MaterialType::Glass(
        custom::GlassMaterial {
          prop_rough:  to_val( DEFAULT_ROUGH,  m.prop_rough ),
          prop_ior:    to_val( DEFAULT_IOR,    m.prop_ior ),
          prop_weight: to_val( DEFAULT_WEIGHT, m.prop_weight )
        }
      ),
    spec::MatlType::Emit =>
      custom::MaterialType::Emit(
        custom::EmitMaterial {
          prop_emit: to_val( DEFAULT_EMIT, m.prop_emit ),
          prop_flux: to_val( DEFAULT_FLUX, m.prop_flux ),
          prop_ldr:  to_val( DEFAULT_LDR,  m.prop_ldr )
        }
      ),
    spec::MatlType::Blend =>
      custom::MaterialType::Blend(
        custom::BlendMaterial {
          prop_rough: to_val( DEFAULT_ROUGH, m.prop_rough ),
          prop_metal: to_val( DEFAULT_METAL, m.prop_metal ),
          prop_ior:   to_val( DEFAULT_IOR,   m.prop_ior ),
          prop_alpha: to_val( DEFAULT_ALPHA, m.prop_alpha )
        }
      ),
    spec::MatlType::Media => custom::MaterialType::Media
  }
}

/// Converts the `MATT` chunk to a material type in the custom structure
/// ([`custom::Material`]).
fn matt2material( m: &spec::Matt ) -> custom::MaterialType {
  match m.matt_type {
    spec::MattType::Diffuse => custom::MaterialType::Diffuse,
    spec::MattType::Metal( w ) =>
      custom::MaterialType::Metal(
        custom::MetalMaterial {
          prop_rough: to_val( DEFAULT_ROUGH, m.prop_roughness ),
          prop_ior:   to_val( DEFAULT_IOR,   m.prop_ior ),
          prop_metal: w,
        }
      ),
    spec::MattType::Glass( w ) =>
      custom::MaterialType::Glass(
        custom::GlassMaterial {
          prop_rough:  to_val( DEFAULT_ROUGH, m.prop_roughness ),
          prop_ior:    to_val( DEFAULT_IOR,   m.prop_ior ),
          prop_weight: w,
        }
      ),
    spec::MattType::Emissive( w ) =>
      // Very unsure about these conversions from the deprecated MATT
      // chunk to the new format. These guesses are as good as any.
      custom::MaterialType::Emit(
        custom::EmitMaterial {
          prop_emit: w,
          // The "power" slider corresponds to the flux field.
          prop_flux: to_val( DEFAULT_FLUX, m.prop_power.map( |f| f as u32 ) ),
          prop_ldr:  0.0,
        }
      )
  }
}

/// Converts the material back to the `MATL` chunk.
fn material2matl( id: u8, t: &custom::MaterialType ) -> spec::Matl {
  match t {
    custom::MaterialType::Diffuse =>
      spec::Matl::new( id, spec::MatlType::Diffuse ),
    custom::MaterialType::Metal( m ) => {
      let mut out = spec::Matl::new( id, spec::MatlType::Metal );
      out.prop_rough = from_val( DEFAULT_ROUGH, m.prop_rough );
      out.prop_ior   = from_val( DEFAULT_IOR, m.prop_ior );
      out.prop_metal = from_val( DEFAULT_METAL, m.prop_metal );
      out
    },
    custom::MaterialType::Glass( m ) => {
      let mut out = spec::Matl::new( id, spec::MatlType::Glass );
      out.prop_rough  = from_val( DEFAULT_ROUGH, m.prop_rough );
      out.prop_ior    = from_val( DEFAULT_IOR, m.prop_ior );
      out.prop_weight = from_val( DEFAULT_ALPHA, m.prop_weight );
      out
    },
    custom::MaterialType::Emit( m ) => {
      let mut out = spec::Matl::new( id, spec::MatlType::Emit );
      out.prop_emit = from_val( DEFAULT_EMIT, m.prop_emit );
      out.prop_flux = from_val( DEFAULT_FLUX, m.prop_flux );
      out.prop_ldr  = from_val( DEFAULT_LDR, m.prop_ldr );
      out
    },
    custom::MaterialType::Blend( m ) => {
      let mut out = spec::Matl::new( id, spec::MatlType::Blend );
      out.prop_rough = from_val( DEFAULT_ROUGH, m.prop_rough );
      out.prop_metal = from_val( DEFAULT_METAL, m.prop_metal );
      out.prop_ior   = from_val( DEFAULT_IOR, m.prop_ior );
      out.prop_alpha = from_val( DEFAULT_ALPHA, m.prop_alpha );
      out
    },
    custom::MaterialType::Media =>
      spec::Matl::new( id, spec::MatlType::Media )
  }
}

/// Converts an explicit value back to an optional value; This means `None` is
/// returned if the value equals its default value.
/// 
/// The data structures often store explicit values. However, inside the file,
/// a value need not be stored if it is assigned its default value.
fn from_val< T: PartialEq >( default_val: T, v: T ) -> Option< T > {
  if default_val == v {
    None
  } else {
    Some( v )
  }
}

/// Converts an optional value to it's explicit value.
/// 
/// The file need no explicitly store values assigned with their default value.
/// However, in the data structure it is convenient to have explicit values.
fn to_val< T >( default_val: T, v: Option< T > ) -> T {
  v.unwrap_or( default_val )
}
