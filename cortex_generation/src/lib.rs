use std::{io::Write, path::PathBuf};

use convert_case::{Case, Casing};
use cortex::prelude::{Statement, Step};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

pub struct CortexGenerator {
    path: PathBuf,
}

impl CortexGenerator {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
    // create a rust source file
    pub fn create_file(&self, data: Vec<Step>) -> std::io::Result<()> {
        let structs = Self::generate_structs_from_tables(data);
        let mut file = std::fs::File::create(&self.path)?;
        file.write_all(structs.to_string().as_bytes())?;
        let _ = std::process::Command::new("rustfmt")
            .arg(&self.path)
            .output()
            .expect("failed to execute process");
        Ok(())
    }

    fn generate_structs_from_tables(data: Vec<Step>) -> TokenStream {
        let flatten = Step::flatten(data);
        let stmts = flatten
            .statements
            .into_iter()
            .map(|(s, _)| s)
            .collect::<Vec<_>>();
        // get any enum member as table skip the rest
        let tables = stmts
            .iter()
            .filter_map(|s| match s {
                Statement::Table(t) => Some(t),
                _ => None,
            })
            .collect::<Vec<_>>();
        let build_structs = tables.into_iter().map(|x| {
            let name = Ident::new(
                &(x.name.to_string()).to_case(Case::Pascal),
                Span::call_site(),
            );
            let params = x
                .props
                .iter()
                .map(|p| {
                    if let Some(field_text) = p.field.clone().get_as_text() {
                        let t_type =
                            Ident::new(&p.field_type.clone().to_rust_type(), Span::call_site());
                        let field_text = Ident::new(&field_text, Span::call_site());
                        quote! {
                            #field_text: #t_type
                        }
                    } else {
                        quote!()
                    }
                })
                .collect::<Vec<_>>();
            quote! {
              #[derive(Debug, Clone)]
              struct #name {
                #(#params),*
              }
            }
        });
        quote! {
            #(#build_structs)*
        }
    }

    /// DB Composite Type struct generation
    #[allow(dead_code)]
    fn generate_structs_from_db_type() {
        todo!()
    }

    #[allow(dead_code)]
    fn generate_db_functions() {
        todo!()
    }
}
