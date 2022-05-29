use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Ident, Token, Expr, parse::Parse, parse_macro_input, punctuated::Punctuated};

use crate::FEATURES;

pub fn make_system_vector(items: TokenStream) -> TokenStream {
	let content = parse_macro_input!(items as Block);

	let mut attributes = Vec::new();
	let mut function_names = Vec::new();
	let mut function_values = Vec::new();

	for pair in &content.0 {
		if pair.is_active() {
			attributes.push(pair.attrinutes());
			function_names.push(pair.name());
			function_values.push(pair.value());
		}
	}

	let count = function_names.len();
	let enum_counter = 0isize..;

	quote! {
		const SYSTEM_VECTOR: [SysFunc; #count] = [
			#(#function_values),*
		];

		/// All available system calls. Every call has 3
		/// `isize` parameters(`a`, `b` and `c`) and returns 1 `isize` value.
		/// You may have to convert different types for some calls.
		/// All calls can return [`Error`]
		pub enum SysCall {
			#(#(#attributes)* #function_names = #enum_counter),*
		}
	}.into()
}

struct Block(Punctuated<Pair, Token![,]>);

impl Parse for Block {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		Ok(Block(input.parse_terminated(Pair::parse)?))
	}
}

struct Pair {
	attrs: Vec<Attribute>,
	name: Ident,
	_arrow: Token![=>],
	value: Expr
}

impl Pair {
	fn is_active(&self) -> bool {
		(&self.attrs).into_iter().fold(true, |acc, attr| match attr.path.get_ident() {
				Some(ident) => if ident == "cfg" {
					let attr = attr.tokens.to_string();
					let attr = &attr[1..attr.len() - 1];
					acc && cfg_expr::Expression::parse(attr).unwrap().eval(|pred| match pred {
						cfg_expr::Predicate::Feature(feature) => FEATURES.contains(feature),
						_ => panic!("only feature cfg is supported")
					})
				} else { acc },
				None => acc
			})
	}

	fn name(&self) -> &Ident {
		&self.name
	}

	fn value(&self) -> &Expr {
		&self.value
	}

	fn attrinutes(&self) -> &Vec<Attribute> {
		&self.attrs
	}
}

impl Parse for Pair {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		Ok(Pair {
			attrs: input.call(Attribute::parse_outer)?,
			name: input.parse()?,
			_arrow: input.parse()?,
			value: input.parse()?
		})
	}
}
