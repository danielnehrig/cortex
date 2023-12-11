use std::collections::HashMap;

use syn::{visit::Visit, Expr};

type TableName = String;
type TableParamName = String;
type TableParamType = String;
type TableParams = Vec<(TableParamName, TableParamType)>;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct TableExtractor {
    pub(crate) data: HashMap<TableName, TableParams>,
}

impl<'ast> Visit<'ast> for TableExtractor {
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        let m_name = i.method.to_string();
        if let Some(name) = self.find_type_name(i) {
            if m_name == "add_prop" {
                eprintln!("Found type: {:#?}", i);
                // this is the param name
                let param_name = if let Expr::Lit(lit) = i.args.first().unwrap() {
                    if let syn::Lit::Str(lit) = &lit.lit {
                        lit.value()
                    } else {
                        panic!("Expected name to be a string")
                    }
                } else {
                    panic!("Expected name to be a string")
                };
                // this is the param type
                let param_type: Option<String> = if let Expr::Path(lit) = i.args.last().unwrap() {
                    Some(lit.path.segments.last().unwrap().ident.to_string())
                } else {
                    None
                };

                if let Some(params) = self.data.get_mut(&name) {
                    params.push((param_name, param_type.unwrap()));
                } else {
                    self.data
                        .insert(name.clone(), vec![(param_name, param_type.unwrap())]);
                }
            }
        }
        syn::visit::visit_expr_method_call(self, i);
    }
}

impl TableExtractor {
    fn find_type_name(&self, expr: &syn::ExprMethodCall) -> Option<String> {
        let mut current_expr = &*expr.receiver;
        loop {
            match current_expr {
                Expr::Call(call) => {
                    eprintln!("{:#?}", call);
                    if let Expr::Path(path) = &*call.func {
                        if path.path.segments.first().unwrap().ident != "Table" {
                            return None;
                        }
                        if path.path.segments.last().unwrap().ident == "new" {
                            if let Expr::Lit(lit) = &call.args.first().unwrap() {
                                if let syn::Lit::Str(lit) = &lit.lit {
                                    return Some(lit.value());
                                }
                            }
                        }
                    }
                    return None;
                }
                Expr::MethodCall(method_call) => {
                    current_expr = &*method_call.receiver;
                }
                _ => break,
            }
        }
        None
    }
}
