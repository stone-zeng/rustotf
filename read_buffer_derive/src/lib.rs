extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, Data, DeriveInput, Fields};

#[proc_macro_derive(ReadBuffer)]
pub fn read_buffer_derive(input: TokenStream) -> TokenStream {
    let ast = parse(input).unwrap();
    impl_read_buffer(ast)
}

fn impl_read_buffer(ast: DeriveInput) -> TokenStream {
    let name = ast.ident;
    let fields = match ast.data {
        Data::Struct(data_struct) => data_struct.fields,
        _ => unreachable!(),
    };
    let body = match fields {
        // In our case, only named fields are considered.
        Fields::Named(fields) => {
            let recurse = fields.named.iter().map(|f| {
                let name = &f.ident;
                quote! { #name }
            });
            quote! { Self { #(#recurse: buffer.get(),)* } }
        }
        _ => unreachable!(),
    };
    let gen = quote! {
        impl ReadBuffer for #name {
            fn read(buffer: &mut Buffer) -> Self {
                #body
            }
        }
    };
    gen.into()
}
