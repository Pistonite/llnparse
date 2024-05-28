use crate::{Lexer, Span, SyntaxError, SyntaxErrorKind, SyntaxResult, SyntaxResultExt, SyntaxTree, Token, TokenStorage, TokenType};

pub struct Parser<'s, T, L, Ctx>
    where
        T: TokenType,
        L: Lexer<'s, T= T>,
    {
    /// The context
    pub context: Ctx,
    /// The parts of the source code that were unable to be parsed
    pub invalid_source: Vec<Span>,
    /// The tokens that were unable to be parsed with the syntax
    pub invalid_tokens: Vec<Token<T>>,
    /// The valid tokens parsed
    pub tokens: TokenStorage<T>,
    /// The encountered tokens that were marked with extract
    pub extracted_tokens: TokenStorage<T>,
    /// Errors encountered during parsing
    pub errors: Vec<SyntaxError>,
    /// The lexer
    lexer: L,
    /// The source code to parse
    source: &'s str,

    // position stack

    /// Position stack for backtracking. Elements are indices into tokens
    pos_stack: Vec<usize>,
    /// Current index in the tokens, this might be equal to tokens.len()
    /// meaning a new token should be read from the lexer
    idx: usize,
    /// The max state size for the pos stack
    max_stack_size: usize,
}
impl<'s, T, L> Parser<'s, T, L, ()> 
    where
        T: TokenType,
        L: Lexer<'s, T = T>,
        // ST: SyntaxTree<'s, T = T, Ctx=()>,
    {
    /// Create a new ParseState
    pub fn new(source: &'s str) -> Self {
        Self::new_with_context(source, ())
    }
}

impl<'s, T, L, Ctx> Parser<'s, T, L, Ctx> 
    where
        T: TokenType,
        L: Lexer<'s, T = T>,
        // ST: SyntaxTree<'s, T = T>,
    {
    /// Create a new ParseState
    pub fn new_with_context(source: &'s str, context: Ctx) -> Self {
        Self {
            source,
            lexer: L::new(source),
            context,
            extracted_tokens: TokenStorage::new(),
            invalid_source: Vec::new(),
            invalid_tokens: Vec::new(),
            tokens: TokenStorage::new(),
            pos_stack: Vec::new(),
            idx: 0,
            errors: Vec::new(),
            max_stack_size: 2048,
        }
    }

    pub fn set_max_stack_size(&mut self, size: usize) {
        self.max_stack_size = size;
    }

    /// Attempt to parse one syntax tree into the state, may skip invalid tokens and characters
    /// to form a valid syntax tree
    ///
    /// Return None if a valid syntax tree could not be formed
    /// when the end of the source is reached
    pub fn next<ST: SyntaxTree<T=T, Ctx=Ctx>>(&mut self) -> Option<ST> {
        let mut error_already_added = false;
        loop {
            match self.try_parse_internal() {
                Ok(tree) => {
                    return Some(tree);
                }
                Err(errors) => {
                    self.errors.extend(errors.errors);
                    if errors.obj.is_some() {
                        return errors.obj;
                    }
                }
            }
            // cannot parse a tree, skip a token and try again
            let token = match self.consume_token() {
                None => {
                    // no more tokens
                    return None;
                }
                Some(token) => token,
            };
            // only add error if this is the first token being skipped
            // for this attempt
            if !error_already_added {
                self.errors.push(
                    SyntaxError::new(
                        token.span, 
                        SyntaxErrorKind::UnexpectedToken));
                error_already_added = true;
            }
        }
    }

    /// Create an iterator that can be used to parse all syntax tree roots in the source
    pub fn iter<ST: SyntaxTree<T=T,Ctx=Ctx>>(&mut self) -> 
    ParserIter<'s, '_, T, L, ST> {
        ParserIter { state: self }
    }

    /// Parses all syntax trees in the source until the end
    pub fn parse_all
    <ST: SyntaxTree<T=T,Ctx=Ctx>>
    (&mut self) -> Vec<ST> {
        self.iter().collect()
    }

    /// Internally parse a syntax tree node from the state and apply semantic info
    fn try_parse_internal
    <ST: SyntaxTree<T=T,Ctx=Ctx>>
    (&mut self) -> SyntaxResult<ST> {
        ST::try_parse_ast(self).map_ext(|ast| {
            ST::into_parse_tree(ast, self)
        })
    }

    /// Get the token at self.idx, or None if the end of the source is reached
    fn get_or_read_token(&mut self) -> Option<Token<T>> {
        if let Some(token) = self.tokens.at(self.idx) {
            return Some(**token);
        }
        // read until a token that is not extract
        loop {
            let (invalid, token) = self.lexer.next();
            if let Some(span) = invalid {
                self.invalid_source.push(span);
                self.errors.push(SyntaxError::new(span, SyntaxErrorKind::UnexpectedCharacters));
            }
            if let Some(token) = token {
                if !token.ty.should_extract() {
                    self.tokens.add_last(token);
                    return Some(token);
                } 
                self.extracted_tokens.add_last(token);
                continue;
            }
            return None;
        }
    }
}

pub struct ParserIter<'s, 'p, T, L, ST> 
    where
        T: TokenType,
        L: Lexer<'s, T= T>,
        ST: SyntaxTree<T= T>,
    {
    state: &'p mut Parser<'s, T, L, ST::Ctx>,
}

impl<'s, 'p, T, L,ST> Iterator for ParserIter<'s, 'p, T, L, ST> 
    where
        T: TokenType,
        L: Lexer<'s, T= T>,
        ST: SyntaxTree<T= T>,
    {
    type Item = ST;

    fn next(&mut self) -> Option<Self::Item> {
        self.state.next()
    }
}


/// Internal state management for the parser
///
/// ## Note
/// This trait is meant to be only used internally by the [`SyntaxTree`] nodes.
pub trait ParserState<'s> {
    type T: TokenType;
    type Ctx;
    /// Push the current position to the stack so it can be restored later
    ///
    /// Returns false if the stack is full
    fn push_state(&mut self) -> SyntaxResult<()>;

    /// Pop the stack but do not restore the position
    fn pop_state(&mut self);

    /// Restore the position to stack top without popping
    fn restore_state(&mut self);

    /// Get the source code of a token
    fn get_src(&self, token: &Token<Self::T>) -> &'s str;

    /// Get an empty span at the current location
    fn current_span(&self) -> Span;

    /// Get and consume the current token, advancing the token stream position
    fn consume_token(&mut self) -> Option<Token<Self::T>>;

    /// Create a syntax error for an unexpected end of file
    fn unexpected_eof(&self) -> SyntaxError;
}

impl<'s, T, L, Ctx> ParserState<'s> for Parser<'s, T, L, Ctx> 
    where
        T: TokenType,
        L: Lexer<'s, T = T>,
        // ST: SyntaxTree<'s, T = T>,
    {

    type T = T;
    type Ctx = Ctx;

    fn push_state(&mut self) -> SyntaxResult<()> {
        if self.pos_stack.len() >= self.max_stack_size {
            return SyntaxError::new(
                Span::new(self.idx, self.idx),
                SyntaxErrorKind::StackOverflow,
            ).into();
        }
        self.pos_stack.push(self.idx);
        Ok(())
    }

    #[inline]
    fn pop_state(&mut self) {
        self.pos_stack.pop();
    }

    fn restore_state(&mut self) {
        if let Some(pos) = self.pos_stack.last() {
            self.idx = *pos;
        }
    }

    fn get_src(&self, token: &Token<T>) -> &'s str {
        token.get_src(self.source)
    }

    fn current_span(&self) -> Span {
        Span::new(self.idx, self.idx)
    }

    fn consume_token(&mut self) -> Option<Token<T>> {
        let token = self.get_or_read_token();
        if token.is_some() {
            self.idx += 1;
        }
        token
    }

    fn unexpected_eof(&self) -> SyntaxError {
        SyntaxError::new(Span::new(self.source.len(), self.source.len()), SyntaxErrorKind::UnexpectedEof)
    }
}
