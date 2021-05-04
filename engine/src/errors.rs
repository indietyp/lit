use crate::types::LineNo;

#[derive(new)]
pub struct Error {
    lno: LineNo,
    variant: ErrorVariant,
}

#[derive(new)]
pub enum ErrorVariant {
    Message(String),
}
