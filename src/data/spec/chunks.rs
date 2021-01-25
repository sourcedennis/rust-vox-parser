//! Data structures for the chunks contained within a .vox file.
//! 
//! The data format specification given in the documentation are verbatim copied
//! from their respective `.vox` specification files. The syntax in both these
//! files varies slightly.
//! - https://github.com/ephtracy/voxel-model/blob/master/MagicaVoxel-file-format-vox.txt
//! - https://github.com/ephtracy/voxel-model/blob/master/MagicaVoxel-file-format-vox-extension.txt


// Stdlib imports
use std::collections::HashMap;
// Local imports
use crate::data::spec::special::MatRowCols;


/// Enum for chunks contained in a `.vox` file.
/// 
/// The chunks that are not included in the specification cannot be represented
/// by this enum; this includes some (unspecified) chunks produces by
/// MagicaVoxel.
#[derive(Debug)]
pub enum Chunk< 'a > {
  PACK( u32 ),
  SIZE( (u32,u32,u32) ),
  XYZI( Vec< (u8,u8,u8,u8) > ),
  RGBA( [(u8,u8,u8,u8); 255] ),
  MATT( Matt ),
  NTRN( TransformNode ),
  NGRP( GroupNode< 'a > ),
  NSHP( ShapeNode< 'a > ),
  MATL( Matl ),
  LAYR( Layr< 'a > )
}

impl< 'a > Chunk< 'a > {
  /// Returns the tag bytes of the chunk.
  pub fn tag( &self ) -> [u8; 4] {
    match self {
      Chunk::PACK( _ ) => *b"PACK",
      Chunk::SIZE( _ ) => *b"SIZE",
      Chunk::XYZI( _ ) => *b"XYZI",
      Chunk::RGBA( _ ) => *b"RGBA",
      Chunk::MATT( _ ) => *b"MATT",
      Chunk::NTRN( _ ) => *b"nTRN",
      Chunk::NGRP( _ ) => *b"nGRP",
      Chunk::NSHP( _ ) => *b"nSHP",
      Chunk::MATL( _ ) => *b"MATL",
      Chunk::LAYR( _ ) => *b"LAYR"
    }
  }
}

/// Raw chunk; It's payload and children are not yet parsed.
/// 
/// This structure is used as an intermediate representation before parsing into
/// a [`Chunk`]. However, as not all chunks are included in the specification
/// (e.g., `rOBJ`), it may be beneficial to keep this representation around.
/// 
/// | # Bytes  | Type       | Value                            |
/// | -------- | ---------- | -------------------------------- |
/// | 1x4      | char       | chunk id                         |
/// | 4        | int        | num bytes of chunk content (N)   |
/// | 4        | int        | num bytes of children chunks (M) |
/// | N        |            | chunk content                    |
/// | M        |            | children chunks                  |
#[derive(Debug)]
pub struct RawChunk< 'a > {
  /// Chunk tag (e.g., `PACK` or `RGBA`)
  pub tag          : [u8; 4],
  /// Payload data. This is often parsed by into a corresponding `Chunk`.
  pub payload_data : &'a [u8],
  /// Bytes representing the children. In practise, only the `MAIN` chunk has
  /// children.
  pub children     : Vec< RawChunk< 'a > >
}


// ------------
//  MATT chunk
// ------------

/// `MATT` chunk. A material within the palette. (_deprecated_ in favor of
/// [`Matl`])
/// 
/// Note that this chunk is _deprecated_ in recent MagicaVoxel versions, which
/// use the newer MATL chunk instead.
///
/// | # Bytes  | Type       | Value                     |
/// | -------- | ---------- | ------------------------- |
/// | 4        | int        | id [1-255]                |
/// | 4        | int        | material type             |
/// | 4        | float      | material weight           |
/// | 4        | int        | property bits             |
/// | 4 * N    | float      | normalized property value |
#[derive(Debug)]
pub struct Matt {
  pub id                  : u8,
  pub matt_type           : MattType,
  pub prop_plastic        : Option< f32 >,
  pub prop_roughness      : Option< f32 >,
  pub prop_specular       : Option< f32 >,
  pub prop_ior            : Option< f32 >,
  pub prop_attenuation    : Option< f32 >,
  pub prop_power          : Option< f32 >,
  pub prop_glow           : Option< f32 >,
  pub prop_is_total_power : bool
}

/// Material type for the `MATT` chunk. (See [`Matt`])
/// 
/// The stored value represents the _weight_ of the value. (For diffuse
/// materials this weight is always 1.0)
#[derive(Debug)]
pub enum MattType {
  Diffuse,
  /// (0.0 - 1.0] - blend between metal and diffuse material
  Metal( f32 ),
  /// (0.0 - 1.0] - blend between glass and diffuse material
  Glass( f32 ),
  /// (0.0 - 1.0] - emission
  Emissive( f32 )
}


// ------------
//  nTRN chunk
// ------------

/// `nTRN` chunk. Transform node in scene graph.
/// 
/// Its immediate child is either a group (nGRP) or shape (nSHP) node.
/// 
/// As transform nodes currently contain only a single frame, the frames are
/// _not_ independently stored.
///
/// ```
/// int32	: node id
/// DICT	: node attributes
///         (_name : string)
///         (_hidden : 0/1)
/// int32 : child node id
/// int32 : reserved id (must be -1)
/// int32	: layer id
/// int32	: num of frames (must be 1)
/// DICT	: frame attributes
///         (_r : int8) ROTATION
///         (_t : int32x3) translation
/// ```
#[derive(Debug)]
pub struct TransformNode {
  pub node_id       : u32,
  pub name          : Option< String >,
  pub is_hidden     : bool,
  pub child_node_id : u32,
  pub layer_id      : Option< u32 >,

  // Frame
  pub rotation    : MatRowCols,
  pub translation : (i32,i32,i32)
}


// ------------
//  nGRP chunk
// ------------

/// `nGRP` chunk. Group node in the scene graph.
/// 
/// Its immediate children are all transform nodes.
/// 
/// ```
/// int32 : node id
/// DICT  : node attributes
/// int32 : num of children nodes
/// {
///   int32 : child node id
/// }xN
/// ```
#[derive(Debug, Clone)]
pub struct GroupNode< 'a > {
  pub node_id     : u32,
  pub attributes  : HashMap< &'a str, &'a str >,
  pub child_nodes : Vec< u32 >
}


// ------------
//  nSHP chunk
// ------------

/// `nSHP` chunk. Shape node in the scene graph.
/// 
/// It references a single model in the list of `SIZE`/`XYZI` chunks.
/// 
/// ```
/// int32	: node id
/// DICT	: node attributes
/// int32 	: num of models (must be 1)
/// {
/// int32	: model id
/// DICT	: model attributes : reserved
/// }xN
/// ```
#[derive(Debug)]
pub struct ShapeNode< 'a > {
  pub node_id    : u32,
  pub attributes : HashMap< &'a str, &'a str >,

  pub model_id         : u32,
  pub model_attributes : HashMap< &'a str, &'a str >
}


// ------------
//  MATL chunk
// ------------

/// `MATL` chunk. A material within the palette.
/// 
/// Replaces the deprecated MATT chunk (`Matt`).
/// 
/// Not all properties are included in the specification; some of these are
/// inferred through observation from existing `.vox` files.
/// 
/// ```
/// int32	: material id
/// DICT	: material properties
///         (_type    : str) _diffuse, _metal, _glass, _emit, _blend, _media
///         (_weight  : float) range 0 ~ 1
///         (_rough   : float)
///         (_spec    : float)
///         (_ior     : float)
///         (_att     : float)
///         (_flux    : float)
///         (_density : float)
///         (_alpha   : float)
///         (_emit    : float)
///         (_ldr     : float)
///         (_metal   : float)
///         (_plastic)
/// ```
#[derive(Debug)]
pub struct Matl {
  pub id           : u8,
  pub prop_type    : MatlType,
  pub prop_weight  : Option< f32 >,
  pub prop_rough   : Option< f32 >,
  pub prop_spec    : Option< f32 >,
  pub prop_ior     : Option< f32 >,
  pub prop_att     : Option< f32 >,
  pub prop_flux    : Option< u32 >,
  pub prop_density : Option< f32 >,
  pub prop_alpha   : Option< f32 >,
  pub prop_emit    : Option< f32 >,
  pub prop_ldr     : Option< f32 >,
  pub prop_metal   : Option< f32 >,
  pub prop_plastic : bool
}

impl Matl {
  /// Constructs a material with the given id and type, but _without_
  /// properties.
  pub fn new( id: u8, prop_type: MatlType ) -> Matl {
    Matl {
      id,
      prop_type,
      prop_weight: None,
      prop_rough: None,
      prop_spec: None,
      prop_ior: None,
      prop_att: None,
      prop_flux: None,
      prop_density: None,
      prop_alpha: None,
      prop_emit: None,
      prop_ldr: None,
      prop_metal: None,
      prop_plastic: false
    }
  }
}

/// Material type for the `MATL` chunk. (See [`Matl`])
#[derive(Debug)]
pub enum MatlType {
  Diffuse,
  Metal,
  Glass,
  Emit,
  Blend, // blends between glass and metal
  Media // clouds
}


// ------------
//  LAYR chunk
// ------------

/// `LAYR` chunk. A layer in these scene.
/// 
/// Every layer may contain multiple transform nodes from within the scene
/// graph. (See [`TransformNode`])
/// 
/// ```
/// int32 : layer id
/// DICT  : layer attributes
///         (_name : string)
///         (_hidden : 0/1)
/// int32 : reserved id, must be -1
/// ```
#[derive(Debug)]
pub struct Layr< 'a > {
  pub id        : u32,
  pub name      : Option< &'a str >,
  pub is_hidden : bool
}
