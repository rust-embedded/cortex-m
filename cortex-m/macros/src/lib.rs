//! Internal implementation details of `cortex-m`.
//!
//! Do not use this crate directly.

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Item, Meta, parse_macro_input, parse_quote};

#[proc_macro_attribute]
pub fn asm_cfg(attr: TokenStream, item: TokenStream) -> TokenStream {
    let cfg_expr = parse_macro_input!(attr as Meta);
    let wrapped_item = parse_macro_input!(item as Item);

    let new_item = match wrapped_item {
        Item::Fn(f) => asm_cfg_wrap_fn(cfg_expr, f),
        // TODO(wt): we should probably support modules as well
        // Item::Mod(m) => asm_wrapper_wrap_mod(cfg_expr, m),
        _ => unimplemented!(),
    };

    quote! {
        #new_item
    }
    .into()
}

fn asm_cfg_wrap_fn(cfg_expr: Meta, mut f: syn::ItemFn) -> Item {
    let old_block = f.block;
    f.block = parse_quote! {
        {
            #[cfg(#cfg_expr)]
            #old_block

            #[cfg(not(#cfg_expr))]
            unimplemented!()
        }
    };
    parse_quote! {
        #[allow(unused)]
        #f
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_asm_cfg() {
        macrotest::expand("proc_macro_tests/*.rs");
    }
}
