// Rust-only port of Scala's reflective `Collector` (utils/collector.rs), specialized for the I-side
// (instantiating) AST. Scala walks any object graph via `productIterator`; Rust has no reflection, so
// this is a hand-written exhaustive walker over the value-AST: PrototypeI / IdI / INameI (+ all name
// structs) / CoordI / KindIT (+ kind structs) / ITemplataI (+ payloads).
//
// Template-name `template` fields are not separately descended: every node reachable through a
// template name is either a leaf (interface/struct/function template names carry no templatas/ids)
// or is reachable directly from the owning name's own fields (e.g. the override-dispatcher's
// `template.impl_id` and the forwarder's `inner` function name). The expression and
// top-level-definition hierarchies are NOT covered here yet — add `visit_*` + `NodeRefI` variants
// for them when a function/HinputsI-rooted `Collector` use is reached.
//
// `all(root, pred)` mirrors Scala's `Collector.all`: walk the whole tree, push every `pred` match.
// `only(root, pred)` mirrors `Collector.only`: exactly one match or panic. Predicates narrow the wide
// `NodeRefI` (e.g. `NodeRefI::Templata(ITemplataI::Coord(_))`) the same way Rust matches everywhere.
/* Guardian: disable-all */

use crate::instantiating::ast::names::*;
use crate::instantiating::ast::types::*;
use crate::instantiating::ast::templata::*;
use crate::instantiating::ast::ast::PrototypeI;
use crate::typing::names::names::IdT;
use indexmap::IndexMap;

/// A reference to a node yielded to the collector predicate. Wide (predicate narrows within).
#[derive(Copy, Clone)]
pub enum NodeRefI<'s, 'i, R> {
    Prototype(&'i PrototypeI<'s, 'i, R>),
    Id(IdI<'s, 'i, R>),
    Name(INameI<'s, 'i, R>),
    Coord(CoordI<'s, 'i, R>),
    Kind(KindIT<'s, 'i, R>),
    Templata(ITemplataI<'s, 'i, R>),
}

fn collect_if<'s, 'i, R, T, F>(pred: &F, out: &mut Vec<T>, node: NodeRefI<'s, 'i, R>)
where
    F: Fn(NodeRefI<'s, 'i, R>) -> Option<T>,
{
    if let Some(t) = pred(node) {
        out.push(t);
    }
}

// ── Public API (mirrors Scala Collector.all / Collector.only) ────────────────────────────────────

pub fn all_in_prototype<'s, 'i, R, T, F>(root: &'i PrototypeI<'s, 'i, R>, pred: &F) -> Vec<T>
where F: Fn(NodeRefI<'s, 'i, R>) -> Option<T>, 's: 'i, R: Copy {
    let mut out = Vec::new();
    visit_prototype(pred, &mut out, root);
    out
}

pub fn all_in_id<'s, 'i, R, T, F>(root: IdI<'s, 'i, R>, pred: &F) -> Vec<T>
where F: Fn(NodeRefI<'s, 'i, R>) -> Option<T>, 's: 'i, R: Copy {
    let mut out = Vec::new();
    visit_id(pred, &mut out, root);
    out
}

pub fn all_in_coord<'s, 'i, R, T, F>(root: CoordI<'s, 'i, R>, pred: &F) -> Vec<T>
where F: Fn(NodeRefI<'s, 'i, R>) -> Option<T>, 's: 'i, R: Copy {
    let mut out = Vec::new();
    visit_coord(pred, &mut out, root);
    out
}

pub fn all_in_kind<'s, 'i, R, T, F>(root: KindIT<'s, 'i, R>, pred: &F) -> Vec<T>
where F: Fn(NodeRefI<'s, 'i, R>) -> Option<T>, 's: 'i, R: Copy {
    let mut out = Vec::new();
    visit_kind(pred, &mut out, root);
    out
}

pub fn all_in_templata<'s, 'i, R, T, F>(root: ITemplataI<'s, 'i, R>, pred: &F) -> Vec<T>
where F: Fn(NodeRefI<'s, 'i, R>) -> Option<T>, 's: 'i, R: Copy {
    let mut out = Vec::new();
    visit_templata(pred, &mut out, root);
    out
}

/// Scala `Collector.all(substitutions.toVector, pred)` — walks the I-side `ITemplataI` values of a
/// typing-to-instantiating substitutions map. Keys are typing-side `IdT` placeholder ids; the I-side
/// collector cannot walk them, but an I-side templata (e.g. `RegionTemplataI`) cannot structurally
/// appear inside a typing-side `IdT` anyway — so walking values is complete w.r.t. any I-side-typed
/// predicate.
pub fn all_in_substitutions<'s, 't, 'i, R, T, F>(
    subs: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, R>>,
    pred: &F,
) -> Vec<T>
where F: Fn(NodeRefI<'s, 'i, R>) -> Option<T>, 's: 'i, R: Copy {
    let mut out = Vec::new();
    for (_id, templata) in subs.iter() {
        visit_templata(pred, &mut out, *templata);
    }
    out
}

/// Scala `Collector.only` over a prototype root — exactly one match or panic.
pub fn only_in_prototype<'s, 'i, R, T, F>(root: &'i PrototypeI<'s, 'i, R>, pred: &F) -> T
where F: Fn(NodeRefI<'s, 'i, R>) -> Option<T>, 's: 'i, R: Copy {
    let mut matches = all_in_prototype(root, pred);
    assert_eq!(matches.len(), 1, "Collector::only expected exactly one match");
    matches.remove(0)
}

// ── Value-AST walkers ────────────────────────────────────────────────────────────────────────────

fn visit_prototype<'s, 'i, R, T, F>(pred: &F, out: &mut Vec<T>, p: &'i PrototypeI<'s, 'i, R>)
where F: Fn(NodeRefI<'s, 'i, R>) -> Option<T>, 's: 'i, R: Copy {
    collect_if(pred, out, NodeRefI::Prototype(p));
    visit_id(pred, out, p.id);
    visit_coord(pred, out, p.return_type);
}

fn visit_id<'s, 'i, R, T, F>(pred: &F, out: &mut Vec<T>, id: IdI<'s, 'i, R>)
where F: Fn(NodeRefI<'s, 'i, R>) -> Option<T>, 's: 'i, R: Copy {
    collect_if(pred, out, NodeRefI::Id(id));
    for step in id.init_steps {
        visit_name(pred, out, *step);
    }
    visit_name(pred, out, id.local_name);
}

fn visit_coord<'s, 'i, R, T, F>(pred: &F, out: &mut Vec<T>, c: CoordI<'s, 'i, R>)
where F: Fn(NodeRefI<'s, 'i, R>) -> Option<T>, 's: 'i, R: Copy {
    collect_if(pred, out, NodeRefI::Coord(c));
    visit_kind(pred, out, c.kind);
}

fn visit_kind<'s, 'i, R, T, F>(pred: &F, out: &mut Vec<T>, k: KindIT<'s, 'i, R>)
where F: Fn(NodeRefI<'s, 'i, R>) -> Option<T>, 's: 'i, R: Copy {
    collect_if(pred, out, NodeRefI::Kind(k));
    match k {
        KindIT::StaticSizedArrayIT(a) => visit_id(pred, out, a.name),
        KindIT::RuntimeSizedArrayIT(a) => visit_id(pred, out, a.name),
        KindIT::StructIT(s) => visit_id(pred, out, s.id),
        KindIT::InterfaceIT(i) => visit_id(pred, out, i.id),
        _ => {} // primitives (Never/Void/Int/Bool/Str/Float): leaves
    }
}

fn visit_citizen_it<'s, 'i, R, T, F>(pred: &F, out: &mut Vec<T>, c: ICitizenIT<'s, 'i, R>)
where F: Fn(NodeRefI<'s, 'i, R>) -> Option<T>, 's: 'i, R: Copy {
    match c {
        ICitizenIT::StructIT(s) => visit_id(pred, out, s.id),
        ICitizenIT::InterfaceIT(i) => visit_id(pred, out, i.id),
    }
}

// ── Names (all 75 INameI variants; leaf names fall to `_ => {}`) ──────────────────────────────────

fn visit_name<'s, 'i, R, T, F>(pred: &F, out: &mut Vec<T>, n: INameI<'s, 'i, R>)
where F: Fn(NodeRefI<'s, 'i, R>) -> Option<T>, 's: 'i, R: Copy {
    collect_if(pred, out, NodeRefI::Name(n));
    match n {
        // Function-ish names
        INameI::ExternFunction(x) => {
            for t in x.template_args { visit_templata(pred, out, *t); }
            for c in x.parameters { visit_coord(pred, out, *c); }
        }
        INameI::FunctionNameIX(x) => {
            for t in x.template_args { visit_templata(pred, out, *t); }
            for c in x.parameters { visit_coord(pred, out, *c); }
        }
        INameI::ForwarderFunction(x) => {
            visit_name(pred, out, INameI::from(x.inner));
        }
        INameI::FunctionBound(x) => {
            for t in x.template_args { visit_templata(pred, out, *t); }
            for c in x.parameters { visit_coord(pred, out, *c); }
        }
        INameI::ReachableFunction(x) => {
            for t in x.template_args { visit_templata(pred, out, *t); }
            for c in x.parameters { visit_coord(pred, out, *c); }
        }
        INameI::LambdaCallFunction(x) => {
            for t in x.template_args { visit_templata(pred, out, *t); }
            for c in x.parameters { visit_coord(pred, out, *c); }
        }
        INameI::OverrideDispatcher(x) => {
            visit_id(pred, out, x.template.impl_id);
            for t in x.template_args { visit_templata(pred, out, *t); }
            for c in x.parameters { visit_coord(pred, out, *c); }
        }
        INameI::OverrideDispatcherCase(x) => {
            for t in x.independent_impl_template_args { visit_templata(pred, out, *t); }
        }
        INameI::CaseFunctionFromImpl(x) => {
            for t in x.template_args { visit_templata(pred, out, *t); }
            for c in x.parameters { visit_coord(pred, out, *c); }
        }
        // Citizen / struct / interface names
        INameI::StructName(x) => {
            for t in x.template_args { visit_templata(pred, out, *t); }
        }
        INameI::InterfaceName(x) => {
            for t in x.template_args { visit_templata(pred, out, *t); }
        }
        // Impl names
        INameI::Impl(x) => {
            for t in x.template_args { visit_templata(pred, out, *t); }
            visit_citizen_it(pred, out, x.sub_citizen);
        }
        INameI::ImplBound(x) => {
            for t in x.template_args { visit_templata(pred, out, *t); }
        }
        INameI::AnonymousSubstructImpl(x) => {
            for t in x.template_args { visit_templata(pred, out, *t); }
            visit_citizen_it(pred, out, x.sub_citizen);
        }
        // Anonymous substruct names
        INameI::AnonymousSubstruct(x) => {
            for t in x.template_args { visit_templata(pred, out, *t); }
        }
        INameI::AnonymousSubstructConstructor(x) => {
            for t in x.template_args { visit_templata(pred, out, *t); }
            for c in x.parameters { visit_coord(pred, out, *c); }
        }
        // Array names
        INameI::RawArray(x) => {
            visit_templata(pred, out, ITemplataI::Coord(x.element_type));
        }
        INameI::StaticSizedArray(x) => {
            visit_templata(pred, out, ITemplataI::Coord(x.arr.element_type));
        }
        INameI::RuntimeSizedArray(x) => {
            visit_templata(pred, out, ITemplataI::Coord(x.arr.element_type));
        }
        // Everything else is a leaf name (var/local/path/package/region/template-name/etc.).
        _ => {}
    }
}

// ── Templatas (20 ITemplataI payloads; leaf payloads fall to `_ => {}`) ───────────────────────────

fn visit_templata<'s, 'i, R, T, F>(pred: &F, out: &mut Vec<T>, t: ITemplataI<'s, 'i, R>)
where F: Fn(NodeRefI<'s, 'i, R>) -> Option<T>, 's: 'i, R: Copy {
    collect_if(pred, out, NodeRefI::Templata(t));
    match t {
        ITemplataI::Coord(x) => visit_coord(pred, out, x.coord),
        ITemplataI::Kind(x) => visit_kind(pred, out, x.kind),
        ITemplataI::Function(x) => visit_id(pred, out, x.env_id),
        ITemplataI::StructDefinition(x) => visit_id(pred, out, x.env_id),
        ITemplataI::InterfaceDefinition(x) => visit_id(pred, out, x.env_id),
        ITemplataI::ImplDefinition(x) => visit_id(pred, out, x.env_id),
        ITemplataI::Prototype(x) => visit_prototype(pred, out, x.prototype),
        ITemplataI::Isa(x) => {
            visit_id(pred, out, x.impl_name);
            visit_kind(pred, out, x.sub_kind);
            visit_kind(pred, out, x.super_kind);
        }
        ITemplataI::CoordList(x) => {
            for c in x.coords { visit_coord(pred, out, *c); }
        }
        // leaves: Ownership / Variability / Mutability / Location / Boolean / Integer / String /
        // Region / RuntimeSizedArrayTemplate / StaticSizedArrayTemplate. ExternFunction's header is a
        // cI-region definition node not bridgeable into the caller's generic R — not descended here.
        _ => {}
    }
}
