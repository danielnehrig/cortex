use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sequence {
    pub name: Rc<str>,
    pub start: Option<i64>,
    pub increment: Option<i64>,
    pub min_value: Option<i64>,
    pub max_value: Option<i64>,
    pub cache: Option<i64>,
    pub cycle: Option<bool>,
}

impl Sequence {
    pub fn new(name: &str) -> Self {
        Self {
            name: Rc::from(name),
            start: Some(1),
            increment: Some(1),
            min_value: Some(1),
            cache: Some(1),
            max_value: Some(9223372036854775807),
            cycle: Some(false),
        }
    }

    pub fn set_start(&mut self, start: i64) -> &mut Self {
        self.start = Some(start);
        self
    }

    pub fn set_increment(&mut self, increment: i64) -> &mut Self {
        self.increment = Some(increment);
        self
    }

    pub fn set_min_value(&mut self, min_value: i64) -> &mut Self {
        self.min_value = Some(min_value);
        self
    }

    pub fn set_max_value(&mut self, max_value: i64) -> &mut Self {
        self.max_value = Some(max_value);
        self
    }

    pub fn set_cycle(&mut self, cycle: bool) -> &mut Self {
        self.cycle = Some(cycle);
        self
    }

    pub fn set_cache(&mut self, cache: i64) -> &mut Self {
        self.cache = Some(cache);
        self
    }
}
