//! Spec-conformant represention of `.vox` chunks
//! 
//! This module contains data structures which represents the contents of the
//! binary `.vox` format. These data structures correspond mostly to the
//! chunks.


mod chunks;
mod default_palette;
mod special;

pub use self::special::{IsNeg, MatRowCols};
pub use self::default_palette::DEFAULT_PALETTE;
pub use self::chunks::{Chunk, RawChunk, Matt, MattType, TransformNode,
  GroupNode, ShapeNode, Matl, MatlType, Layr};
