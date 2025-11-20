pub mod error;

use ammonia_defs::source::Span;
use ariadne::Report;

use crate::{error::AmmoniaErr, reporter::error::ErrReporterErr};

pub struct ErrReporter<T = AmmoniaErr>
where
    T: Into<AmmoniaErr>,
{
    errors: Vec<T>,
}

impl<T> ErrReporter<T>
where
    T: Into<AmmoniaErr> + Clone,
{
    #[must_use]
    pub fn new(errors: Vec<T>) -> Self {
        Self { errors }
    }

    pub fn push(&mut self, err: T) {
        self.errors.push(err);
    }

    pub fn report_err(&self, err: &T) -> Result<Report<'_, Span>, ErrReporterErr> {
        let err: AmmoniaErr = err.clone().into();

        if err.spans.is_empty() {
            return Err(ErrReporterErr::NotEnoughSpans(
                "Errors cannot have zero spans!".into(),
            ));
        }

        let builder = Report::build(err.severity.into(), err.spans[0].clone());
        Ok(builder.finish())
    }
}
