use crate::{codes::ErrCode, severity::ErrSeverity};
use ammonia_defs::source::Span;

#[derive(Debug, PartialEq, Eq)]
pub struct AmmoniaErr {
    pub code: ErrCode,
    pub severity: ErrSeverity,
    pub spans: Vec<Span>,
}
