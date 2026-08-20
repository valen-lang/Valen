// Coordinates the full compilation pipeline

use bumpalo::Bump;
use crate::compile_options::GlobalOptions;
use crate::scout_arena::ScoutArena;
use crate::keywords::Keywords;
use crate::lexing::ast::RangeL;
use crate::lexing::errors::FailedParse;
use crate::parsing::ast::FileP;
use crate::code_source::CodeSource;
use crate::utils::code_hierarchy::FileCoordinateMap;
use crate::utils::code_hierarchy::PackageCoordinate;
use std::sync::Arc;
use crate::parse_arena::ParseArena;
use crate::instantiating::instantiated_compilation::{InstantiatedCompilation, InstantiatorCompilationOptions};
use crate::instantiating::ast::hinputs::HinputsI;
use crate::typing::typing_interner::TypingInterner;
use crate::postparsing::ast::ProgramS;
use crate::postparsing::post_parser::ICompileErrorS;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::hinputs_t::HinputsT;


pub struct FullCompilationOptions {
  pub global_options: GlobalOptions,
  pub debug_out: Arc<dyn Fn(&str) + Send + Sync>,
}


// A thin wrapper over InstantiatedCompilation (typing -> instantiating -> HinputsI). The
// simplifying/hammer stage and its ProgramH ('h) output are gone; HinputsI (in the 'i
// instantiating arena) is the sole backend contract, so there is no longer an 'h lifetime.
pub struct FullCompilation<'s, 'ctx, 't, 'i, 'p>
where
  's: 't,
  's: 'i,
  'p: 'ctx,
{
  pub instantiated_compilation: InstantiatedCompilation<'s, 'ctx, 't, 'i, 'p>,
}


impl<'s, 'ctx, 't, 'i, 'p> FullCompilation<'s, 'ctx, 't, 'i, 'p>
where
  's: 't,
  's: 'i,
  'p: 'ctx,
{
  pub fn new(
    scout_arena: &'ctx ScoutArena<'s>,
    typing_interner: &'ctx TypingInterner<'s, 't>,
    keywords: &'ctx Keywords<'s>,
    parser_keywords: &'ctx Keywords<'p>,
    // VV: crate::
    parse_arena: &'ctx ParseArena<'p>,
    packages_to_build: Vec<&'p PackageCoordinate<'p>>,
    code_source: &'ctx CodeSource<'p>,
    options: FullCompilationOptions,
    instantiating_bump: &'i Bump,
  ) -> Self {
    let instantiator_options = InstantiatorCompilationOptions {
      debug_out: options.debug_out,
    };
    let instantiated_compilation = InstantiatedCompilation::new(
      typing_interner,
      scout_arena,
      keywords,
      parser_keywords,
      parse_arena,
      packages_to_build,
      code_source,
      options.global_options,
      instantiator_options,
      instantiating_bump,
    );
    FullCompilation { instantiated_compilation }
  }
}


impl<'s, 'ctx, 't, 'i, 'p> FullCompilation<'s, 'ctx, 't, 'i, 'p>
where 's: 't, 's: 'i, 'p: 'ctx, 'i: 't,
{
  pub fn get_code_map(&mut self) -> Result<FileCoordinateMap<'p, String>, FailedParse<'p>> {
    self.instantiated_compilation.get_code_map()
  }



  pub fn get_parseds(&mut self) -> Result<FileCoordinateMap<'p, (FileP<'p>, Vec<RangeL>)>, FailedParse<'p>> {
    self.instantiated_compilation.get_parseds()
  }



  pub fn get_vpst_map(&mut self) -> Result<FileCoordinateMap<'p, String>, FailedParse<'p>> {
    self.instantiated_compilation.get_vpst_map()
  }



  pub fn get_scoutput(&mut self) -> Result<&FileCoordinateMap<'s, ProgramS<'s>>, ICompileErrorS<'s>> {
    self.instantiated_compilation.get_scoutput()
  }



  pub fn get_compiler_outputs(&mut self) -> Result<&HinputsT<'s, 't>, ICompileErrorT<'s, 't>> {
    self.instantiated_compilation.get_compiler_outputs()
  }



  pub fn expect_compiler_outputs(&mut self) -> &HinputsT<'s, 't> {
    self.instantiated_compilation.expect_compiler_outputs()
  }



  pub fn get_monouts(&mut self) -> &HinputsI<'s, 'i> {
    self.instantiated_compilation.get_monouts()
  }
}
