//! Parser, unparser, and data structures for MagicaVoxel `.vox` file format.
//! 
//! This implementation mostly corresponds to the `.vox` specification listed
//! at:
//! 
//! * [MagicaVoxel-file-format-vox.txt](https://github.com/ephtracy/voxel-model/blob/master/MagicaVoxel-file-format-vox.txt)
//! * [MagicaVoxel-file-format-vox-extension.txt](https://github.com/ephtracy/voxel-model/blob/master/MagicaVoxel-file-format-vox-extension.txt)
//! 
//! Note that the specification is incomplete, as MagicaVoxel produces `.vox`
//! files with unspecified chunks (e.g., `rOBJ`).
//! 
//! This library offers two representations:
//! * [`data::spec`] - Spec-conformant chunks. Use this representation if you
//!   have your own scene, which you construct yourself.
//! * [`data::custom`] - Custom voxel scene, which is much easier to work with.
//! 
//! The parser uses [`nom`] (v6).
//! 
//! # Supported chunks:
//! 
//! * `MAIN`
//! * `PACK`
//! * `SIZE`
//! * `XYZI`
//! * `RGBA`
//! * `MATT`
//! * `nTRN`
//! * `nGRP`
//! * `nSHP`
//! * `MATL`
//! * `LAYR`
//! 
//! # Example: Read and write to file
//! 
//! This example reads a voxel file to the custom scene representation
//! [`VoxScene`](data::custom::VoxScene) with [`parse::file_custom`].
//! 
//! ```
//! if let Ok( content ) = std::fs::read( "input.vox" ) {
//!   if let Ok( scene ) = vox_parser::parse::file_custom( &content ) {
//!     // Actual applications would now use this scene.
//!     // Here, it is written back to another file.
//!     let out = vox_parser::unparse::file_custom( &scene );
//!     std::fs::write( "output.vox", out );
//!     println!( "File written!" );
//!   } else {
//!     println!( "Failed to parse file" );
//!   }
//! } else { // Ignore error
//!   println!( "Failed to read file" );
//! }
//! ```
//! 
//! # Example: Count red voxels
//! 
//! This example demonstrates a larger interaction with a custom scene; It
//! counts the voxels in the scene that have a positive red component.
//! 
//! ```
//! use vox_parser::data::custom::{NodeType, SceneNode, VoxScene};
//! 
//! fn main( ) {
//!   let content = std::fs::read( "input.vox" ).unwrap( );
//!   let scene = vox_parser::parse::file_custom( &content ).unwrap( );
//!   let count = num_reddish_voxels( &scene.graph, &scene );
//!   println!( "Count: {}", count );
//! }
//! 
//! fn num_reddish_voxels( n: &SceneNode, scene: &VoxScene ) -> usize {
//!   match &n.node_type {
//!     NodeType::Group( xs ) =>
//!       // Check in all the child scene nodes
//!       xs.iter( ).map( |child| num_reddish_voxels( child, scene ) ).sum( ),
//!     NodeType::Shape( i ) => {
//!       let model = &scene.models[ *i as usize ];
//!       model.xyzi.iter( )
//!         // Warning: Palette index 0 is not stored, as it does not exist. Subtract one.
//!         .map( |(_,_,_,palette_index)| scene.palette[ (*palette_index - 1) as usize ] )
//!         .filter( |m| m.rgba.0 > 0 ) // Check if the material color has a red component
//!         .count( )
//!     }
//!   }
//! }
//! ```


pub mod data;
pub mod parse;
pub mod unparse;

mod convert;

pub use convert::{to_custom, from_custom};
