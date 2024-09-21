/*
 * @file    :   ast.rs
 * @time    :   2024/02/12 13:26:46
 * @author  :   zzjcarrot
 */

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConstExp {
    pub exp: Exp,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Array {
    List(Vec<Exp>, Span),
    Template(Exp, ConstExp, Span),
}

impl Array {
    pub fn get_span(&self) -> Span {
        match self {
            Array::List(_, span) => span.clone(),
            Array::Template(_, _, span) => span.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Exp {
    Exp(Box<Exp>, Span),
    Number(Number),
    LVal(Box<LVal>),
    FuncCall(FuncCall),
    Str(String, Span),
    Unary(UnaryOp, Box<Exp>, Span),
    Binary(Box<Exp>, BinaryOp, Box<Exp>, Span),
    GetAddr(Box<GetAddr>),
    Deref(Box<Deref>),
    Array(Box<Array>),
}

impl Exp {
    pub fn get_span(&self) -> Span {
        match self {
            Exp::Exp(_, span) => span.clone(),
            Exp::Number(number) => number.span.clone(),
            Exp::LVal(lval) => lval.span.clone(),
            Exp::FuncCall(func_call) => func_call.span.clone(),
            Exp::Unary(_, _, span) => span.clone(),
            Exp::Binary(_, _, _, span) => span.clone(),
            Exp::GetAddr(get_addr) => get_addr.span.clone(),
            Exp::Str(_, span) => span.clone(),
            Exp::Deref(deref) => deref.get_span(),
            Exp::Array(array) => array.get_span(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LVal {
    pub ids: Vec<String>,
    pub span: Span,
    pub exp: Option<Exp>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Deref {
    DerefId(LVal, Span),
    DerefExp(Exp, Span),
    DerefPtrExp(Exp, Exp, Span),
    DerefPtr(LVal, Exp, Span),
}

impl Deref {
    pub fn get_span(&self) -> Span {
        match self {
            Deref::DerefId(_, span) => span.clone(),
            Deref::DerefExp(_, span) => span.clone(),
            Deref::DerefPtrExp(_, _, span) => span.clone(),
            Deref::DerefPtr(_, _, span) => span.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAddr {
    pub lval: LVal,
    pub span: Span,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Number {
    pub num: u64,
    pub span: Span,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UnaryOp {
    Positive,
    Negative,
    Not,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BinaryOp {
    Mul,
    Div,
    Mod,
    Add,
    Sub,
    Lt,
    Gt,
    Le,
    Ge,
    Eq,
    Neq,
}
