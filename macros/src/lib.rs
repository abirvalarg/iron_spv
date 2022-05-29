use proc_macro::TokenStream;

const FEATURES: &[&str] = &[
	#[cfg(feature = "gpio")]
	"gpio",
	#[cfg(feature = "heap")]
	"heap",
];

mod sys_vect;

#[proc_macro]
pub fn make_system_vector(items: TokenStream) -> TokenStream {
	sys_vect::make_system_vector(items)
}
