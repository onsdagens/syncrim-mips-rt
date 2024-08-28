#![feature(log_syntax)]
#[macro_use]
extern crate quote;
use proc_macro::TokenStream;
use syn::{ItemFn, ItemConst, ItemStatic};
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

#[proc_macro_attribute]
pub fn ktext(_: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);

    let name = f.sig.ident;
    let stmts = f.block.stmts;
    let unsafety = f.sig.unsafety;
    let ret = f.sig.output;
    quote!(
        #[link_section = ".ktext"]
        #[no_mangle]
        #unsafety fn #name () #ret {
            #(#stmts)*
        }
    )
    .into()
}

#[proc_macro_attribute]
pub fn data(_: TokenStream, input: TokenStream) -> TokenStream {
    if input.to_string().contains("const") {
        let i = parse_macro_input!(input as ItemConst);
        let ty = i.ty;
        let vis = i.vis;
        let expr = i.expr;
        let ident = i.ident;
        quote!(
            #[link_section = ".data"]
            #[no_mangle]
            #vis const #ident: #ty = #expr;
        ).into()
    } else {
        let i = parse_macro_input!(input as ItemStatic);
        let ty = i.ty;
        let vis = i.vis;
        let expr = i.expr;
        let ident = i.ident;
        let mutability = i.mutability;
        quote!(
            #[link_section = ".data"]
            #[no_mangle]
            #vis static #mutability #ident: #ty = #expr;
        ).into()
    }
    //let i = parse_macro_input!(input as ItemConst);
}
