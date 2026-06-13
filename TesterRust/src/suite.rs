use std::path::PathBuf;
use std::process::Child;

pub struct Subprocess {
    pub child: Child,
    pub command: String,
}
// no main.vale counterpart (Vale's stdlib `Subprocess` exposes a `command` field;
// Rust's Child doesn't, so we wrap to make `build_process.command` work 1:1).

// TODO: replace with concurrency
pub struct TestSuite {
    pub cwd: PathBuf,
    pub verbose: bool,
    pub flares: bool,
    pub valec_path: PathBuf,
    pub common_build_args: Vec<String>,
    pub test_filters: Vec<String>,
    pub max_concurrent_tests: i64,
    pub backend_tests_dir: PathBuf,
    pub stdlib_dir: PathBuf,
    pub test_instances: Vec<TestInstance>,
    pub num_successes: i64,
    pub num_failures: i64,
}
/* vale
struct TestSuite {
  cwd Path;
  verbose bool;
  flares bool;
  valec_path Path;
  common_build_args List<str>;
  test_filters List<str>;
  max_concurrent_tests int;
  backend_tests_dir Path;
  stdlib_dir Path;

  test_instances List<TestInstance>;
  num_successes! int;
  num_failures! int;
}
*/

pub struct TestInstance {
    pub test_name: String,
    pub region: String,
    pub test_build_dir: PathBuf,
    pub process: Subprocess,
    pub runs_args: Vec<Step>,
    pub expect_stdout: Option<String>,
}
/* vale
struct TestInstance {
  test_name str;
  region str;
  test_build_dir Path;
  process Subprocess;
  runs_args List<Step>;
  expect_stdout Opt<str>;
}
*/

pub struct Step {
    pub args: Vec<String>,
    pub expected_return_code: i64,
}
/* vale
struct Step {
  args List<str>;
  expected_return_code int;
}
*/

pub fn matches_filters(name: &str, filters: &[String]) -> bool {
    let mut i = 0;
    while i < filters.len() {
        let filter = &filters[i];
        if !name.contains(filter.as_str()) {
            return false;
        }
        i += 1;
    }
    true
}
/* vale
func matches_filters(name str, filters &List<str>) bool {
  i = 0;
  while (i < filters.len()) {
    filter = filters.get(i);
    if (not name.contains(filter)) {
      return false;
    }
    set i = i + 1;
  }
  return true;
}
*/
