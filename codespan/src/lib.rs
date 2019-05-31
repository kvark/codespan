//! Utilities for working with source code and printing nicely formatted
//! diagnostic information like warnings and errors.
//!
//! # Optional Features
//!
//! Extra functionality is accessible by enabling feature flags. The features
//! currently available are:
//!
//! - **serialization** - Adds `Serialize` and `Deserialize` implementations
//!   for use with `serde`
//! - **memory_usage** - Adds `HeapSizeOf` implementations for use with the
//!   `heapsize` crate

mod codemap;
mod file;
mod index;
mod location;
mod span;

pub use crate::codemap::CodeMap;
pub use crate::file::File;
pub use crate::file::{ByteIndexError, LineIndexError, LocationError, SpanError};
pub use crate::index::{ByteIndex, ByteOffset};
pub use crate::index::{ColumnIndex, ColumnNumber, ColumnOffset};
pub use crate::index::{Index, Offset};
pub use crate::index::{LineIndex, LineNumber, LineOffset};
pub use crate::index::{RawIndex, RawOffset};
pub use crate::location::Location;
pub use crate::span::{ByteSpan, Span};
