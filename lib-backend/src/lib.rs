extern crate proc_macro;

use crate::proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn some_attribute(attr: TokenStream, input: TokenStream) -> TokenStream { 
    wasm_bindgen_macro_support::expand(attr.into(), input.into()).unwrap().into()
}
