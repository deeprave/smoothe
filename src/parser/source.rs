#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
}

impl SourceLocation {
    pub fn for_offset(source: &str, offset: usize) -> Self {
        let mut line = 1;
        let mut column = 1;

        for (byte_index, character) in source.char_indices() {
            if byte_index >= offset {
                break;
            }

            if character == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }

        Self { line, column }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceSpan {
    pub start: usize,
    pub end: usize,
}

impl SourceSpan {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}
