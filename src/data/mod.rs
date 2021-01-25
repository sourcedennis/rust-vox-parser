//! Data structures representing `.vox` file contents.
//! 
//! Two representations are included: [`spec`] and [`custom`]. The former
//! faithfully represents chunks as demanded by the specification. However,
//! working with a vector of raw chunks may be difficult. `custom` contains
//! convenient data structures that represents the scene described by the `.vox`
//! chunks.
//!
//! See [`from_custom`](crate::from_custom) and
//! [`to_custom`](`crate::to_custom`) for the corresponding conversion
//! functions.


pub mod spec;
pub mod custom;
