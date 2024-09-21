use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Attributes {
    pub attrs: Vec<String>,
    pub span: Span,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FuncDef {
    pub func_type: VType,
    pub block: Block,
    pub params: Vec<Param>,
    pub span: Span,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProtoDef {
    pub func_type: VType,
    pub params: Vec<Param>,
    pub span: Span,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Param {
    pub ty: VType,
    pub id: String,
    pub span: Span,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Decl {
    Const(ConstDecl),
    Var(VarDecl),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassMember {
    pub ty: VType,
    pub id: String,
    pub span: Span,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassDef {
    pub members: Vec<ClassMember>,
    pub consts: Vec<ConstDecl>,
    pub span: Span,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewClassMember {
    pub id: String,
    pub val: Exp,
    pub span: Span,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewClass {
    pub class: String,
    pub members: Vec<NewClassMember>,
    pub span: Span,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct While {
    pub cond: Exp,
    pub then: Block,
    pub span: Span,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConstDecl {
    pub attr: Option<Attributes>,
    pub id: String,
    pub init: ConstInitVal,
    pub span: Span,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ConstInitVal {
    Exp(ConstExp),
    Function(FuncDef),
    Proto(ProtoDef),
    Class(ClassDef),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VarDecl {
    pub id: String,
    pub ty: VType,
    pub init: InitVal,
    pub span: Span,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitVal {
    pub exp: Exp,
}
