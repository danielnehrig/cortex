use std::collections::HashMap;

use syn::{visit::Visit, Expr};

type FnName = String;
type FnReturn = String;
type FnParamName = String;
type FnParamType = String;
type FnParams = Vec<(FnParamName, FnParamType)>;
type FnData = (FnParams, FnReturn);

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct StoredProcExtractor {
    pub(crate) data: HashMap<FnName, FnData>,
}

impl<'ast> Visit<'ast> for StoredProcExtractor {
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        let m_name = i.method.to_string();
        if let Some(name) = self.find_type_name(i) {
            if m_name == "add_param" {
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

                if let Some((params, _)) = self.data.get_mut(&name) {
                    params.push((param_name, param_type.unwrap()));
                } else {
                    self.data.insert(
                        name.clone(),
                        (vec![(param_name, param_type.unwrap())], "".to_string()),
                    );
                }
            }
            if m_name == "add_return" {
                // get the return type
                if let Expr::Path(path) = &i.args[0] {
                    let return_type = path.path.segments.last().unwrap().ident.to_string();
                    if let Some((_, return_type)) = self.data.get_mut(&name) {
                        *return_type = return_type.clone() + ", " + &return_type;
                    } else {
                        self.data.insert(name, (vec![], return_type));
                    }
                }
            }
        }
        syn::visit::visit_expr_method_call(self, i);
    }
}

impl StoredProcExtractor {
    fn find_type_name(&self, expr: &syn::ExprMethodCall) -> Option<String> {
        let mut current_expr = &*expr.receiver;
        loop {
            match current_expr {
                Expr::Call(call) => {
                    if let Expr::Path(path) = &*call.func {
                        if path.path.segments.first().unwrap().ident != "StoredProcedure" {
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
