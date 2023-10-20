use quote::quote;
#[proc_macro_derive(Create)]
pub fn Create(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_table_macro(&ast)
}

fn impl_table_macro(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let name = &ast.ident;
    let columns = match &ast.data {
        syn::Data::Struct(s) => match &s.fields {
            syn::Fields::Named(n) => &n.named,
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };
    let gen = quote! {
        impl From<#name> for Table {
            fn from(item: #name) -> Self {
                Table {
                    name: #name,
                    columns: #columns,
                }
            }
        }
    };
    gen.into()
}
