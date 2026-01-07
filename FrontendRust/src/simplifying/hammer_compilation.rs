// From Frontend/SimplifyingPass/src/dev/vale/simplifying/HammerCompilation.scala
// Coordinates the Hammer (simplifying) pass

use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::utils::code_hierarchy::{PackageCoordinate, IPackageResolver};
use crate::instantiating::InstantiatedCompilation;
use crate::pass_manager::{FullCompilationOptions};
use crate::compile_options::GlobalOptions;
use crate::lexing::errors::FailedParse;
use crate::utils::code_hierarchy::FileCoordinateMap;
use crate::parsing::ast::FileP;
use crate::lexing::ast::RangeL;
use std::collections::HashMap;
use std::sync::Arc;

/*
package dev.vale.simplifying

import dev.vale.highertyping.{ICompileErrorA, ProgramA}
import dev.vale.finalast.ProgramH
import dev.vale.options.GlobalOptions
import dev.vale.parsing.ast.FileP
import dev.vale.postparsing._
import dev.vale.{FileCoordinateMap, IPackageResolver, Interner, Keywords, PackageCoordinate, PackageCoordinateMap, Profiler, Result, vassertSome, vcurious, vimpl}
import dev.vale.highertyping.ICompileErrorA
import dev.vale.instantiating.ast.HinputsI
import dev.vale.lexing.{FailedParse, RangeL}
import dev.vale.instantiating.{InstantiatedCompilation, InstantiatorCompilationOptions}
import dev.vale.postparsing.ICompileErrorS
import dev.vale.typing.{HinputsT, ICompileErrorT}

import scala.collection.immutable.List
*/

// From HammerCompilation.scala lines 18-23: HammerCompilationOptions
pub struct HammerCompilationOptions {
    pub debug_out: Arc<dyn Fn(&str) + Send + Sync>,
    pub global_options: GlobalOptions,
}

// From HammerCompilation.scala lines 25-66: HammerCompilation class
pub struct HammerCompilation {
    instantiated_compilation: InstantiatedCompilation,
    hamuts_cache: Option<()>, // ProgramH not yet ported
    von_hammer_cache: Option<()>, // VonHammer not yet ported
}

impl HammerCompilation {
    // From HammerCompilation.scala lines 25-40
    pub fn new(
        interner: Arc<Interner>,
        keywords: Arc<Keywords>,
        packages_to_build: Vec<Arc<PackageCoordinate>>,
        package_to_contents_resolver: Arc<dyn IPackageResolver<HashMap<String, String>>>,
        options: FullCompilationOptions,
    ) -> Self {
        let hammer_options = HammerCompilationOptions {
            debug_out: options.debug_out.clone(),
            global_options: options.global_options,
        };
        
        let instantiated_compilation = InstantiatedCompilation::new(
            interner,
            keywords,
            packages_to_build,
            package_to_contents_resolver,
            hammer_options,
        );
        
        HammerCompilation {
            instantiated_compilation,
            hamuts_cache: None,
            von_hammer_cache: None,
        }
    }

    // From HammerCompilation.scala line 43: getVonHammer
    pub fn get_von_hammer(&self) -> () {
        panic!("HammerCompilation.get_von_hammer not yet implemented - see HammerCompilation.scala line 43")
    }

    // From HammerCompilation.scala line 45: getCodeMap
    pub fn get_code_map(&mut self) -> Result<FileCoordinateMap<String>, FailedParse> {
        self.instantiated_compilation.get_code_map()
    }

    // From HammerCompilation.scala line 46: getParseds
    pub fn get_parseds(&mut self) -> Result<FileCoordinateMap<(FileP, Vec<RangeL>)>, FailedParse> {
        self.instantiated_compilation.get_parseds()
    }

    // From HammerCompilation.scala line 47: getVpstMap
    pub fn get_vpst_map(&mut self) -> Result<FileCoordinateMap<String>, FailedParse> {
        self.instantiated_compilation.get_vpst_map()
    }

    // From HammerCompilation.scala line 48: getScoutput
    pub fn get_scoutput(&mut self) -> Result<(), String> {
        panic!("HammerCompilation.get_scoutput not yet implemented - see HammerCompilation.scala line 48")
    }

    // From HammerCompilation.scala line 49: getAstrouts
    pub fn get_astrouts(&mut self) -> Result<(), String> {
        panic!("HammerCompilation.get_astrouts not yet implemented - see HammerCompilation.scala line 49")
    }

    // From HammerCompilation.scala line 50: getCompilerOutputs
    pub fn get_compiler_outputs(&mut self) -> Result<(), String> {
        panic!("HammerCompilation.get_compiler_outputs not yet implemented - see HammerCompilation.scala line 50")
    }

    // From HammerCompilation.scala line 51: getMonouts
    pub fn get_monouts(&mut self) -> () {
        panic!("HammerCompilation.get_monouts not yet implemented - see HammerCompilation.scala line 51")
    }

    // From HammerCompilation.scala line 52: expectCompilerOutputs
    pub fn expect_compiler_outputs(&mut self) -> () {
        panic!("HammerCompilation.expect_compiler_outputs not yet implemented - see HammerCompilation.scala line 52")
    }

    // From HammerCompilation.scala lines 54-65: getHamuts
    pub fn get_hamuts(&mut self) -> () {
        panic!("HammerCompilation.get_hamuts not yet implemented - see HammerCompilation.scala lines 54-65")
    }
}

