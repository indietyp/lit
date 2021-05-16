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
        self.1.chars().into_iter().take(3).collect()
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

impl ToString for FuncQualName {
    fn to_string(&self) -> String {
        vec![self.0.to_string(), self.1.to_string()].join("::")
    }
}
