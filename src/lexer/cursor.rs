use crate::lexer::errors::LexError;

pub(crate) struct Cursor<'cursor> {
    code: &'cursor [char],
    pub(crate) current: usize,
}

impl<'cursor> Cursor<'cursor> {
    pub(crate) fn new(code: &'cursor [char]) -> Self {
        Cursor { code, current: 0 }
    }

    pub(crate) fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            Some(self.offset(0)?)
        }
    }

    pub(crate) fn is_at_end(&self) -> bool {
        self.current >= self.code.len()
    }

    pub(crate) fn offset(&self, offset: usize) -> Option<char> {
        let index = self.current + offset;
        if self.code.len() > index {
            Some(self.code[index])
        } else {
            None
        }
    }

    pub(crate) fn offset_result(&self, offset: usize) -> Result<char, LexError> {
        let index = self.current + offset;
        if self.code.len() > index {
            Ok(self.code[index])
        } else {
            Err(LexError::UnexpectedEof)
        }
    }
}
