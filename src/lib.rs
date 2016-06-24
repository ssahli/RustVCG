// Copyright 2016 The Rust-Proof Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution
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

#[macro_use]
extern crate rustc;
extern crate syntax;
extern crate rustc_plugin;

pub mod reporting;
pub mod z3_interface;
pub mod weakest_precondition;
pub mod parser;

#[cfg(test)]
mod tests;

use rustc_plugin::Registry;
use syntax::ast::{MetaItem, Item, ItemKind, MetaItemKind};
use syntax::ext::base::{ExtCtxt, Annotatable};
use syntax::ext::base::SyntaxExtension::MultiDecorator;
use syntax::codemap::Span;
use syntax::parse::token::intern;



// Register plugin with compiler
#[plugin_registrar]
pub fn registrar(reg: &mut Registry) {
    reg.register_syntax_extension(intern("condition"), MultiDecorator(Box::new(expand_condition)));
}



// For every #[condition], this function is called
// FIXME: I don't really know what `push: &mut FnMut(Annotatable)` is, but I know its required.
fn expand_condition(ctx: &mut ExtCtxt, span: Span, meta: &MetaItem, item: &Annotatable, push: &mut FnMut(Annotatable)) {
    match item {
        &Annotatable::Item(ref it) => match it.node {
            // If the item is a function
            ItemKind::Fn(..) => {
                expand_condition_fn(meta);
            },
            // Otherwise, it shouldn't have #[condition] on it
            _ => expand_bad_item(ctx, span),
        },
        // If it isn't an item at all, also shouldn't have #[condition] on it
        _ => expand_bad_item(ctx, span),
    }
}



// If the #[condition] is on a function...
fn expand_condition_fn(meta: &MetaItem) {
    // FIXME: both of these are just for debug
    println!("\nThis #[condition] is correctly placed on a function\n");
    println!("{:?}", meta.node);
    match meta.node {
        // FIXME: just a note, Word(s) is because the Word type in MetaItemKind is a
        // Word(InternedString), which must be captured in the match
        MetaItemKind::Word(ref s) => {
            println!("\nIt is a Word enum\n");
        },
        // FIXME: just a note, here List(..) is because we aren't referencing anything within
        // List(), but we still have to acknowledge that there are things inside of List()
        MetaItemKind::List(..) => {
            println!("\nIt is a List enum\n");
        },
        MetaItemKind::NameValue(..) => {
            println!("\nIt is a NameValue enum\n");
        },
    }
    //let () = meta.node;
}



// If the #[condition] is not on a function, error out
fn expand_bad_item(ctx: &mut ExtCtxt, span: Span) {
    ctx.span_err(span, "#[condition] must be placed on a function".into());
}
