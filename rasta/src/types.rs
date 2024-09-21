use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum VTypeEnum {
    U64,
    I8,
    Void,
    Others(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VType {
    pub ty: VTypeEnum,
    pub star: usize,
    pub span: Span,
}
