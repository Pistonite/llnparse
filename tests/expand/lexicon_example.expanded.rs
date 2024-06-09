use teleparse::prelude::*;
#[repr(usize)]
pub enum TokenType {
    Integer = 0usize,
    Operator = 1usize,
    Param = 2usize,
}
#[automatically_derived]
impl ::core::fmt::Debug for TokenType {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                TokenType::Integer => "Integer",
                TokenType::Operator => "Operator",
                TokenType::Param => "Param",
            },
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for TokenType {
    #[inline]
    fn clone(&self) -> TokenType {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for TokenType {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for TokenType {}
#[automatically_derived]
impl ::core::cmp::PartialEq for TokenType {
    #[inline]
    fn eq(&self, other: &TokenType) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for TokenType {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::hash::Hash for TokenType {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_tag, state)
    }
}
/// Terminal symbol derived from [`TokenType`] with `terminal(Integer)`
pub struct Integer(pub teleparse::Token<TokenType>);
#[automatically_derived]
impl ::core::clone::Clone for Integer {
    #[inline]
    fn clone(&self) -> Integer {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenType>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Integer {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Integer {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Integer {
    #[inline]
    fn eq(&self, other: &Integer) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for Integer {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenType>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for Integer {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for Integer {
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
const _: () = {
    #[automatically_derived]
    impl ::core::convert::From<teleparse::Token<TokenType>> for Integer {
        fn from(token: teleparse::Token<TokenType>) -> Self {
            Self(token)
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Integer {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }
    #[automatically_derived]
    impl teleparse::AbstractSyntaxTree for Integer {
        type L = TokenType;
        fn debug() -> ::std::borrow::Cow<'static, str> {
            ::std::borrow::Cow::Borrowed("Integer")
        }
        fn build_first(builder: &mut teleparse::syntax::FirstBuilder<Self::L>) {
            let t = Self::type_id();
            if builder.visit(t, "Integer") {
                let expr = teleparse::syntax::FirstRel::insert_token(
                    t,
                    TokenType::Integer,
                    None,
                );
                builder.add(expr);
            }
        }
        fn check_left_recursive(
            _seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
            _stack: &mut ::std::vec::Vec<::std::string::String>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
            _first: &teleparse::syntax::First<Self::L>,
        ) -> ::core::result::Result<(), teleparse::GrammarError> {
            Ok(())
        }
        fn check_first_conflict(
            _seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
            _first: &teleparse::syntax::First<Self::L>,
        ) -> ::core::result::Result<(), teleparse::GrammarError> {
            Ok(())
        }
        fn build_follow(_builder: &mut teleparse::syntax::FollowBuilder<Self::L>) {}
        fn check_first_follow_conflict(
            _seen: &mut std::collections::BTreeSet<::core::any::TypeId>,
            _first: &teleparse::syntax::First<Self::L>,
            _follow: &teleparse::syntax::Follow<Self::L>,
        ) -> ::core::result::Result<(), teleparse::GrammarError> {
            Ok(())
        }
        fn build_jump(
            _seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
            _first: &teleparse::syntax::First<Self::L>,
            _jump: &mut teleparse::syntax::Jump<Self::L>,
        ) {}
        #[inline]
        fn parse_ast<'s>(
            parser: &mut teleparse::Parser<'s, Self::L>,
            meta: &teleparse::syntax::Metadata<Self::L>,
        ) -> teleparse::syntax::Result<Self, Self::L> {
            parser.parse_token(TokenType::Integer).map(Self::from)
        }
    }
    #[automatically_derived]
    impl teleparse::ParseTree for Integer {
        type AST = Self;
        fn from_ast<'s>(ast: Self, _: &mut teleparse::Parser<'s, TokenType>) -> Self {
            ast
        }
    }
};
/// Terminal symbol derived from [`TokenType`] with `terminal(OpAdd = "+")`
pub struct OpAdd(pub teleparse::Token<TokenType>);
#[automatically_derived]
impl ::core::clone::Clone for OpAdd {
    #[inline]
    fn clone(&self) -> OpAdd {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenType>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for OpAdd {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for OpAdd {}
#[automatically_derived]
impl ::core::cmp::PartialEq for OpAdd {
    #[inline]
    fn eq(&self, other: &OpAdd) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for OpAdd {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenType>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for OpAdd {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for OpAdd {
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
const _: () = {
    #[automatically_derived]
    impl ::core::convert::From<teleparse::Token<TokenType>> for OpAdd {
        fn from(token: teleparse::Token<TokenType>) -> Self {
            Self(token)
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for OpAdd {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }
    #[automatically_derived]
    impl teleparse::AbstractSyntaxTree for OpAdd {
        type L = TokenType;
        fn debug() -> ::std::borrow::Cow<'static, str> {
            ::std::borrow::Cow::Borrowed("OpAdd")
        }
        fn build_first(builder: &mut teleparse::syntax::FirstBuilder<Self::L>) {
            let t = Self::type_id();
            if builder.visit(t, "OpAdd") {
                let expr = teleparse::syntax::FirstRel::insert_token(
                    t,
                    TokenType::Operator,
                    Some("+"),
                );
                builder.add(expr);
            }
        }
        fn check_left_recursive(
            _seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
            _stack: &mut ::std::vec::Vec<::std::string::String>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
            _first: &teleparse::syntax::First<Self::L>,
        ) -> ::core::result::Result<(), teleparse::GrammarError> {
            Ok(())
        }
        fn check_first_conflict(
            _seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
            _first: &teleparse::syntax::First<Self::L>,
        ) -> ::core::result::Result<(), teleparse::GrammarError> {
            Ok(())
        }
        fn build_follow(_builder: &mut teleparse::syntax::FollowBuilder<Self::L>) {}
        fn check_first_follow_conflict(
            _seen: &mut std::collections::BTreeSet<::core::any::TypeId>,
            _first: &teleparse::syntax::First<Self::L>,
            _follow: &teleparse::syntax::Follow<Self::L>,
        ) -> ::core::result::Result<(), teleparse::GrammarError> {
            Ok(())
        }
        fn build_jump(
            _seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
            _first: &teleparse::syntax::First<Self::L>,
            _jump: &mut teleparse::syntax::Jump<Self::L>,
        ) {}
        #[inline]
        fn parse_ast<'s>(
            parser: &mut teleparse::Parser<'s, Self::L>,
            meta: &teleparse::syntax::Metadata<Self::L>,
        ) -> teleparse::syntax::Result<Self, Self::L> {
            let follow = meta.follow.get(&Self::type_id());
            parser.parse_token_lit(TokenType::Operator, "+", follow).map(Self::from)
        }
    }
    #[automatically_derived]
    impl teleparse::ParseTree for OpAdd {
        type AST = Self;
        fn from_ast<'s>(ast: Self, _: &mut teleparse::Parser<'s, TokenType>) -> Self {
            ast
        }
    }
};
/// Terminal symbol derived from [`TokenType`] with `terminal(OpMul = "*")`
pub struct OpMul(pub teleparse::Token<TokenType>);
#[automatically_derived]
impl ::core::clone::Clone for OpMul {
    #[inline]
    fn clone(&self) -> OpMul {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenType>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for OpMul {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for OpMul {}
#[automatically_derived]
impl ::core::cmp::PartialEq for OpMul {
    #[inline]
    fn eq(&self, other: &OpMul) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for OpMul {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenType>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for OpMul {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for OpMul {
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
const _: () = {
    #[automatically_derived]
    impl ::core::convert::From<teleparse::Token<TokenType>> for OpMul {
        fn from(token: teleparse::Token<TokenType>) -> Self {
            Self(token)
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for OpMul {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }
    #[automatically_derived]
    impl teleparse::AbstractSyntaxTree for OpMul {
        type L = TokenType;
        fn debug() -> ::std::borrow::Cow<'static, str> {
            ::std::borrow::Cow::Borrowed("OpMul")
        }
        fn build_first(builder: &mut teleparse::syntax::FirstBuilder<Self::L>) {
            let t = Self::type_id();
            if builder.visit(t, "OpMul") {
                let expr = teleparse::syntax::FirstRel::insert_token(
                    t,
                    TokenType::Operator,
                    Some("*"),
                );
                builder.add(expr);
            }
        }
        fn check_left_recursive(
            _seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
            _stack: &mut ::std::vec::Vec<::std::string::String>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
            _first: &teleparse::syntax::First<Self::L>,
        ) -> ::core::result::Result<(), teleparse::GrammarError> {
            Ok(())
        }
        fn check_first_conflict(
            _seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
            _first: &teleparse::syntax::First<Self::L>,
        ) -> ::core::result::Result<(), teleparse::GrammarError> {
            Ok(())
        }
        fn build_follow(_builder: &mut teleparse::syntax::FollowBuilder<Self::L>) {}
        fn check_first_follow_conflict(
            _seen: &mut std::collections::BTreeSet<::core::any::TypeId>,
            _first: &teleparse::syntax::First<Self::L>,
            _follow: &teleparse::syntax::Follow<Self::L>,
        ) -> ::core::result::Result<(), teleparse::GrammarError> {
            Ok(())
        }
        fn build_jump(
            _seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
            _first: &teleparse::syntax::First<Self::L>,
            _jump: &mut teleparse::syntax::Jump<Self::L>,
        ) {}
        #[inline]
        fn parse_ast<'s>(
            parser: &mut teleparse::Parser<'s, Self::L>,
            meta: &teleparse::syntax::Metadata<Self::L>,
        ) -> teleparse::syntax::Result<Self, Self::L> {
            let follow = meta.follow.get(&Self::type_id());
            parser.parse_token_lit(TokenType::Operator, "*", follow).map(Self::from)
        }
    }
    #[automatically_derived]
    impl teleparse::ParseTree for OpMul {
        type AST = Self;
        fn from_ast<'s>(ast: Self, _: &mut teleparse::Parser<'s, TokenType>) -> Self {
            ast
        }
    }
};
/// Terminal symbol derived from [`TokenType`] with `terminal(ParamOpen = "(")`
pub struct ParamOpen(pub teleparse::Token<TokenType>);
#[automatically_derived]
impl ::core::clone::Clone for ParamOpen {
    #[inline]
    fn clone(&self) -> ParamOpen {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenType>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for ParamOpen {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for ParamOpen {}
#[automatically_derived]
impl ::core::cmp::PartialEq for ParamOpen {
    #[inline]
    fn eq(&self, other: &ParamOpen) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for ParamOpen {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenType>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for ParamOpen {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for ParamOpen {
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
const _: () = {
    #[automatically_derived]
    impl ::core::convert::From<teleparse::Token<TokenType>> for ParamOpen {
        fn from(token: teleparse::Token<TokenType>) -> Self {
            Self(token)
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ParamOpen {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }
    #[automatically_derived]
    impl teleparse::AbstractSyntaxTree for ParamOpen {
        type L = TokenType;
        fn debug() -> ::std::borrow::Cow<'static, str> {
            ::std::borrow::Cow::Borrowed("ParamOpen")
        }
        fn build_first(builder: &mut teleparse::syntax::FirstBuilder<Self::L>) {
            let t = Self::type_id();
            if builder.visit(t, "ParamOpen") {
                let expr = teleparse::syntax::FirstRel::insert_token(
                    t,
                    TokenType::Param,
                    Some("("),
                );
                builder.add(expr);
            }
        }
        fn check_left_recursive(
            _seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
            _stack: &mut ::std::vec::Vec<::std::string::String>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
            _first: &teleparse::syntax::First<Self::L>,
        ) -> ::core::result::Result<(), teleparse::GrammarError> {
            Ok(())
        }
        fn check_first_conflict(
            _seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
            _first: &teleparse::syntax::First<Self::L>,
        ) -> ::core::result::Result<(), teleparse::GrammarError> {
            Ok(())
        }
        fn build_follow(_builder: &mut teleparse::syntax::FollowBuilder<Self::L>) {}
        fn check_first_follow_conflict(
            _seen: &mut std::collections::BTreeSet<::core::any::TypeId>,
            _first: &teleparse::syntax::First<Self::L>,
            _follow: &teleparse::syntax::Follow<Self::L>,
        ) -> ::core::result::Result<(), teleparse::GrammarError> {
            Ok(())
        }
        fn build_jump(
            _seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
            _first: &teleparse::syntax::First<Self::L>,
            _jump: &mut teleparse::syntax::Jump<Self::L>,
        ) {}
        #[inline]
        fn parse_ast<'s>(
            parser: &mut teleparse::Parser<'s, Self::L>,
            meta: &teleparse::syntax::Metadata<Self::L>,
        ) -> teleparse::syntax::Result<Self, Self::L> {
            let follow = meta.follow.get(&Self::type_id());
            parser.parse_token_lit(TokenType::Param, "(", follow).map(Self::from)
        }
    }
    #[automatically_derived]
    impl teleparse::ParseTree for ParamOpen {
        type AST = Self;
        fn from_ast<'s>(ast: Self, _: &mut teleparse::Parser<'s, TokenType>) -> Self {
            ast
        }
    }
};
/// Terminal symbol derived from [`TokenType`] with `terminal(ParamClose = ")")`
pub struct ParamClose(pub teleparse::Token<TokenType>);
#[automatically_derived]
impl ::core::clone::Clone for ParamClose {
    #[inline]
    fn clone(&self) -> ParamClose {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenType>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for ParamClose {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for ParamClose {}
#[automatically_derived]
impl ::core::cmp::PartialEq for ParamClose {
    #[inline]
    fn eq(&self, other: &ParamClose) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for ParamClose {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenType>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for ParamClose {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for ParamClose {
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
const _: () = {
    #[automatically_derived]
    impl ::core::convert::From<teleparse::Token<TokenType>> for ParamClose {
        fn from(token: teleparse::Token<TokenType>) -> Self {
            Self(token)
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ParamClose {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }
    #[automatically_derived]
    impl teleparse::AbstractSyntaxTree for ParamClose {
        type L = TokenType;
        fn debug() -> ::std::borrow::Cow<'static, str> {
            ::std::borrow::Cow::Borrowed("ParamClose")
        }
        fn build_first(builder: &mut teleparse::syntax::FirstBuilder<Self::L>) {
            let t = Self::type_id();
            if builder.visit(t, "ParamClose") {
                let expr = teleparse::syntax::FirstRel::insert_token(
                    t,
                    TokenType::Param,
                    Some(")"),
                );
                builder.add(expr);
            }
        }
        fn check_left_recursive(
            _seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
            _stack: &mut ::std::vec::Vec<::std::string::String>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
            _first: &teleparse::syntax::First<Self::L>,
        ) -> ::core::result::Result<(), teleparse::GrammarError> {
            Ok(())
        }
        fn check_first_conflict(
            _seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
            _first: &teleparse::syntax::First<Self::L>,
        ) -> ::core::result::Result<(), teleparse::GrammarError> {
            Ok(())
        }
        fn build_follow(_builder: &mut teleparse::syntax::FollowBuilder<Self::L>) {}
        fn check_first_follow_conflict(
            _seen: &mut std::collections::BTreeSet<::core::any::TypeId>,
            _first: &teleparse::syntax::First<Self::L>,
            _follow: &teleparse::syntax::Follow<Self::L>,
        ) -> ::core::result::Result<(), teleparse::GrammarError> {
            Ok(())
        }
        fn build_jump(
            _seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
            _first: &teleparse::syntax::First<Self::L>,
            _jump: &mut teleparse::syntax::Jump<Self::L>,
        ) {}
        #[inline]
        fn parse_ast<'s>(
            parser: &mut teleparse::Parser<'s, Self::L>,
            meta: &teleparse::syntax::Metadata<Self::L>,
        ) -> teleparse::syntax::Result<Self, Self::L> {
            let follow = meta.follow.get(&Self::type_id());
            parser.parse_token_lit(TokenType::Param, ")", follow).map(Self::from)
        }
    }
    #[automatically_derived]
    impl teleparse::ParseTree for ParamClose {
        type AST = Self;
        fn from_ast<'s>(ast: Self, _: &mut teleparse::Parser<'s, TokenType>) -> Self {
            ast
        }
    }
};
const _: () = {
    #[logos(skip r"\s")]
    pub enum DerivedLogos {
        #[regex(r"\d+")]
        Integer,
        #[token("+")]
        #[token("*")]
        Operator,
        #[token("(")]
        #[token(")")]
        Param,
    }
    impl<'s> ::logos::Logos<'s> for DerivedLogos {
        type Error = ();
        type Extras = ();
        type Source = str;
        fn lex(lex: &mut ::logos::Lexer<'s, Self>) {
            use ::logos::internal::{LexerInternal, CallbackResult};
            type Lexer<'s> = ::logos::Lexer<'s, DerivedLogos>;
            fn _end<'s>(lex: &mut Lexer<'s>) {
                lex.end()
            }
            fn _error<'s>(lex: &mut Lexer<'s>) {
                lex.bump_unchecked(1);
                lex.error();
            }
            #[inline]
            fn goto21_ctx21_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(DerivedLogos::Integer));
            }
            #[inline]
            fn goto21_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(DerivedLogos::Integer));
            }
            #[inline]
            fn goto29_at2_ctx21_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([166u8..=175u8]) => {
                        lex.bump_unchecked(3usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => goto21_x(lex),
                }
            }
            #[inline]
            fn goto48_at2_ctx21_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([144u8..=153u8]) => {
                        lex.bump_unchecked(3usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => goto21_x(lex),
                }
            }
            #[inline]
            fn goto23_at2_ctx21_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([160u8..=169u8]) => {
                        lex.bump_unchecked(3usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => goto21_x(lex),
                }
            }
            #[inline]
            fn goto52_at1_ctx21_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J29,
                    J48,
                    J23,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J29,
                        __,
                        J29,
                        __,
                        J29,
                        __,
                        J29,
                        __,
                        J29,
                        __,
                        J29,
                        __,
                        J29,
                        __,
                        J29,
                        __,
                        J29,
                        __,
                        J29,
                        __,
                        J48,
                        __,
                        J48,
                        J23,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto21_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J29 => goto29_at2_ctx21_x(lex),
                    Jump::J48 => goto48_at2_ctx21_x(lex),
                    Jump::J23 => goto23_at2_ctx21_x(lex),
                    Jump::__ => goto21_x(lex),
                }
            }
            #[inline]
            fn goto23_at1_ctx21_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(1usize) {
                    Some([160u8..=169u8]) => {
                        lex.bump_unchecked(2usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => goto21_x(lex),
                }
            }
            #[inline]
            fn pattern0(byte: u8) -> bool {
                match byte {
                    128u8..=137u8 | 144u8..=153u8 => true,
                    _ => false,
                }
            }
            #[inline]
            fn goto69_at2_ctx21_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto21_x(lex),
                };
                match byte {
                    byte if pattern0(byte) => {
                        lex.bump_unchecked(3usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => goto21_x(lex),
                }
            }
            #[inline]
            fn goto62_at2_ctx21_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([134u8..=143u8]) => {
                        lex.bump_unchecked(3usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => goto21_x(lex),
                }
            }
            #[inline]
            fn goto25_at2_ctx21_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([128u8..=137u8]) => {
                        lex.bump_unchecked(3usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => goto21_x(lex),
                }
            }
            #[inline]
            fn goto24_at2_ctx21_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([176u8..=185u8]) => {
                        lex.bump_unchecked(3usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => goto21_x(lex),
                }
            }
            #[inline]
            fn goto77_at1_ctx21_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J69,
                    J23,
                    J62,
                    J25,
                    J48,
                    J24,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J25,
                        J48,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J23,
                        J48,
                        __,
                        __,
                        __,
                        __,
                        J62,
                        __,
                        J48,
                        __,
                        __,
                        J69,
                        __,
                        __,
                        J48,
                        J24,
                        __,
                        __,
                        J69,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto21_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J69 => goto69_at2_ctx21_x(lex),
                    Jump::J23 => goto23_at2_ctx21_x(lex),
                    Jump::J62 => goto62_at2_ctx21_x(lex),
                    Jump::J25 => goto25_at2_ctx21_x(lex),
                    Jump::J48 => goto48_at2_ctx21_x(lex),
                    Jump::J24 => goto24_at2_ctx21_x(lex),
                    Jump::__ => goto21_x(lex),
                }
            }
            #[inline]
            fn goto173_at2_ctx21_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(2usize) {
                    Some([159u8, 142u8..=191u8]) => {
                        lex.bump_unchecked(4usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => goto21_x(lex),
                }
            }
            #[inline]
            fn goto25_at3_ctx21_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([128u8..=137u8]) => {
                        lex.bump_unchecked(4usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => goto21_x(lex),
                }
            }
            #[inline]
            fn goto24_at3_ctx21_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([176u8..=185u8]) => {
                        lex.bump_unchecked(4usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => goto21_x(lex),
                }
            }
            #[inline]
            fn goto48_at3_ctx21_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([144u8..=153u8]) => {
                        lex.bump_unchecked(4usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => goto21_x(lex),
                }
            }
            #[inline]
            fn goto186_at2_ctx21_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J25,
                    J24,
                    J48,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J25,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J24,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J24,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J48,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto21_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J25 => goto25_at3_ctx21_x(lex),
                    Jump::J24 => goto24_at3_ctx21_x(lex),
                    Jump::J48 => goto48_at3_ctx21_x(lex),
                    Jump::__ => goto21_x(lex),
                }
            }
            #[inline]
            fn goto23_at3_ctx21_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([160u8..=169u8]) => {
                        lex.bump_unchecked(4usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => goto21_x(lex),
                }
            }
            #[inline]
            fn goto97_at2_ctx21_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto21_x(lex),
                };
                match byte {
                    180u8 => goto24_at3_ctx21_x(lex),
                    146u8 => goto23_at3_ctx21_x(lex),
                    _ => goto21_x(lex),
                }
            }
            #[inline]
            fn goto29_at3_ctx21_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([166u8..=175u8]) => {
                        lex.bump_unchecked(4usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => goto21_x(lex),
                }
            }
            #[inline]
            fn goto109_at3_ctx21_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([182u8..=191u8]) => {
                        lex.bump_unchecked(4usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => goto21_x(lex),
                }
            }
            #[inline]
            fn goto160_at2_ctx21_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J29,
                    J109,
                    J23,
                    J25,
                    J24,
                    J48,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J29,
                        __,
                        J24,
                        J109,
                        __,
                        __,
                        J48,
                        __,
                        __,
                        __,
                        J24,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J48,
                        __,
                        J48,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J48,
                        __,
                        J25,
                        J24,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J23,
                        __,
                        J48,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J48,
                        __,
                        __,
                        __,
                        J48,
                        J23,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J48,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto21_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J29 => goto29_at3_ctx21_x(lex),
                    Jump::J109 => goto109_at3_ctx21_x(lex),
                    Jump::J23 => goto23_at3_ctx21_x(lex),
                    Jump::J25 => goto25_at3_ctx21_x(lex),
                    Jump::J24 => goto24_at3_ctx21_x(lex),
                    Jump::J48 => goto48_at3_ctx21_x(lex),
                    Jump::__ => goto21_x(lex),
                }
            }
            #[inline]
            fn goto89_at2_ctx21_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(2usize) {
                    Some([175u8, 176u8..=185u8]) => {
                        lex.bump_unchecked(4usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => goto21_x(lex),
                }
            }
            #[inline]
            fn goto170_at2_ctx21_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J25,
                    J48,
                    J23,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J23,
                        __,
                        J25,
                        __,
                        J48,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto21_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J25 => goto25_at3_ctx21_x(lex),
                    Jump::J48 => goto48_at3_ctx21_x(lex),
                    Jump::J23 => goto23_at3_ctx21_x(lex),
                    Jump::__ => goto21_x(lex),
                }
            }
            #[inline]
            fn goto188_at1_ctx21_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J173,
                    J186,
                    J97,
                    J160,
                    J89,
                    J170,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J97,
                        J160,
                        __,
                        __,
                        __,
                        __,
                        J170,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J173,
                        J186,
                        J89,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto21_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J173 => goto173_at2_ctx21_x(lex),
                    Jump::J186 => goto186_at2_ctx21_x(lex),
                    Jump::J97 => goto97_at2_ctx21_x(lex),
                    Jump::J160 => goto160_at2_ctx21_x(lex),
                    Jump::J89 => goto89_at2_ctx21_x(lex),
                    Jump::J170 => goto170_at2_ctx21_x(lex),
                    Jump::__ => goto21_x(lex),
                }
            }
            #[inline]
            fn goto25_at1_ctx21_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(1usize) {
                    Some([128u8..=137u8]) => {
                        lex.bump_unchecked(2usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => goto21_x(lex),
                }
            }
            #[inline]
            fn goto24_at1_ctx21_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(1usize) {
                    Some([176u8..=185u8]) => {
                        lex.bump_unchecked(2usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => goto21_x(lex),
                }
            }
            #[inline]
            fn goto91_at1_ctx21_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(1usize) {
                    Some([188u8, 144u8..=153u8]) => {
                        lex.bump_unchecked(3usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => goto21_x(lex),
                }
            }
            #[inline]
            fn pattern1(byte: u8) -> bool {
                match byte {
                    144u8..=153u8 | 176u8..=185u8 => true,
                    _ => false,
                }
            }
            #[inline]
            fn goto86_at2_ctx21_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return goto21_x(lex),
                };
                match byte {
                    byte if pattern1(byte) => {
                        lex.bump_unchecked(3usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => goto21_x(lex),
                }
            }
            #[inline]
            fn goto90_at1_ctx21_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J23,
                    J86,
                    J25,
                    J48,
                    J24,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J23,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J48,
                        J25,
                        __,
                        __,
                        J86,
                        __,
                        J48,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J24,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto21_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J23 => goto23_at2_ctx21_x(lex),
                    Jump::J86 => goto86_at2_ctx21_x(lex),
                    Jump::J25 => goto25_at2_ctx21_x(lex),
                    Jump::J48 => goto48_at2_ctx21_x(lex),
                    Jump::J24 => goto24_at2_ctx21_x(lex),
                    Jump::__ => goto21_x(lex),
                }
            }
            #[inline]
            fn goto22_ctx21_x<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J52,
                    J23,
                    J22,
                    J77,
                    J188,
                    J25,
                    J24,
                    J91,
                    J90,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J22,
                        J22,
                        J22,
                        J22,
                        J22,
                        J22,
                        J22,
                        J22,
                        J22,
                        J22,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J23,
                        __,
                        J24,
                        __,
                        __,
                        __,
                        J25,
                        J52,
                        J77,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J90,
                        __,
                        __,
                        __,
                        __,
                        J91,
                        J188,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                    ]
                };
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto21_ctx21_x(lex),
                };
                match LUT[byte as usize] {
                    Jump::J52 => goto52_at1_ctx21_x(lex),
                    Jump::J23 => goto23_at1_ctx21_x(lex),
                    Jump::J22 => {
                        lex.bump_unchecked(1usize);
                        goto22_ctx21_x(lex)
                    }
                    Jump::J77 => goto77_at1_ctx21_x(lex),
                    Jump::J188 => goto188_at1_ctx21_x(lex),
                    Jump::J25 => goto25_at1_ctx21_x(lex),
                    Jump::J24 => goto24_at1_ctx21_x(lex),
                    Jump::J91 => goto91_at1_ctx21_x(lex),
                    Jump::J90 => goto90_at1_ctx21_x(lex),
                    Jump::__ => goto21_ctx21_x(lex),
                }
            }
            #[inline]
            fn goto29_at2<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([166u8..=175u8]) => {
                        lex.bump_unchecked(3usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto48_at2<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([144u8..=153u8]) => {
                        lex.bump_unchecked(3usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto23_at2<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([160u8..=169u8]) => {
                        lex.bump_unchecked(3usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto52_at1<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J29,
                    J48,
                    J23,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J29,
                        __,
                        J29,
                        __,
                        J29,
                        __,
                        J29,
                        __,
                        J29,
                        __,
                        J29,
                        __,
                        J29,
                        __,
                        J29,
                        __,
                        J29,
                        __,
                        J29,
                        __,
                        J48,
                        __,
                        J48,
                        J23,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match LUT[byte as usize] {
                    Jump::J29 => goto29_at2(lex),
                    Jump::J48 => goto48_at2(lex),
                    Jump::J23 => goto23_at2(lex),
                    Jump::__ => _error(lex),
                }
            }
            #[inline]
            fn goto1_x<'s>(lex: &mut Lexer<'s>) {
                lex.trivia();
                DerivedLogos::lex(lex);
            }
            #[inline]
            fn goto193_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(DerivedLogos::Param));
            }
            #[inline]
            fn goto173_at2<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(2usize) {
                    Some([159u8, 142u8..=191u8]) => {
                        lex.bump_unchecked(4usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto25_at3<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([128u8..=137u8]) => {
                        lex.bump_unchecked(4usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto24_at3<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([176u8..=185u8]) => {
                        lex.bump_unchecked(4usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto48_at3<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([144u8..=153u8]) => {
                        lex.bump_unchecked(4usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto186_at2<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J25,
                    J24,
                    J48,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J25,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J24,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J24,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J48,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match LUT[byte as usize] {
                    Jump::J25 => goto25_at3(lex),
                    Jump::J24 => goto24_at3(lex),
                    Jump::J48 => goto48_at3(lex),
                    Jump::__ => _error(lex),
                }
            }
            #[inline]
            fn goto23_at3<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([160u8..=169u8]) => {
                        lex.bump_unchecked(4usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto97_at2<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match byte {
                    180u8 => goto24_at3(lex),
                    146u8 => goto23_at3(lex),
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto29_at3<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([166u8..=175u8]) => {
                        lex.bump_unchecked(4usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto109_at3<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(3usize) {
                    Some([182u8..=191u8]) => {
                        lex.bump_unchecked(4usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto160_at2<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J29,
                    J109,
                    J23,
                    J25,
                    J24,
                    J48,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J29,
                        __,
                        J24,
                        J109,
                        __,
                        __,
                        J48,
                        __,
                        __,
                        __,
                        J24,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J48,
                        __,
                        J48,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J48,
                        __,
                        J25,
                        J24,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J23,
                        __,
                        J48,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J48,
                        __,
                        __,
                        __,
                        J48,
                        J23,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J48,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match LUT[byte as usize] {
                    Jump::J29 => goto29_at3(lex),
                    Jump::J109 => goto109_at3(lex),
                    Jump::J23 => goto23_at3(lex),
                    Jump::J25 => goto25_at3(lex),
                    Jump::J24 => goto24_at3(lex),
                    Jump::J48 => goto48_at3(lex),
                    Jump::__ => _error(lex),
                }
            }
            #[inline]
            fn goto89_at2<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(2usize) {
                    Some([175u8, 176u8..=185u8]) => {
                        lex.bump_unchecked(4usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto170_at2<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J25,
                    J48,
                    J23,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J23,
                        __,
                        J25,
                        __,
                        J48,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                    ]
                };
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match LUT[byte as usize] {
                    Jump::J25 => goto25_at3(lex),
                    Jump::J48 => goto48_at3(lex),
                    Jump::J23 => goto23_at3(lex),
                    Jump::__ => _error(lex),
                }
            }
            #[inline]
            fn goto188_at1<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J173,
                    J186,
                    J97,
                    J160,
                    J89,
                    J170,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J97,
                        J160,
                        __,
                        __,
                        __,
                        __,
                        J170,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J173,
                        J186,
                        J89,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match LUT[byte as usize] {
                    Jump::J173 => goto173_at2(lex),
                    Jump::J186 => goto186_at2(lex),
                    Jump::J97 => goto97_at2(lex),
                    Jump::J160 => goto160_at2(lex),
                    Jump::J89 => goto89_at2(lex),
                    Jump::J170 => goto170_at2(lex),
                    Jump::__ => _error(lex),
                }
            }
            #[inline]
            fn goto190_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(DerivedLogos::Operator));
            }
            #[inline]
            fn goto24_at1<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(1usize) {
                    Some([176u8..=185u8]) => {
                        lex.bump_unchecked(2usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto86_at2<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match byte {
                    byte if pattern1(byte) => {
                        lex.bump_unchecked(3usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto25_at2<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([128u8..=137u8]) => {
                        lex.bump_unchecked(3usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto24_at2<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([176u8..=185u8]) => {
                        lex.bump_unchecked(3usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto90_at1<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J23,
                    J86,
                    J25,
                    J48,
                    J24,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J23,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J48,
                        J25,
                        __,
                        __,
                        J86,
                        __,
                        J48,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J24,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match LUT[byte as usize] {
                    Jump::J23 => goto23_at2(lex),
                    Jump::J86 => goto86_at2(lex),
                    Jump::J25 => goto25_at2(lex),
                    Jump::J48 => goto48_at2(lex),
                    Jump::J24 => goto24_at2(lex),
                    Jump::__ => _error(lex),
                }
            }
            #[inline]
            fn goto4_at1<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match byte {
                    133u8 | 160u8 => {
                        lex.bump_unchecked(2usize);
                        goto1_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto23_at1<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(1usize) {
                    Some([160u8..=169u8]) => {
                        lex.bump_unchecked(2usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn pattern2(byte: u8) -> bool {
                const LUT: u64 = 144036023240703u64;
                match 1u64.checked_shl(byte.wrapping_sub(128u8) as u32) {
                    Some(shift) => LUT & shift != 0,
                    None => false,
                }
            }
            #[inline]
            fn goto15_at2<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match byte {
                    byte if pattern2(byte) => {
                        lex.bump_unchecked(3usize);
                        goto1_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto18_at2<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some(&[159u8]) => {
                        lex.bump_unchecked(3usize);
                        goto1_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto17_at1<'s>(lex: &mut Lexer<'s>) {
                let arr = match lex.read_at::<&[u8; 2usize]>(1usize) {
                    Some(arr) => arr,
                    None => return _error(lex),
                };
                match arr[0] {
                    128u8 => goto15_at2(lex),
                    129u8 => goto18_at2(lex),
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto192_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(DerivedLogos::Param));
            }
            #[inline]
            fn goto19_at1<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(1usize) {
                    Some(&[128u8, 128u8]) => {
                        lex.bump_unchecked(3usize);
                        goto1_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto69_at2<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(2usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match byte {
                    byte if pattern0(byte) => {
                        lex.bump_unchecked(3usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto62_at2<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some([134u8..=143u8]) => {
                        lex.bump_unchecked(3usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto195_at2<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(2usize) {
                    Some(&[128u8]) => {
                        lex.bump_unchecked(3usize);
                        goto1_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto194_at1<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J69,
                    J23,
                    J62,
                    J25,
                    J48,
                    J195,
                    J24,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J25,
                        J48,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J195,
                        __,
                        __,
                        __,
                        __,
                        J23,
                        J48,
                        __,
                        __,
                        __,
                        __,
                        J62,
                        __,
                        J48,
                        __,
                        __,
                        J69,
                        __,
                        __,
                        J48,
                        J24,
                        __,
                        __,
                        J69,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                    ]
                };
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return _error(lex),
                };
                match LUT[byte as usize] {
                    Jump::J69 => goto69_at2(lex),
                    Jump::J23 => goto23_at2(lex),
                    Jump::J62 => goto62_at2(lex),
                    Jump::J25 => goto25_at2(lex),
                    Jump::J48 => goto48_at2(lex),
                    Jump::J195 => goto195_at2(lex),
                    Jump::J24 => goto24_at2(lex),
                    Jump::__ => _error(lex),
                }
            }
            #[inline]
            fn goto191_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(DerivedLogos::Operator));
            }
            #[inline]
            fn goto25_at1<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(1usize) {
                    Some([128u8..=137u8]) => {
                        lex.bump_unchecked(2usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto91_at1<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 2usize]>(1usize) {
                    Some([188u8, 144u8..=153u8]) => {
                        lex.bump_unchecked(3usize);
                        goto22_ctx21_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto196<'s>(lex: &mut Lexer<'s>) {
                enum Jump {
                    __,
                    J52,
                    J22,
                    J1,
                    J193,
                    J188,
                    J190,
                    J24,
                    J90,
                    J4,
                    J23,
                    J17,
                    J192,
                    J19,
                    J194,
                    J191,
                    J25,
                    J91,
                }
                const LUT: [Jump; 256] = {
                    use Jump::*;
                    [
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J1,
                        J1,
                        J1,
                        J1,
                        J1,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J1,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J192,
                        J193,
                        J191,
                        J190,
                        __,
                        __,
                        __,
                        __,
                        J22,
                        J22,
                        J22,
                        J22,
                        J22,
                        J22,
                        J22,
                        J22,
                        J22,
                        J22,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J4,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J23,
                        __,
                        J24,
                        __,
                        __,
                        __,
                        J25,
                        J52,
                        J194,
                        J17,
                        J19,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        J90,
                        __,
                        __,
                        __,
                        __,
                        J91,
                        J188,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                        __,
                    ]
                };
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return _end(lex),
                };
                match LUT[byte as usize] {
                    Jump::J52 => goto52_at1(lex),
                    Jump::J22 => {
                        lex.bump_unchecked(1usize);
                        goto22_ctx21_x(lex)
                    }
                    Jump::J1 => {
                        lex.bump_unchecked(1usize);
                        goto1_x(lex)
                    }
                    Jump::J193 => {
                        lex.bump_unchecked(1usize);
                        goto193_x(lex)
                    }
                    Jump::J188 => goto188_at1(lex),
                    Jump::J190 => {
                        lex.bump_unchecked(1usize);
                        goto190_x(lex)
                    }
                    Jump::J24 => goto24_at1(lex),
                    Jump::J90 => goto90_at1(lex),
                    Jump::J4 => goto4_at1(lex),
                    Jump::J23 => goto23_at1(lex),
                    Jump::J17 => goto17_at1(lex),
                    Jump::J192 => {
                        lex.bump_unchecked(1usize);
                        goto192_x(lex)
                    }
                    Jump::J19 => goto19_at1(lex),
                    Jump::J194 => goto194_at1(lex),
                    Jump::J191 => {
                        lex.bump_unchecked(1usize);
                        goto191_x(lex)
                    }
                    Jump::J25 => goto25_at1(lex),
                    Jump::J91 => goto91_at1(lex),
                    Jump::__ => _error(lex),
                }
            }
            goto196(lex)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<DerivedLogos> for TokenType {
        fn from(token: DerivedLogos) -> Self {
            match token {
                DerivedLogos::Integer => Self::Integer,
                DerivedLogos::Operator => Self::Operator,
                DerivedLogos::Param => Self::Param,
            }
        }
    }
    #[automatically_derived]
    impl teleparse::Lexicon for TokenType {
        type Bit = u8;
        type Lexer<'s> = teleparse::lex::LogosLexerWrapper<'s, Self, DerivedLogos>;
        type Map<T: Default + Clone> = [T; 3usize];
        fn id(&self) -> usize {
            *self as usize
        }
        fn from_id(id: usize) -> Self {
            unsafe { std::mem::transmute(id) }
        }
        fn to_bit(&self) -> Self::Bit {
            (1 << self.id()) as Self::Bit
        }
        fn first() -> Self {
            Self::Integer
        }
        fn next(&self) -> ::core::option::Option<Self> {
            match self {
                Self::Param => None,
                _ => {
                    let next = self.id() + 1;
                    Some(Self::from_id(next))
                }
            }
        }
        fn should_extract(&self) -> bool {
            match self {
                _ => false,
            }
        }
        fn lexer<'s>(
            source: &'s str,
        ) -> ::core::result::Result<Self::Lexer<'s>, teleparse::GrammarError> {
            use teleparse::__priv::logos::Logos;
            Ok(teleparse::lex::LogosLexerWrapper::new(DerivedLogos::lexer(source)))
        }
    }
};