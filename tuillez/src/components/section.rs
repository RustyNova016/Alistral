use std::fmt::Display;

pub struct Section(pub Box<dyn Display>);

// impl<T: Display> Display for Section<T> {

// }
