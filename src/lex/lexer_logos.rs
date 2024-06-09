use std::marker::PhantomData;

use logos::{Lexer as LogosLexer, Logos};

use super::{Lexer, Lexicon, Span, Token};


pub struct LogosLexerWrapper<'s, L: Lexicon, T: Logos<'s>> {
    inner: LogosLexer<'s, T>,
    _marker: PhantomData<L>,
}

impl<'s, T: Logos<'s>, L: Lexicon + From<T>> Lexer<'s> for LogosLexerWrapper<'s, L, T> {
    type L = L;

    fn next(&mut self) -> (Option<Span>, Option<Token<Self::L>>) {
        let mut has_invalid = false;
        let mut invalid_start = self.inner.span().end;
        let mut invalid_end = invalid_start;
        let mut token = None;
        loop {
            match self.next_internal() {
                None => break,
                Some(Ok(next)) => {
                    token = Some(next);
                    break;
                }
                _ => {}
            }
            let invalid_span = self.inner.span();
            // if no invalid detected so far, check if there were
            // ignored input, since we don't want those to be considered
            // invalid
            if !has_invalid {
                // logos lexer would have already skipped one character
                // when it's invalid, so we need use start instead of end
                invalid_start = invalid_span.start;
            }
            has_invalid = true;
            invalid_end = invalid_span.end;
        }
        let invalid_span = if has_invalid {
            Some(Span::new(invalid_start, invalid_end))
        } else {
            None
        };

        (invalid_span, token)
    }
}

impl<'s, T: Logos<'s>, L: Lexicon + From<T>> LogosLexerWrapper<'s, L, T> {
    fn next_internal(&mut self) -> Option<Result<Token<L>, ()>> {
        match self.inner.next() {
            Some(Ok(token)) => {
                let ty = L::from(token);
                let span = self.inner.span();
                Some(Ok(Token::new(span, ty)))
            }
            Some(Err(_)) => Some(Err(())),
            None => None,
        }
    }
}
