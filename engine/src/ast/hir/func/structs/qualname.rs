use crate::ast::hir::func::structs::funcname::FuncName;
use crate::ast::hir::func::structs::modname::ModuleName;

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct FuncQualName(ModuleName, FuncName);

impl FuncQualName {
    pub fn module(&self) -> &ModuleName {
        &self.0
    }

    pub fn module_mut(&mut self) -> &mut ModuleName {
        &mut self.0
    }

    pub fn func(&self) -> &FuncName {
        &self.1
    }

    pub fn func_mut(&mut self) -> &mut FuncName {
        &mut self.1
    }

    pub fn func_smol(&self) -> String {
        self.1[..3].to_string()
    }
}

impl From<(ModuleName, FuncName)> for FuncQualName {
    fn from((module, func): (ModuleName, FuncName)) -> Self {
        Self(module, func)
    }
}

impl From<(Vec<String>, String)> for FuncQualName {
    fn from((module, func): (Vec<String>, String)) -> Self {
        Self(module.into(), func.into())
    }
}
