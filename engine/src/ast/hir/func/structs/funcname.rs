#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct FuncName(String);

NewtypeDeref! {() pub struct FuncName(String); }
NewtypeDerefMut! {() pub struct FuncName(String); }

impl From<String> for FuncName {
    fn from(val: String) -> Self {
        Self(val)
    }
}

impl From<&str> for FuncName {
    fn from(val: &str) -> Self {
        Self(val.into())
    }
}

impl ToString for FuncName {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
