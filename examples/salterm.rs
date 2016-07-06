#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]

extern crate rustproof;

use rustproof::expression::VariableMappingData;
use rustproof::expression::Predicate;
use rustproof::expression::AndData;

fn main() {
	let x = 7u32;
	let z = foo(x);
	let p = Predicate::And(AndData { p1: Box::new(Predicate::BooleanLiteral(true)), p2: Box::new(Predicate::BooleanLiteral(false)) });
	println!("p: {}", p);
	let q = rustproof::parser::parse_predicate_from_string("true && false".to_string());
	println!("q: {}", q)
}

#[condition(pre="1 > 3", post="true || false")]
fn foo(x: u32) -> u32 {
	let y = 5u32;
	x + y
}