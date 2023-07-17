use std::collections::HashMap;

pub trait MetaData {
    fn author(&self) -> &'static str;
    fn version(&self) -> &'static str;
    fn field_authors(&self) -> HashMap<&'static str, &'static str>;
}