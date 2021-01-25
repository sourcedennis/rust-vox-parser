/// The default palette
/// 
/// This palette is taken from the specification:
/// - [MagicaVoxel-file-format-vox.txt](https://github.com/ephtracy/voxel-model/blob/master/MagicaVoxel-file-format-vox.txt)
///
/// This palette is used when the file contains no palette. This palette does
/// _not_ store palette index 0, as it is unused. So, array index 0 represents
/// index 1 in the actual palette.
/// 
/// The elements are ordered as RGBA.
pub static DEFAULT_PALETTE: [(u8,u8,u8,u8); 255] =
  [ (0xff, 0xff, 0xff, 0xff), (0xff, 0xff, 0xcc, 0xff), (0xff, 0xff, 0x99, 0xff),
    (0xff, 0xff, 0x66, 0xff), (0xff, 0xff, 0x33, 0xff), (0xff, 0xff, 0x00, 0xff),
    (0xff, 0xcc, 0xff, 0xff), (0xff, 0xcc, 0xcc, 0xff), (0xff, 0xcc, 0x99, 0xff),
    (0xff, 0xcc, 0x66, 0xff), (0xff, 0xcc, 0x33, 0xff), (0xff, 0xcc, 0x00, 0xff),
    (0xff, 0x99, 0xff, 0xff), (0xff, 0x99, 0xcc, 0xff), (0xff, 0x99, 0x99, 0xff),
    (0xff, 0x99, 0x66, 0xff), (0xff, 0x99, 0x33, 0xff), (0xff, 0x99, 0x00, 0xff),
    (0xff, 0x66, 0xff, 0xff), (0xff, 0x66, 0xcc, 0xff), (0xff, 0x66, 0x99, 0xff),
    (0xff, 0x66, 0x66, 0xff), (0xff, 0x66, 0x33, 0xff), (0xff, 0x66, 0x00, 0xff),
    (0xff, 0x33, 0xff, 0xff), (0xff, 0x33, 0xcc, 0xff), (0xff, 0x33, 0x99, 0xff),
    (0xff, 0x33, 0x66, 0xff), (0xff, 0x33, 0x33, 0xff), (0xff, 0x33, 0x00, 0xff),
    (0xff, 0x00, 0xff, 0xff), (0xff, 0x00, 0xcc, 0xff), (0xff, 0x00, 0x99, 0xff),
    (0xff, 0x00, 0x66, 0xff), (0xff, 0x00, 0x33, 0xff), (0xff, 0x00, 0x00, 0xff),
    (0xcc, 0xff, 0xff, 0xff), (0xcc, 0xff, 0xcc, 0xff), (0xcc, 0xff, 0x99, 0xff),
    (0xcc, 0xff, 0x66, 0xff), (0xcc, 0xff, 0x33, 0xff), (0xcc, 0xff, 0x00, 0xff),
    (0xcc, 0xcc, 0xff, 0xff), (0xcc, 0xcc, 0xcc, 0xff), (0xcc, 0xcc, 0x99, 0xff),
    (0xcc, 0xcc, 0x66, 0xff), (0xcc, 0xcc, 0x33, 0xff), (0xcc, 0xcc, 0x00, 0xff),
    (0xcc, 0x99, 0xff, 0xff), (0xcc, 0x99, 0xcc, 0xff), (0xcc, 0x99, 0x99, 0xff),
    (0xcc, 0x99, 0x66, 0xff), (0xcc, 0x99, 0x33, 0xff), (0xcc, 0x99, 0x00, 0xff),
    (0xcc, 0x66, 0xff, 0xff), (0xcc, 0x66, 0xcc, 0xff), (0xcc, 0x66, 0x99, 0xff),
    (0xcc, 0x66, 0x66, 0xff), (0xcc, 0x66, 0x33, 0xff), (0xcc, 0x66, 0x00, 0xff),
    (0xcc, 0x33, 0xff, 0xff), (0xcc, 0x33, 0xcc, 0xff), (0xcc, 0x33, 0x99, 0xff),
    (0xcc, 0x33, 0x66, 0xff), (0xcc, 0x33, 0x33, 0xff), (0xcc, 0x33, 0x00, 0xff),
    (0xcc, 0x00, 0xff, 0xff), (0xcc, 0x00, 0xcc, 0xff), (0xcc, 0x00, 0x99, 0xff),
    (0xcc, 0x00, 0x66, 0xff), (0xcc, 0x00, 0x33, 0xff), (0xcc, 0x00, 0x00, 0xff),
    (0x99, 0xff, 0xff, 0xff), (0x99, 0xff, 0xcc, 0xff), (0x99, 0xff, 0x99, 0xff),
    (0x99, 0xff, 0x66, 0xff), (0x99, 0xff, 0x33, 0xff), (0x99, 0xff, 0x00, 0xff),
    (0x99, 0xcc, 0xff, 0xff), (0x99, 0xcc, 0xcc, 0xff), (0x99, 0xcc, 0x99, 0xff),
    (0x99, 0xcc, 0x66, 0xff), (0x99, 0xcc, 0x33, 0xff), (0x99, 0xcc, 0x00, 0xff),
    (0x99, 0x99, 0xff, 0xff), (0x99, 0x99, 0xcc, 0xff), (0x99, 0x99, 0x99, 0xff),
    (0x99, 0x99, 0x66, 0xff), (0x99, 0x99, 0x33, 0xff), (0x99, 0x99, 0x00, 0xff),
    (0x99, 0x66, 0xff, 0xff), (0x99, 0x66, 0xcc, 0xff), (0x99, 0x66, 0x99, 0xff),
    (0x99, 0x66, 0x66, 0xff), (0x99, 0x66, 0x33, 0xff), (0x99, 0x66, 0x00, 0xff),
    (0x99, 0x33, 0xff, 0xff), (0x99, 0x33, 0xcc, 0xff), (0x99, 0x33, 0x99, 0xff),
    (0x99, 0x33, 0x66, 0xff), (0x99, 0x33, 0x33, 0xff), (0x99, 0x33, 0x00, 0xff),
    (0x99, 0x00, 0xff, 0xff), (0x99, 0x00, 0xcc, 0xff), (0x99, 0x00, 0x99, 0xff),
    (0x99, 0x00, 0x66, 0xff), (0x99, 0x00, 0x33, 0xff), (0x99, 0x00, 0x00, 0xff),
    (0x66, 0xff, 0xff, 0xff), (0x66, 0xff, 0xcc, 0xff), (0x66, 0xff, 0x99, 0xff),
    (0x66, 0xff, 0x66, 0xff), (0x66, 0xff, 0x33, 0xff), (0x66, 0xff, 0x00, 0xff),
    (0x66, 0xcc, 0xff, 0xff), (0x66, 0xcc, 0xcc, 0xff), (0x66, 0xcc, 0x99, 0xff),
    (0x66, 0xcc, 0x66, 0xff), (0x66, 0xcc, 0x33, 0xff), (0x66, 0xcc, 0x00, 0xff),
    (0x66, 0x99, 0xff, 0xff), (0x66, 0x99, 0xcc, 0xff), (0x66, 0x99, 0x99, 0xff),
    (0x66, 0x99, 0x66, 0xff), (0x66, 0x99, 0x33, 0xff), (0x66, 0x99, 0x00, 0xff),
    (0x66, 0x66, 0xff, 0xff), (0x66, 0x66, 0xcc, 0xff), (0x66, 0x66, 0x99, 0xff),
    (0x66, 0x66, 0x66, 0xff), (0x66, 0x66, 0x33, 0xff), (0x66, 0x66, 0x00, 0xff),
    (0x66, 0x33, 0xff, 0xff), (0x66, 0x33, 0xcc, 0xff), (0x66, 0x33, 0x99, 0xff),
    (0x66, 0x33, 0x66, 0xff), (0x66, 0x33, 0x33, 0xff), (0x66, 0x33, 0x00, 0xff),
    (0x66, 0x00, 0xff, 0xff), (0x66, 0x00, 0xcc, 0xff), (0x66, 0x00, 0x99, 0xff),
    (0x66, 0x00, 0x66, 0xff), (0x66, 0x00, 0x33, 0xff), (0x66, 0x00, 0x00, 0xff),
    (0x33, 0xff, 0xff, 0xff), (0x33, 0xff, 0xcc, 0xff), (0x33, 0xff, 0x99, 0xff),
    (0x33, 0xff, 0x66, 0xff), (0x33, 0xff, 0x33, 0xff), (0x33, 0xff, 0x00, 0xff),
    (0x33, 0xcc, 0xff, 0xff), (0x33, 0xcc, 0xcc, 0xff), (0x33, 0xcc, 0x99, 0xff),
    (0x33, 0xcc, 0x66, 0xff), (0x33, 0xcc, 0x33, 0xff), (0x33, 0xcc, 0x00, 0xff),
    (0x33, 0x99, 0xff, 0xff), (0x33, 0x99, 0xcc, 0xff), (0x33, 0x99, 0x99, 0xff),
    (0x33, 0x99, 0x66, 0xff), (0x33, 0x99, 0x33, 0xff), (0x33, 0x99, 0x00, 0xff),
    (0x33, 0x66, 0xff, 0xff), (0x33, 0x66, 0xcc, 0xff), (0x33, 0x66, 0x99, 0xff),
    (0x33, 0x66, 0x66, 0xff), (0x33, 0x66, 0x33, 0xff), (0x33, 0x66, 0x00, 0xff),
    (0x33, 0x33, 0xff, 0xff), (0x33, 0x33, 0xcc, 0xff), (0x33, 0x33, 0x99, 0xff),
    (0x33, 0x33, 0x66, 0xff), (0x33, 0x33, 0x33, 0xff), (0x33, 0x33, 0x00, 0xff),
    (0x33, 0x00, 0xff, 0xff), (0x33, 0x00, 0xcc, 0xff), (0x33, 0x00, 0x99, 0xff),
    (0x33, 0x00, 0x66, 0xff), (0x33, 0x00, 0x33, 0xff), (0x33, 0x00, 0x00, 0xff),
    (0x00, 0xff, 0xff, 0xff), (0x00, 0xff, 0xcc, 0xff), (0x00, 0xff, 0x99, 0xff),
    (0x00, 0xff, 0x66, 0xff), (0x00, 0xff, 0x33, 0xff), (0x00, 0xff, 0x00, 0xff),
    (0x00, 0xcc, 0xff, 0xff), (0x00, 0xcc, 0xcc, 0xff), (0x00, 0xcc, 0x99, 0xff),
    (0x00, 0xcc, 0x66, 0xff), (0x00, 0xcc, 0x33, 0xff), (0x00, 0xcc, 0x00, 0xff),
    (0x00, 0x99, 0xff, 0xff), (0x00, 0x99, 0xcc, 0xff), (0x00, 0x99, 0x99, 0xff),
    (0x00, 0x99, 0x66, 0xff), (0x00, 0x99, 0x33, 0xff), (0x00, 0x99, 0x00, 0xff),
    (0x00, 0x66, 0xff, 0xff), (0x00, 0x66, 0xcc, 0xff), (0x00, 0x66, 0x99, 0xff),
    (0x00, 0x66, 0x66, 0xff), (0x00, 0x66, 0x33, 0xff), (0x00, 0x66, 0x00, 0xff),
    (0x00, 0x33, 0xff, 0xff), (0x00, 0x33, 0xcc, 0xff), (0x00, 0x33, 0x99, 0xff),
    (0x00, 0x33, 0x66, 0xff), (0x00, 0x33, 0x33, 0xff), (0x00, 0x33, 0x00, 0xff),
    (0x00, 0x00, 0xff, 0xff), (0x00, 0x00, 0xcc, 0xff), (0x00, 0x00, 0x99, 0xff),
    (0x00, 0x00, 0x66, 0xff), (0x00, 0x00, 0x33, 0xff), (0xee, 0x00, 0x00, 0xff),
    (0xdd, 0x00, 0x00, 0xff), (0xbb, 0x00, 0x00, 0xff), (0xaa, 0x00, 0x00, 0xff),
    (0x88, 0x00, 0x00, 0xff), (0x77, 0x00, 0x00, 0xff), (0x55, 0x00, 0x00, 0xff),
    (0x44, 0x00, 0x00, 0xff), (0x22, 0x00, 0x00, 0xff), (0x11, 0x00, 0x00, 0xff),
    (0x00, 0xee, 0x00, 0xff), (0x00, 0xdd, 0x00, 0xff), (0x00, 0xbb, 0x00, 0xff),
    (0x00, 0xaa, 0x00, 0xff), (0x00, 0x88, 0x00, 0xff), (0x00, 0x77, 0x00, 0xff),
    (0x00, 0x55, 0x00, 0xff), (0x00, 0x44, 0x00, 0xff), (0x00, 0x22, 0x00, 0xff),
    (0x00, 0x11, 0x00, 0xff), (0x00, 0x00, 0xee, 0xff), (0x00, 0x00, 0xdd, 0xff),
    (0x00, 0x00, 0xbb, 0xff), (0x00, 0x00, 0xaa, 0xff), (0x00, 0x00, 0x88, 0xff),
    (0x00, 0x00, 0x77, 0xff), (0x00, 0x00, 0x55, 0xff), (0x00, 0x00, 0x44, 0xff),
    (0x00, 0x00, 0x22, 0xff), (0x00, 0x00, 0x11, 0xff), (0xee, 0xee, 0xee, 0xff),
    (0xdd, 0xdd, 0xdd, 0xff), (0xbb, 0xbb, 0xbb, 0xff), (0xaa, 0xaa, 0xaa, 0xff),
    (0x88, 0x88, 0x88, 0xff), (0x77, 0x77, 0x77, 0xff), (0x55, 0x55, 0x55, 0xff),
    (0x44, 0x44, 0x44, 0xff), (0x22, 0x22, 0x22, 0xff), (0x11, 0x11, 0x11, 0xff)
  ];
