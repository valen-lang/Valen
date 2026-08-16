use std::result::Result as StdResult;

pub mod arena_index_map;
pub mod arena_utils;
pub mod code_hierarchy;
pub mod fx;
pub mod profiler;
pub mod range;
pub mod source_code_utils;
pub mod utils;
pub mod vassert;

pub type Result<T, E> = StdResult<T, E>;

#[inline(always)]
pub fn vpass() {}
