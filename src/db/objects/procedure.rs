use crate::db::producer::DatabaseSpeicifics;

#[derive(Debug, Clone)]
pub struct StoredProcedure<'a, T: DatabaseSpeicifics> {
    pub name: &'a str,
    pub params: Vec<Parameter>,
    pub returns: Vec<Parameter>,
    pub body: String,
    _marker: std::marker::PhantomData<T>,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub data_type: String,
}

impl<'a, T: DatabaseSpeicifics> StoredProcedure<'a, T> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            params: Vec::new(),
            returns: Vec::new(),
            body: String::new(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn add_param(&mut self, param: Parameter) {
        self.params.push(param);
    }

    pub fn add_return(&mut self, param: Parameter) {
        self.returns.push(param);
    }

    pub fn set_body(&mut self, body: &str) {
        self.body = body.to_string();
    }
}
