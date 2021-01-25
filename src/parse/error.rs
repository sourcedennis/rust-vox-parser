//! Custom errors for the nom parser, which are specific for the `.vox` parser.


/// A custom nom error kind, specific to the `.vox` parser.
/// 
/// It wraps [`nom`]'s [`ErrorKind`](nom::error::ErrorKind) with the `Nom`
/// constructor.
#[derive(Debug, Clone, PartialEq)]
pub enum VoxErrorKind {
  /// Generic parser error as specified by [`nom`].
  Nom( nom::error::ErrorKind ),
  /// Only file version 150 is supported.
  FileVersionUnknown( u32 ),
  /// When the main chunk has payload (while it should only have children)
  InvalidMainChunk,
  /// When the MATT id is outside the palette (i.e., i<0 || i > 255)
  InvalidMattId( u32 ),
  /// When the type is not [0=diffuse, 1=metal, 2=glass, 3=emissive] are defined.
  /// Or when it's weight-attribute is out-of-range for the type.
  InvalidMattType,
  /// When a MATT material property value is out of it's specified range.
  InvalidMattProperty,
  /// When a [u8] cannot be parsed to a string.
  InvalidUTF8String,
  /// When the _hidden field for nTRN is neither 0 or 1
  InvalidTRNHidden,
  /// When the reserved field for nTRN is not -1
  InvalidTRNReserved( i32 ),
  /// When the number of frames in the nTRN is not 1
  InvalidTRNFrames( i32 ),
  InvalidTRNProperty,
  /// When the number of nSHP models is not 1
  InvalidSHPModelCount( u32 ),
  /// When the MATL id is outside the palette (i.e., i<0 || i > 255)
  InvalidMatlId( i32 ),
  /// When the MATL type is neither of: _diffuse, _metal, _glass, _emit
  InvalidMatlType,
  InvalidMatlProperty,
  /// When the layer reserved id is not -1
  InvalidLayrReserved( i32 ),
  InvalidLayrId,
  InvalidLayrProperty,
  UnknownChunk( [u8; 4] ),

  // # Scene conversion errors
  // These are not actually parse errors and might be moved later.

  /// When SIZE and XYZI do not alternate during scene construction
  NonAlternatingModel,
  /// The scene graph references non-existant nodes or models
  InvalidScene
}

/// A custom nom error for the `.vox` parser.
#[derive(Debug, Clone, PartialEq)]
pub struct VoxError< I > {
  pub input : I,
  pub code  : VoxErrorKind
}

impl< I > VoxError< I > {
  pub fn new( input: I, code: VoxErrorKind ) -> VoxError< I > {
    VoxError { input, code }
  }
}

impl< I > nom::error::ParseError< I > for VoxError< I > {
  fn from_error_kind( input: I, kind: nom::error::ErrorKind ) -> Self {
    Self::new( input, VoxErrorKind::Nom( kind ) )
  }

  fn append( _: I, _: nom::error::ErrorKind, other: Self ) -> Self {
    other
  }
}
