//! Custom data structure for the scene represented by a `.vox` file.
//! 
//! This data structure is _not_ strictly spec-conformant, but is derived from
//! the `.vox` chunk specifications. These structures exist as it is easier to
//! work with than raw chunks. See also the [examples](crate) in the root.
//! 
//! Note that the _z_ dimension references the gravity direction (as per spec).
//! 
//! The root structure is [`VoxScene`], which contains the palette, models,
//! scene graph, and layers.


use crate::data::spec::MatRowCols;


/// Represents a scene described by a `.vox` file.
pub struct VoxScene {
  /// The material palette. Every voxel in the scene has an index referencing
  /// into this palette.
  /// 
  /// # WARNING
  /// Palette index 0 does not exist and is thus not stored. Array
  /// index 0 corresponds to palette index 1. This means:
  /// ```
  /// let (x,y,z,i) = model.xyzi[ 42 ];
  /// let color = scene.palette[ i - 1 ];
  /// ```
  pub palette : [Material; 255],

  /// Individual voxel models
  pub models  : Vec< Model >,

  /// The scene graph (actually a tree). This graph positions individual models
  /// into a larger composed scene.
  pub graph   : SceneNode,

  /// Editing layers. These layers are relevant for editing, where different
  /// fragments of the scene reside on different layers. Layers can individually
  /// change visibility.
  /// 
  /// While the file format supports any number of layers, MagicaVoxel supports
  /// exactly 8 layers.
  pub layers  : Vec< Layer >
}

/// A voxel model
pub struct Model {
  /// Size of the model `(x_size, y_size, z_size)`. _z_ is the gravity
  /// direction.
  pub size : (u32, u32, u32),

  /// (x,y,z,color_idx). The color indexes references elements in the
  /// `VoxFile#palette`, with a negative offset of 1:
  /// ```
  /// let (x,y,z,i) = model.xyzi[ 42 ];
  /// let color = vox_file.palette[ i - 1 ];
  /// ```
  /// A color index of 0 is invalid.
  pub xyzi : Vec< (u8, u8, u8, u8) >
}

/// A layer in the scene.
/// 
/// MagicaVoxel supports exactly 8 layers.
pub struct Layer {
  pub name      : String,
  pub is_hidden : bool
}

/// A node in the voxel scene graph.
/// 
/// This condenses the nTRN and nSHP/nGRP nodes together.
pub struct SceneNode {
  pub rotation    : MatRowCols,
  pub translation : (i32,i32,i32),
  pub layer_id    : Option< u32 >,
  pub node_type   : NodeType
}

/// An enum for the different types of nodes in the scene graph. (Used by
/// [`SceneNode`])
pub enum NodeType {
  /// A group of nodes in the scene graph.
  Group( Vec< SceneNode > ),
  /// A model positioned by the scene graph.
  /// 
  /// The shape index references a model in [`VoxScene`]
  Shape( u32 )
}

/// A single material in the palette.
/// 
/// This representation roughly abstracts over both the `MATT` and `MATL` chunks.
#[derive(Debug,Copy,Clone)]
pub struct Material {
  pub rgba     : (u8, u8, u8, u8),
  pub mat_type : MaterialType
}

/// An enum for the different types of materials. (Used by [`Material`])
/// 
/// Note that every material still has a color (which is contained in
/// `Material`).
#[derive(Debug,Copy,Clone)]
pub enum MaterialType {
  Diffuse,
  Metal( MetalMaterial ),
  Glass( GlassMaterial ),
  Emit( EmitMaterial ),
  Blend( BlendMaterial ),
  /// Unsure about these. In files, stores: _d _ior _rough
  Media
}

/// A metallic material
#[derive(Debug,Copy,Clone)]
pub struct MetalMaterial {
  pub prop_rough  : f32,
  /// Index-of-Refraction.
  /// 
  /// WARNING: The offset from 1.0 is stored. So, when the value is 0.14, the
  ///   actual IOR is 1.14.
  pub prop_ior    : f32,
  pub prop_metal  : f32
}

/// A semi-transparent material.
#[derive(Debug,Copy,Clone)]
pub struct GlassMaterial {
  pub prop_rough  : f32,
  /// Index-of-Refraction.
  /// 
  /// WARNING: The offset from 1.0 is stored. So, when the value is 0.14, the
  ///   actual IOR is 1.14.
  pub prop_ior    : f32,
  pub prop_weight : f32
}

/// An illuminative material.
#[derive(Debug,Copy,Clone)]
pub struct EmitMaterial {
  pub prop_emit : f32,
  pub prop_flux : u32, // power slider
  pub prop_ldr  : f32
}

/// A material that blends between metallic and transparent.
#[derive(Debug,Copy,Clone)]
pub struct BlendMaterial {
  pub prop_rough : f32,
  pub prop_metal : f32,
  /// Index-of-Refraction.
  /// 
  /// WARNING: The offset from 1.0 is stored. So, when the value is 0.14, the
  ///   actual IOR is 1.14.
  pub prop_ior   : f32,
  pub prop_alpha : f32
}
