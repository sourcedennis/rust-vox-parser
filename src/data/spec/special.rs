//! Data structures for the special (non-chunk) binary structures.


/// Boolean alias used by [`MatRowCols`]. True iff an entry is `-1`; `1`
/// otherwise.
pub type IsNeg = bool;

/// Representation for the `ROTATION` structure.
/// 
/// Simple row-major rotation/mirror matrix representation for the `ROTATION`
/// structure. The contained boolean signifies that the corresponding value is
/// _negative_. The structure ensures no other (or invalid) matrices can be
/// represented.
/// 
/// This data structure is (arguably) a bit unfortunate to work with. However,
/// it has the advantage that it cannot represent invalid matrices. For
/// practical use, consider the [`MatRowCols::matrix`] method.
///
/// E.g., `TwoThreeOne(false, true, false)` states the values are stored in the
/// 2nd, 3rd, and 1st columns in the three rows, respectively. Which corresponds
/// to the following matrix:
/// ```
/// R =
///  0  1  0   # by Two and false
///  0  0 -1   # by Three and true
///  1  0  0   # by One and false
/// ```
#[derive(Debug, Clone, Copy)]
pub enum MatRowCols {
  /// `[x,0,0] [0,x,0] [0,0,x]`
  OneTwoThree( IsNeg, IsNeg, IsNeg ),
  /// `[x,0,0] [0,0,x] [0,x,0]`
  OneThreeTwo( IsNeg, IsNeg, IsNeg ),
  /// `[0,x,0] [x,0,0] [0,0,x]`
  TwoOneThree( IsNeg, IsNeg, IsNeg ),
  /// `[0,x,0] [0,0,x] [x,0,0]`
  TwoThreeOne( IsNeg, IsNeg, IsNeg ),
  /// `[0,0,x] [x,0,0] [0,x,0]`
  ThreeOneTwo( IsNeg, IsNeg, IsNeg ),
  /// `[0,0,x] [0,x,0] [x,0,0]`
  ThreeTwoOne( IsNeg, IsNeg, IsNeg )
}

impl MatRowCols {
  /// Constructs the identity transformation
  /// 
  /// The identity transformation maps any 3d vector to itself.
  pub fn identity( ) -> MatRowCols {
    MatRowCols::OneTwoThree( false, false, false )
  }

  /// Converts the transformation object into a row-major matrix.
  pub fn matrix( &self ) -> [i32; 9] {
    match *self {
      MatRowCols::OneTwoThree( a, b, c ) => [nb2i(a),0,0, 0,nb2i(b),0, 0,0,nb2i(c)],
      MatRowCols::OneThreeTwo( a, b, c ) => [nb2i(a),0,0, 0,0,nb2i(b), 0,nb2i(c),0],
      MatRowCols::TwoOneThree( a, b, c ) => [0,nb2i(a),0, nb2i(b),0,0, 0,0,nb2i(c)],
      MatRowCols::TwoThreeOne( a, b, c ) => [0,nb2i(a),0, 0,0,nb2i(b), nb2i(c),0,0],
      MatRowCols::ThreeOneTwo( a, b, c ) => [0,0,nb2i(a), nb2i(b),0,0, 0,nb2i(c),0],
      MatRowCols::ThreeTwoOne( a, b, c ) => [0,0,nb2i(a), 0,nb2i(b),0, nb2i(c),0,0],
    }
  }

  /// Applies the transformation to the given vector.
  pub fn apply_to( &self, (x,y,z): (i32,i32,i32) ) -> (i32,i32,i32) {
    let mat = self.matrix( );

    ( mat[0]*x + mat[1]*y + mat[2]*z
    , mat[3]*x + mat[4]*y + mat[5]*z
    , mat[6]*x + mat[7]*y + mat[8]*z
    )
  }

  /// Returns `true` iff the transformation is the identity transformation.
  /// 
  /// The identity transformation maps any 3d vector to itself.
  pub fn is_identity( &self ) -> bool {
    match self {
      MatRowCols::OneTwoThree(a,b,c) => !a && !b && !c,
      _ => false
    }
  }
}

/// Converts a `is_neg` boolean into its corresponding multiplication factor.
fn nb2i( is_neg: bool ) -> i32 {
  if is_neg {
    -1
  } else {
    1
  }
}
