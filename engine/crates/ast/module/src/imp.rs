use either::Either;
use variants::LineNo;

pub struct Wildcard {}
pub struct Alias {
    pub ident: String,
    pub alias: Option<String>,
}

pub struct Stmt {
    pub lno: LineNo,

    pub path: Vec<String>,

    pub imports: Either<Vec<Alias>, Wildcard>,
}

pub struct Imp {
    macr: Vec<Stmt>,
    func: Vec<Stmt>,
}
