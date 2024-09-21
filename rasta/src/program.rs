use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompUnit {
    pub global_items: Vec<GlobalItem>,
    pub span: Span,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GlobalItem {
    ConstDecl(ConstDecl),
    BuiltinFnCall(BuiltinFunctionCall),
    InlineAsm(InlineAsm),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub items: Vec<BlockItem>,
    pub span: Span,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BlockItem {
    Decl(Decl),
    Stmt(Stmt),
}
