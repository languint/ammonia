use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub range: Range<usize>,
    pub slice: String,
    pub source_id: String,
}

impl ariadne::Span for Span {
    type SourceId = String;

    #[inline]
    fn contains(&self, offset: usize) -> bool {
        self.range.contains(&offset)
    }

    #[inline]
    fn start(&self) -> usize {
        self.range.start
    }

    #[inline]
    fn end(&self) -> usize {
        self.range.end
    }

    #[inline]
    fn source(&self) -> &Self::SourceId {
        &self.source_id
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.range.is_empty()
    }

    #[inline]
    fn len(&self) -> usize {
        self.range.end - self.range.start
    }
}
