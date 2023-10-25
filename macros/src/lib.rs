use quote::{quote, ToTokens};
use syn::ItemFn;
#[proc_macro_derive(create)]
pub fn create_table(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_table_macro(&ast)
}

#[proc_macro_attribute]
pub fn create_stored_proc(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ast: ItemFn = syn::parse(input).unwrap();
    let stmts = (*ast.block).stmts;
    for x in stmts {
        match x {
            syn::Stmt::Local(local) => {
                if let Some(data) = local.init {
                    match *data.expr {
                        syn::Expr::Call(_) => todo!("call"),
                        syn::Expr::Let(_) => todo!("let"),
                        syn::Expr::Lit(_) => todo!("lit"),
                        syn::Expr::MethodCall(m) => {
                            eprintln!("{:#?}", (*m.receiver).into_token_stream());
                        }
                        syn::Expr::Path(_) => todo!("path"),
                        _ => todo!("Not implemented skip"),
                    }
                }
            }
            syn::Stmt::Item(_) => todo!(),
            syn::Stmt::Expr(_, _) => todo!(),
            syn::Stmt::Macro(_) => todo!(),
        }
    }
    // eprintln!("{:#?}", input2);
    todo!()
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
