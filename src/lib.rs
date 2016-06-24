// Copyright 2016 The Rust-Proof Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private)]
// FIXME: these should not be here!
#![allow(unused_variables)]
#![allow(unused_imports)]

#[macro_use]
extern crate rustc;
extern crate syntax;
extern crate rustc_plugin;

// These can be their own .rs file OR
// a named directory with mod.rs + other files
// see: https://doc.rust-lang.org/book/crates-and-modules.html
// see: 'tests' module (some things need pub that tests doesnt mind priv)
// see: 'reporting' module

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
use syntax::ptr::P;



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
    match meta.node {
        // FIXME: at the moment, panic if there are no arguments to the attribute
        MetaItemKind::List(_, ref args) => {
            // FIXME: arguments should be parsed by the parser module, not in this control module
            expand_args(args);
        },
        _ => {
            panic!("Invalid arguments for #[condition]; did you add a pre and/or post condition?");
        }
    }
    //let () = meta.node;
}



// FIXME: this should be in the parser module!
// Parse the condition arguments
fn expand_args(args: &Vec<P<MetaItem>>) {
    match args.len() {
        1 => {
            println!("Found 1 argument:\n");
            println!("{:?}\n", args[0]);
            let ref arg = args[0].node;
            match *arg {
                MetaItemKind::List(ref arg_name, ref items) => {
                    println!("{:?}\n", arg_name);
                },
                _ => {
                    panic!("This shouldn't happen. you messed up.");
                }
            }
        },
        2 => {
            println!("Found 2 arguments:\n");
            println!("{:?}\n", args[0]);
            println!("{:?}\n", args[1]);
            let ref arg1 = args[0].node;
            let ref arg2 = args[1].node;
        },
        _ => {
            panic!("Too many arguments found for #[condition]; must have pre and/or post conditions");
        }
    }
}



// If the #[condition] is not on a function, error out
fn expand_bad_item(ctx: &mut ExtCtxt, span: Span) {
    ctx.span_err(span, "#[condition] must be placed on a function".into());
}
