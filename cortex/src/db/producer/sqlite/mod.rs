use crate::objects::step::Step;

pub struct SQLiteStatementProducer<'a> {
    data: Vec<Step<'a, Self>>,
}
