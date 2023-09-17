use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Database<T> {
    pub name: Rc<str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Database<T> {
    pub fn new(name: &str) -> Self {
        Self {
            name: Rc::from(name),
            _marker: std::marker::PhantomData,
        }
    }
}
