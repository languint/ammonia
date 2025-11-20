use ariadne::{Color, ReportKind};

#[derive(Debug, PartialEq, Eq)]
pub enum ErrSeverity {
    Warning,
    Error,
    Advice,
    Custom(&'static str, Color),
}

impl From<ErrSeverity> for ReportKind<'_> {
    fn from(value: ErrSeverity) -> Self {
        match value {
            ErrSeverity::Error => ReportKind::Error,
            ErrSeverity::Warning => ReportKind::Warning,
            ErrSeverity::Advice => ReportKind::Advice,
            ErrSeverity::Custom(s, c) => ReportKind::Custom(s, c),
        }
    }
}
