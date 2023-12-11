#[derive(Debug, Clone)]
pub struct StoredProcedure {
    pub name: String,
    pub params: Vec<Parameter>,
    pub returns: Option<Parameter>,
    pub body: String,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub data_type: String,
    pub list: bool,
}

impl StoredProcedure {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            params: Vec::new(),
            returns: None,
            body: String::new(),
        }
    }

    pub fn add_param(&mut self, param: Parameter) {
        self.params.push(param);
    }

    pub fn add_return(&mut self, param: Parameter) {
        self.returns = Some(param);
    }

    pub fn set_body(&mut self, body: &str) {
        self.body = body.to_string();
    }
}
