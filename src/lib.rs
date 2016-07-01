// The Rust-Proof Project is copyright 2016, Sami Sahli,
// Michael Salter, Matthew Slocum, Vincent Schuster,
// Bradley Rasmussen, Drew Gohman, and Matthew O'Brien.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// These can be their own .rs file OR
// a named directory with mod.rs + other files
// see: https://doc.rust-lang.org/book/crates-and-modules.html
// see: 'tests' module (some things need pub that tests doesnt mind priv)
// see: 'reporting' module
#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private)]
// FIXME: these should not be here!
#![allow(unused_variables)]
#![allow(unused_imports)]

// FIXME: remove below. only for dev tools
#![feature(core_intrinsics)]

#[macro_use]
extern crate rustc;
extern crate syntax;
extern crate rustc_plugin;

pub mod reporting;
pub mod z3_interface;
pub mod weakest_precondition;
pub mod parser;
pub mod dev_tools;
//pub mod data;

#[cfg(test)]
mod tests;

use std::cell::RefCell;

use rustc_plugin::Registry;
use syntax::ast::{MetaItem, Item, ItemKind, MetaItemKind, Block};
use syntax::ext::base::{ExtCtxt, Annotatable, MultiItemDecorator};
use syntax::ext::base::SyntaxExtension::MultiDecorator;
use syntax::codemap::Span;
use syntax::parse::token::intern;
use syntax::ptr::P;

use rustc::mir::transform::{Pass, MirPass, MirMapPass, MirSource, MirPassHook};
use rustc::mir::mir_map::MirMap;
use rustc::mir::repr::{Mir, BasicBlock, BasicBlockData};
use rustc::mir::visit::Visitor;
use rustc::ty::TyCtxt;

#[derive(Debug, Clone)]
pub struct Attr {
    pub node_id: RefCell<u32>,
    pub func_name: RefCell<String>,
    pub func_span: RefCell<Option<Span>>,
    pub func: RefCell<Option<P<Block>>>,
    //pub func_mir: Option<Vec<_>>,
    pub pre_span: RefCell<Option<Span>>,
    pub post_span: RefCell<Option<Span>>,
    pub pre_str: RefCell<String>,
    pub post_str: RefCell<String>,
}

fn control_flow(builder: &Attr, meta: &MetaItem, item: &Annotatable) {
    // NOTE: EXPERIMENT: control flow happens here
    //struct to hold all data pertaining to operations
    //init to 'nulls'
    /*
    let mut builder = Attr {
        node_id: RefCell::new(0),
        func_name: RefCell::new("".to_string()),
        func_span: RefCell::new(None),
        func: RefCell::new(None),
        pre_str: RefCell::new("".to_string()),
        post_str: RefCell::new("".to_string()),
        pre_span: RefCell::new(None),
        post_span: RefCell::new(None),
    };*/
    //get attribute values
    parser::parse_attribute(builder, meta);
    //get function name and span
    parser::parse_function(builder, item);
    //get mir statements
    //parser::parse_mir(&mut builder, data);

    //println!("\nDEBUG Item\n{:#?}\n", item);
    println!("\nDEBUG Builder\n{:#?}\n", builder);
}

// Register plugin with compiler
#[plugin_registrar]
pub fn registrar(reg: &mut Registry) {
    let builder = Box::new(Attr {
        node_id: RefCell::new(0),
        func_name: RefCell::new("".to_string()),
        func_span: RefCell::new(None),
        func: RefCell::new(None),
        pre_str: RefCell::new("".to_string()),
        post_str: RefCell::new("".to_string()),
        pre_span: RefCell::new(None),
        post_span: RefCell::new(None),
    });
    reg.register_syntax_extension(intern("condition"), MultiDecorator(builder));
    reg.register_mir_pass(Box::new(MirVisitor));
}


// For every #[condition], this function is called
// FIXME: I don't really know what `push: &mut FnMut(Annotatable)` is, but I know its required.
/// Checks an attribute for proper placement and starts the control flow of the application
impl MultiItemDecorator for Attr {
    fn expand(&self, ctx: &mut ExtCtxt, span: Span, meta: &MetaItem, item: &Annotatable, push: &mut FnMut(Annotatable)) {
        match item {
            &Annotatable::Item(ref it) => match it.node {
                // If the item is a function
                ItemKind::Fn(..) => {
                    control_flow(&self, meta, item);
                },
                // Otherwise, it shouldn't have #[condition] on it
                _ => expand_bad_item(ctx, span),
            },
            // If it isn't an item at all, also shouldn't have #[condition] on it
            _ => expand_bad_item(ctx, span),
        }
    }
}



// If the #[condition] is not on a function, error out
fn expand_bad_item(ctx: &mut ExtCtxt, span: Span) {
    ctx.span_err(span, "#[condition] must be placed on a function".into());
}



struct MirVisitor;

impl<'tcx> Visitor<'tcx> for MirVisitor {
    /*
    fn visit_basic_block_data(&mut self, bb: BasicBlock, d: &BasicBlockData<'tcx>) {
        //println!("\n{:#?}\n", bb);
        //println!("\n{:#?}\n", d);
    }
    */
    fn visit_mir(&mut self, mir: &Mir<'tcx>) {
        //println!("\n{:#?}\n", );
    }
    
}



impl<'tcx> Pass for MirVisitor {
}
/*
impl<'tcx> MirMapPass<'tcx> for MirVisitor {
    fn run_pass<'a>(&mut self, tcx: TyCtxt<'a, 'tcx, 'tcx>, map: &mut MirMap<'tcx>, hooks: &mut [Box<for<'s> MirPassHook<'s>>]) {
        //GetVisitor.visit_mir(map);
        for (&id, mir) in &mut map.map {
            let def_id = tcx.map.local_def_id(id);
            let _task = tcx.dep_graph.in_task(self.dep_graph(def_id));
            let src = MirSource::from_node(tcx, id);
        }
    }
}
*/

impl<'tcx> MirPass<'tcx> for MirVisitor {
    fn run_pass<'a>(&mut self, tcx: TyCtxt<'a, 'tcx, 'tcx>, src: MirSource, mir: &mut Mir<'tcx>) {
        //MirVisitor.visit_mir(mir);
        let item_id = src.item_id();
        let def_id = tcx.map.local_def_id(item_id);
        let name = tcx.item_path_str(def_id);
        let attrs = tcx.map.attrs(item_id);
        println!("Outer node id: {:#?}", item_id);
        //println!("\tdef id: {:#?}", def_id);
        println!("\tfn name: {:#?}", name);
        println!("\tattrs: {:#?}", attrs);
    }
}

