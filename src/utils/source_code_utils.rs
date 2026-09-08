use std::path::Path;

use crate::utils::code_hierarchy::PackageCoordinate;
use crate::utils::code_hierarchy::{FileCoordinate, FileCoordinateMap};
use crate::utils::range::{CodeLocationS, RangeS};

// Compute the 1-based (line, column) of byte offset `pos` within `source`. `pos` is clamped to
// the source length (a negative `pos` clamps to the end, preserving the historical behavior of
// the callers that pass raw offsets). Newlines advance the line; the column counts bytes since
// the last line start. This is the single line/column primitive — resolve_line_col and
// humanize_pos_path both route through it.
pub fn line_col_in(source: &str, pos: i32) -> (u32, u32) {
  let end = (pos as usize).min(source.len());
  let bytes = source.as_bytes();
  let mut line: u32 = 0;
  let mut line_begin: usize = 0;
  let mut i = 0;
  while i < end {
    if bytes[i] == b'\n' {
      line_begin = i + 1;
      line += 1;
    }
    i += 1;
  }
  (line + 1, (end - line_begin) as u32 + 1)
}

pub fn humanize_pos_path(humanized_file_path: &str, source: &str, pos: i32) -> String {
  let (line, col) = line_col_in(source, pos);
  format!("{}:{}:{}", humanized_file_path, line, col)
}

pub fn humanize_package<'a>(package_coord: &'a PackageCoordinate<'a>) -> String {
  let mut result = package_coord.module.as_str().to_string();
  for p in package_coord.packages.iter() {
    result.push('.');
    result.push_str(p.as_str());
  }
  result
}

pub fn humanize_file<'a>(coordinate: &FileCoordinate<'a>) -> String {
  format!("{}:{}", humanize_package(coordinate.package_coord), coordinate.filepath.as_str())
}

pub fn humanize_pos_code_map<'a, 'b>(
  code_map: &FileCoordinateMap<'a, String>,
  code_location_s: &CodeLocationS<'b>,
) -> String {
  let file = code_location_s.file;
  if code_location_s.offset < 0 {
    return format!("{}:{}", humanize_file(file), code_location_s.offset);
  }
  let source =
    code_map.get_by_value(file).expect("humanize_pos_code_map: coordinate not found in code map");
  humanize_pos_path(&humanize_file(file), source, code_location_s.offset)
}

pub fn humanize_pos(file_path: &Path, source: &str, pos: i32) -> String {
  humanize_pos_path(&file_path.display().to_string(), source, pos)
}

// Resolve (line, col) for a CodeLocationS against its code map.
// Returns 1-based line + column. For internal/synthetic locations
// (offset < 0 or file not present in the map), returns (1, 1).
pub fn resolve_line_col<'a, 'b>(
  code_map: &FileCoordinateMap<'a, String>,
  code_location_s: &CodeLocationS<'b>,
) -> (u32, u32) {
  if code_location_s.offset < 0 {
    return (1, 1);
  }
  match code_map.get_by_value(code_location_s.file) {
    Some(source) => line_col_in(source, code_location_s.offset),
    None => (1, 1),
  }
}

fn next_thing_and_rest_of_line_code_map<'a>(
  _code_map: &FileCoordinateMap<'a, String>,
  _file: &FileCoordinate<'a>,
  _position: i32,
) -> String {
  panic!("Unimplemented: next_thing_and_rest_of_line");
}

pub fn next_thing_and_rest_of_line(source: &str, pos: usize) -> String {
  let remaining = &source[pos..];
  remaining.split('\n').next().unwrap_or("").trim().to_string()
}

fn line_begin<'a>(
  _code_map: &FileCoordinateMap<'a, String>,
  _code_location_s: &CodeLocationS<'a>,
) -> CodeLocationS<'a> {
  panic!("Unimplemented: line_begin");
}

pub fn line_range_containing<'a, 'b>(
  code_map: &FileCoordinateMap<'a, String>,
  code_location_s: &CodeLocationS<'b>,
) -> RangeS<'b> {
  let file = code_location_s.file;
  let offset = code_location_s.offset;
  if offset < 0 {
    return RangeS::new(CodeLocationS { file, offset: -1 }, CodeLocationS { file, offset: 0 });
  }
  let text = code_map
    .get_by_value(code_location_s.file)
    .expect("line_range_containing: coordinate not found in code map");
  let text_len = text.len() as i32;
  let mut line_begin: i32 = 0;
  while line_begin < text_len {
    let line_end = match text[line_begin as usize..].find('\n') {
      None => text_len,
      Some(i) => line_begin + i as i32,
    };
    if line_begin <= offset && offset <= line_end {
      return RangeS::new(
        CodeLocationS { file: file, offset: line_begin },
        CodeLocationS { file, offset: line_end },
      );
    }
    line_begin = line_end + 1;
  }
  if offset == text_len {
    return RangeS::new(
      CodeLocationS { file: file, offset: line_begin },
      CodeLocationS { file, offset: line_begin },
    );
  }
  panic!("line_range_containing: offset beyond text");
}

pub fn lines_between<'a, 'b>(
  code_map: &FileCoordinateMap<'a, String>,
  begin_code_loc: &CodeLocationS<'b>,
  end_code_loc: &CodeLocationS<'b>,
) -> Vec<RangeS<'b>> {
  assert!(begin_code_loc.file == end_code_loc.file);
  assert!(begin_code_loc.offset <= end_code_loc.offset);

  let file = begin_code_loc.file;
  if file.is_internal() {
    return vec![];
  }
  let range = line_range_containing(code_map, begin_code_loc);
  let mut line_begin = range.begin.offset;
  let mut line_end = range.end.offset;
  let mut result = vec![RangeS::new(
    CodeLocationS { file: file, offset: line_begin },
    CodeLocationS { file: file, offset: line_end },
  )];
  let text = code_map.get_by_value(file).expect("lines_between: coordinate not found in code map");
  let text_len = text.len() as i32;
  while line_begin < end_code_loc.offset && line_begin < text_len {
    line_end = match text[line_begin as usize..].find('\n') {
      None => text_len,
      Some(i) => line_begin + i as i32,
    };
    result.push(RangeS::new(
      CodeLocationS { file: file, offset: line_begin },
      CodeLocationS { file: file, offset: line_end },
    ));
    line_begin = line_end + 1;
  }
  result
}

pub fn line_containing<'a, 'b>(
  code_map: &FileCoordinateMap<'a, String>,
  code_location_s: &CodeLocationS<'b>,
) -> String {
  if code_location_s.file.is_internal() {
    return humanize_file(code_location_s.file);
  }
  let range = line_range_containing(code_map, code_location_s);
  let text = code_map
    .get_by_value(code_location_s.file)
    .expect("line_containing: coordinate not found in code map");
  let begin = range.begin.offset as usize;
  let end = range.end.offset as usize;
  text[begin..end].to_string()
}
