use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum Stmt {
    Assign(Assign),
    Return(Return),
    Block(Block),
    Exp(Option<Exp>),
    If(If),
    InlineAsm(InlineAsm),
    While(While),
    Terminator(Terminator),
    For(For),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct For {
    pub var: String,
    pub start: Exp,
    pub end: Exp,
    pub step: Exp,
    pub then: Block,
    pub span: Span,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AsmConstraint {
    In(String, Exp, Span),
    Out(String, LVal, Span),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineAsm {
    pub asm: String,
    pub constraints: Vec<AsmConstraint>,
    pub span: Span,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Assign {
    WriteVar(LVal, Exp, Span),
    WritePtr(Deref, Exp, Span),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Return {
    pub exp: Exp,
    pub span: Span,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct If {
    pub cond: Exp,
    pub then: Block,
    pub else_then: Option<Block>,
    pub span: Span,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FuncCall {
    pub ids: Vec<String>,
    pub args: Vec<Exp>,
    pub span: Span,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BuiltinFunction {
    Import,
    Module,
    DoMagic,
    FirstModule,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuiltinFunctionCall {
    pub func: BuiltinFunction,
    pub args: Vec<Exp>,
    pub span: Span,
}

impl Deref {
    pub fn span(&self) -> Span {
        match self {
            Self::DerefExp(_, span) => span.clone(),
            Self::DerefId(_, span) => span.clone(),
            Self::DerefPtr(_, _, span) => span.clone(),
            Self::DerefPtrExp(_, _, span) => span.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Terminator {
    Break(Span),
    Continue(Span),
}

impl Terminator {
    pub fn span(&self) -> Span {
        match self {
            Self::Break(span) | Self::Continue(span) => span.clone(),
        }
    }
}
