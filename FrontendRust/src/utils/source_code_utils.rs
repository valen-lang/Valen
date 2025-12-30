use std::path::Path;

// Mirrors SourceCodeUtils.scala:humanizePos
pub fn humanize_pos(file_path: &Path, source: &str, pos: i32) -> String {
    let mut line = 0;
    let mut line_begin = 0;
    let mut i = 0;
    
    while i < pos as usize && i < source.len() {
        if source.chars().nth(i) == Some('\n') {
            line_begin = i + 1;
            line += 1;
        }
        i += 1;
    }
    
    format!("{}:{}:{}", file_path.display(), line + 1, i - line_begin + 1)
}

// Mirrors SourceCodeUtils.scala:nextThingAndRestOfLine
pub fn next_thing_and_rest_of_line(source: &str, pos: usize) -> String {
    let remaining = &source[pos..];
    remaining
        .split('\n')
        .next()
        .unwrap_or("")
        .trim()
        .to_string()
}
