//! # Arrowmax
//!
//! High-performance Arrow data stack: columnar storage, zero-copy streaming, and schema codegen.
//!
//! ## Crates
//!
//! - [`minarrow`] - Apache Arrow-compatible columnar data library with SIMD alignment
//! - [`lightstream`] - Zero-copy Arrow IPC streaming with async support
//! - [`flatarrow`] - Arrow memory layout schema DSL with codegen and wire format
//!
//! ## Feature Flags
//!
//! - `flatarrow` - Enable the flatarrow schema DSL crate

pub use minarrow;
pub use lightstream;

#[cfg(feature = "flatarrow")]
pub use flatarrow;
