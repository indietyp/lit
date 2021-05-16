#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct ModuleName(Vec<String>);

impl ModuleName {
    pub fn main() -> Self {
        ModuleName(vec!["fs".to_string(), "main".to_string()])
    }
}

NewtypeDeref! {() pub struct ModuleName(Vec<String>); }
NewtypeDerefMut! {() pub struct ModuleName(Vec<String>); }

impl From<Vec<String>> for ModuleName {
    fn from(val: Vec<String>) -> Self {
        Self(val)
    }
}

impl From<Vec<&str>> for ModuleName {
    fn from(val: Vec<&str>) -> Self {
        Self(val.iter().map(|v| v.to_string()).collect())
    }
}

impl ToString for ModuleName {
    fn to_string(&self) -> String {
        self.0.join("::")
    }
}
