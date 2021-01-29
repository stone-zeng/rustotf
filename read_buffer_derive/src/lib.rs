extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, Data, DeriveInput, Fields};

// See https://github.com/dtolnay/syn/blob/master/examples/heapsize/heapsize_derive/src/lib.rs.

#[proc_macro_derive(ReadBuffer)]
pub fn read_buffer_derive(input: TokenStream) -> TokenStream {
    let ast = parse(input).unwrap();
    impl_read_buffer(ast)
}

fn impl_read_buffer(ast: DeriveInput) -> TokenStream {
    let name = ast.ident;
    let fields = match ast.data {
        Data::Struct(data_struct) => data_struct.fields,
        _ => unimplemented!(),
    };
    let body = match fields {
        Fields::Named(fields) => {
            let recurse = fields.named.iter().map(|f| {
                let name = &f.ident;
                quote!(#name)
            });
            quote!(Self { #(#recurse: buffer.get(),)* })
        }
        Fields::Unnamed(fields) => {
            let recurse = fields.unnamed.iter().map(|_| quote!(buffer.get()));
            quote!(Self (#(#recurse),*))
        }
        Fields::Unit => {
            quote!(Self)
        }
    };
    let gen = quote!(
        impl ReadBuffer for #name {
            fn read(buffer: &mut Buffer) -> Self {
                #body
            }
        }
    );
    gen.into()
}
