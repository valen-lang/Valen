// From Frontend/HigherTypingPass/src/dev/vale/highertyping/HigherTypingPass.scala
// Coordinates the Higher Typing pass

use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::utils::code_hierarchy::{PackageCoordinate, IPackageResolver};
use crate::postparsing::ScoutCompilation;
use crate::passmanager::GlobalOptions;
use crate::lexing::errors::FailedParse;
use crate::utils::code_hierarchy::FileCoordinateMap;
use crate::parsing::ast::FileP;
use crate::lexing::ast::RangeL;
use std::collections::HashMap;
use std::sync::Arc;

/*
package dev.vale.highertyping

import dev.vale.postparsing.PostParserCompilation
import dev.vale.options.GlobalOptions
import dev.vale.parsing.ast.FileP
import dev.vale.postparsing._
import dev.vale.{Err, FileCoordinateMap, IPackageResolver, Ok, PackageCoordinate, PackageCoordinateMap, Result, vassertSome, vcurious, vfail}
import dev.vale.lexing.{FailedParse, RangeL}
import dev.vale.postparsing.ICompileErrorS

import scala.collection.immutable.{List, Map}
*/

// From HigherTypingPass.scala lines 793-836: HigherTypingCompilation class
pub struct HigherTypingCompilation {
    scout_compilation: ScoutCompilation,
    astrouts_cache: Option<()>, // PackageCoordinateMap[ProgramA] not yet ported
}

impl HigherTypingCompilation {
    // From HigherTypingPass.scala lines 793-799
    pub fn new(
        interner: Arc<Interner>,
        keywords: Arc<Keywords>,
        packages_to_build: Vec<Arc<PackageCoordinate>>,
        package_to_contents_resolver: Arc<dyn IPackageResolver<HashMap<String, String>>>,
        global_options: GlobalOptions,
    ) -> Self {
        let scout_compilation = ScoutCompilation::new(
            interner,
            keywords,
            packages_to_build,
            package_to_contents_resolver,
            global_options,
        );
        
        HigherTypingCompilation {
            scout_compilation,
            astrouts_cache: None,
        }
    }

    // From HigherTypingPass.scala line 802: getCodeMap
    pub fn get_code_map(&mut self) -> Result<FileCoordinateMap<String>, FailedParse> {
        self.scout_compilation.get_code_map()
    }

    // From HigherTypingPass.scala line 803: getParseds
    pub fn get_parseds(&mut self) -> Result<FileCoordinateMap<(FileP, Vec<RangeL>)>, FailedParse> {
        self.scout_compilation.get_parseds()
    }

    // From HigherTypingPass.scala line 804: getVpstMap
    pub fn get_vpst_map(&mut self) -> Result<FileCoordinateMap<String>, FailedParse> {
        self.scout_compilation.get_vpst_map()
    }

    // From HigherTypingPass.scala line 805: getScoutput
    pub fn get_scoutput(&mut self) -> Result<(), String> {
        panic!("HigherTypingCompilation.get_scoutput not yet implemented - see HigherTypingPass.scala line 805")
    }

    // From HigherTypingPass.scala lines 807-820: getAstrouts
    pub fn get_astrouts(&mut self) -> Result<(), String> {
        panic!("HigherTypingCompilation.get_astrouts not yet implemented - see HigherTypingPass.scala lines 807-820")
    }

    // From HigherTypingPass.scala lines 821-835: expectAstrouts
    pub fn expect_astrouts(&mut self) -> () {
        panic!("HigherTypingCompilation.expect_astrouts not yet implemented - see HigherTypingPass.scala lines 821-835")
    }
}

