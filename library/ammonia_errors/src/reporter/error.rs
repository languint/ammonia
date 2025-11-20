use crate::codes::ErrCode;

#[derive(Debug)]
pub enum ErrReporterErr {
    NotEnoughSpans(String),
}

impl ErrReporterErr {
    pub fn to_code(&self) -> ErrCode {
        match self {
            ErrReporterErr::NotEnoughSpans(_) => ErrCode::E9000,
        }
    }
}
