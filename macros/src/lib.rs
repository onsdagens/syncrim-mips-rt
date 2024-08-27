#![feature(log_syntax)]
#[macro_use]
extern crate quote;
use proc_macro::TokenStream;
use syn::ItemFn;
#[macro_use]
extern crate syn;
#[proc_macro_attribute]
pub fn text(_: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);

    let name = f.sig.ident;
    let stmts = f.block.stmts;
    let unsafety = f.sig.unsafety;
    let ret = f.sig.output;
    quote!(
        #[link_section = ".text"]
        #[no_mangle]
        #unsafety fn #name () #ret {
            #(#stmts)*
        }
    )
    .into()
}
