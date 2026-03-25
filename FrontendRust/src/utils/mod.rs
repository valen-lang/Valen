// From Frontend/Utils/src/dev/vale/CodeHierarchy.scala

pub mod arena_index_map;
pub mod arena_utils;
pub mod code_hierarchy;
pub mod profiler;
pub mod range;
pub mod source_code_utils;

/// Result type matching Scala's Result[T, E]
pub type Result<T, E> = std::result::Result<T, E>;
