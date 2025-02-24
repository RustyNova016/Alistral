use core::fmt::Display;

use macon::Builder;

pub struct SelectEle(String);

impl SelectEle {
    pub fn from<S: Into<String>>(self, from: S) -> FromEle {
        FromEle {
            select: self,
            from: from.into(),
            joins: Vec::new()
        }
    }
}

pub struct FromEle {
    select: SelectEle,

    from: String,

    joins: Vec<String>
}

impl FromEle {
    pub fn inner_join<S: Display>(mut self, from: S, join: S, on: S, equals: S) -> Self {
        self.joins.push(format!("INNER JOIN {join} ON {from}.{on} = {join}.{equals}"));
        self
    }
}




#[derive(Debug)]
pub struct QuerryBuilder {
    select_fields: String,

    from: String,

    joins: Vec<String>,
}
