use crate::parse::ParserSpan;
use std::cmp::Ordering;
use std::fmt::Display;

/// The line-column information
#[derive(Clone, Debug, Eq, Ord)]
pub struct LineColumn {
    pub(crate) offset: usize,
    /// The line
    pub line: usize,
    /// The column
    pub column: usize,
}

impl Display for LineColumn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "L{}:{}", self.line, self.column)
    }
}

impl PartialEq for LineColumn {
    fn eq(&self, other: &Self) -> bool {
        self.offset == other.offset
    }
}

impl PartialOrd for LineColumn {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.offset.partial_cmp(&other.offset)
    }
}

impl LineColumn {
    pub(crate) fn create_from(s: &ParserSpan<'_>) -> LineColumn {
        LineColumn {
            offset: s.location_offset(),
            line: s.location_line() as usize,
            column: s.get_utf8_column(),
        }
    }
}
