use std::collections::BTreeSet;

use crate::*;

pub fn expand(input: &mut syn::DeriveInput) -> syn::Result<TokenStream2> {
    if !input.generics.params.is_empty() {
        syn_error!(input, "derive_lexicon cannot be used with generics")
    }

    let teleparse = crate_ident();

    // parse the root attributes
    let RootAttr { ignore, terminal_parse } = parse_root_attributes(input)?;

    // parse the variant attributes
    // note this is done before checking the root attributes,
    // so the error messages are better
    let (variant_attrs, enum_data) = {
        // strip attributes early for better error experience
        let mut variant_attrs = Vec::new();
        // has to be enum
        match &mut input.data {
            syn::Data::Enum(data) => {
                for variant in &mut data.variants {
                    variant_attrs.push(strip_take_attrs(&mut variant.attrs));
                }
            }
            _ => syn_error!(input, "derive_lexicon can only be used with enums"),
        };

        let enum_data = match &input.data {
            syn::Data::Enum(data) => data,
            _ => unreachable!(),
        };

        (variant_attrs, enum_data)
    };

    // check ignore regex
    // let mut ignores = Vec::new();
    for ignore in &ignore {
        checked_regex_rule(ignore)?;
        // ignores.push(ignore);
        // rule_exprs.push(quote! {
        //     #teleparse::lex::Rule::ignore(#ignore),
        // });
    }

    check_enum_precondition(enum_data, &variant_attrs)?;

    let repr_ty = match enum_data.variants.len() {
        0 => syn_error!(
            input,
            "derive_lexicon needs at least one variant in the enum"
        ),
        1..=8 => quote!(u8),
        9..=16 => quote!(u16),
        17..=32 => quote!(u32),
        33..=64 => quote!(u64),
        65..=128 => quote!(u128),
        _ => syn_error!(input, "derive_lexicon can have at most 128 variants"),
    };

    let enum_ident = &input.ident;
    let enum_vis = &input.vis;

    // parse enum body
    let mut variant_idents = Vec::new();
    let mut variant_derived_attrs = Vec::new();
    let mut variant_ordinals = Vec::new();
    let mut extract_idents = Vec::new();

    let mut extra_derives = TokenStream2::new();
    for (i, (variant, attrs)) in enum_data.variants.iter().zip(variant_attrs).enumerate() {
        let variant_ident = &variant.ident;
        variant_idents.push(variant_ident);
        variant_ordinals.push(i);

        // check for attributes
        let mut terminal = None;
        let mut regex = None;
        for attr in attrs {
            let metas = parse_crate_attr_meta(&attr)?;
            for meta in metas {
                let meta = match meta {
                    syn::Meta::List(meta) => meta,
                    _ => syn_error!(meta, "Unknown {} attribute", CRATE),
                };
                if meta.path.is_ident("regex") {
                    if regex.is_some() {
                        syn_error!(meta, "Multiple `regex` attributes found for enum variant! You can put all regexes into the same attribute and separate them with comma");
                    }
                    regex = Some(meta.parse_args_with(
                        Punctuated::<syn::LitStr, syn::Token![,]>::parse_terminated,
                    )?);
                    continue;
                }
                if meta.path.is_ident("terminal") {
                    if terminal.is_some() {
                        syn_error!(meta, "Multiple `terminal` attributes found for enum variant! You might want to merge them.");
                    }
                    terminal = Some(meta.parse_args_with(
                        Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
                    )?);
                    continue;
                }
            }
        }

        // if no terminal is provided, this token is extract type (like comments)
        if terminal.as_ref().map(|x| x.is_empty()).unwrap_or(true) {
            extract_idents.push(variant_ident);
        }

        // derive the terminals
        // infer_literals being None means we cannot infer rule from the literal
        // i.e. there is a terminal without literal, or there is a regex
        let mut infer_literals = Some(Vec::new());
        // literal set is used to check if the literal is matched by the regex
        // if any
        let mut infer_literal_set = BTreeSet::new();
        if let Some(terminal) = terminal {
            for meta in terminal {
                let ident = match meta.path().get_ident() {
                    Some(ident) => ident,
                    None => syn_error!(meta, "Identifier for terminal struct expected."),
                };
                match &meta {
                    // terminal(Ident)
                    syn::Meta::Path(_) => {
                        // since this will match any content of that token type
                        // we cannot infer literal patterns for the lexer
                        // if there's already a terminal without match, it's an error
                        if infer_literals.is_none() {
                            syn_error!(
                                &meta,
                                "There can only be one terminals without a matching literal. Having multiple is likely a mistake since they are interchangable in the AST."
                            );
                        }
                        infer_literals = None;
                        let doc = format!(
                            " Terminal symbol derived from [`{}`] with `terminal({})`",
                            enum_ident, ident
                        );
                        extra_derives.extend(derive_terminal(
                            terminal_parse,
                            &doc,
                            enum_vis,
                            enum_ident,
                            variant_ident,
                            ident,
                            None,
                        ));
                    }
                    // terminal(Ident = "literal")
                    syn::Meta::NameValue(meta) => {
                        let value = match &meta.value {
                            syn::Expr::Lit(syn::ExprLit {
                                lit: syn::Lit::Str(lit_str),
                                ..
                            }) => lit_str,
                            _ => syn_error!(meta, "Expected string literal"),
                        };
                        let match_lit = value.value();
                        if match_lit.is_empty() {
                            syn_error!(value, "Cannot use empty string for matching");
                        }
                        if let Some(infer_literals) = &mut infer_literals {
                            if infer_literal_set.contains(&match_lit) {
                                syn_error!(
                                    value,
                                    "Duplicate literal pattern `{}` for variant `{}`.",
                                    match_lit,
                                    variant_ident
                                );
                            }
                            infer_literals.push(match_lit.clone());
                        }
                        infer_literal_set.insert(match_lit);
                        let doc = format!(
                            " Terminal symbol derived from [`{}`] with `terminal({} = {})`",
                            enum_ident,
                            ident,
                            quote! {#value}
                        );
                        extra_derives.extend(derive_terminal(
                            terminal_parse,
                            &doc,
                            enum_vis,
                            enum_ident,
                            variant_ident,
                            ident,
                            Some(&value),
                        ));
                    }
                    _ => syn_error!(
                        &meta,
                        "Invalid form for terminal. Maybe you meant `terminal({} = ...)`?",
                        ident
                    ),
                }
            }
        }

        // if liternal is not inferred, regex must be provided
        if infer_literals
            .as_ref()
            .map(|x| x.is_empty())
            .unwrap_or(true)
            && regex.is_none()
        {
            syn_error!(variant, "Missing lexer rule for this variant. If all `terminal` are specified with a literal pattern, the rule can be inferred. Otherwise you must provide a `regex`");
        }
        // add the rules
        if let Some(regexes) = regex {
            if let Some(infer_literals) = infer_literals {
                if !infer_literals.is_empty() {
                    syn_error!(
                        variant,
                        "Defining `regex` here is redundant because all terminals have a literal match pattern, so the rule can already be inferred."
                    );
                }
            }
            let mut regex_attr = TokenStream2::new();
            for regex in regexes {
                let regex_obj = checked_regex_rule(&regex)?;
                for match_lit in &infer_literal_set {
                    // if we are able to match, we must be able to match the entire string
                    // For example, if the regex matches `key` and the literal is `keyboard`.
                    // If we were to match `keyboard`, `key` should be matched instead
                    if let Some(mat) = regex_obj.find(&match_lit) {
                        if mat.start() != 0 {
                            syn_error!(
                                regex,
                                "This regex does not match the beginning of `{}`. This is likely a mistake, because the terminal will never be matched",
                                match_lit,
                            );
                        }
                        if mat.end() != match_lit.len() {
                            syn_error!(
                                regex,
                                "This regex matches a proper prefix of `{}`. This is likely a mistake, because the terminal will never be matched (the prefix will instead)",
                                match_lit,
                            );
                        }
                    } else {
                        syn_error!(
                            regex,
                            "This regex does not match the literal `{}`. This is likely a mistake, because the terminal will never be matched",
                            match_lit,
                        );
                    }
                }
                regex_attr.extend(quote! {
                    #[regex(#regex)]
                });
                // rule_exprs.push(quote! {
                //     #teleparse::lex::Rule::token(#enum_ident::#variant_ident, #regex),
                // });
            }
            variant_derived_attrs.push(regex_attr);
        } else if let Some(infer_literals) = infer_literals {
            let mut token_attr = TokenStream2::new();
            for lit in infer_literals {
                token_attr.extend(quote! {
                    #[token(#lit)]
                });
            }
            variant_derived_attrs.push(token_attr);
            // rule_exprs.push(quote! {
            //     #teleparse::lex::Rule::token_literal(#enum_ident::#variant_ident, &[#slice]),
            // });
        }
    }
    // variants size checked when determining repr
    let enum_len = enum_data.variants.len();
    let enum_first_variant = &enum_data.variants.first().unwrap().ident;
    let enum_last_variant = &enum_data.variants.last().unwrap().ident;
    // let rule_count = rule_exprs.len();
    // let rule_exprs = rule_exprs
    //     .into_iter()
    //     .fold(TokenStream2::new(), |mut acc, expr| {
    //         acc.extend(expr);
    //         acc
    //     });
    let out = quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(usize)]
        #enum_vis enum #enum_ident {
            #( #variant_idents = #variant_ordinals, )*
        }
        #extra_derives
        const _: () = {
            #[derive(#teleparse::__priv::logos::Logos)]
        #(
            #[logos(skip #ignore)]
        )*
            pub enum DerivedLogos {
                #( #variant_derived_attrs #variant_idents, )*
            }
            #[automatically_derived]
            impl ::core::convert::From<DerivedLogos> for #enum_ident {
                fn from(token: DerivedLogos) -> Self {
                    match token {
                        #( DerivedLogos::#variant_idents => Self::#variant_idents, )*
                    }
                }
            }
            #[automatically_derived]
            impl #teleparse::Lexicon for #enum_ident {
                type Bit = #repr_ty;
                type Lexer<'s> = #teleparse::lex::LogosLexerWrapper<'s, Self, DerivedLogos>;
                type Map<T: Default + Clone> = [T; #enum_len];

                fn id(&self) -> usize { *self as usize }
                fn from_id(id: usize) -> Self { unsafe { std::mem::transmute(id) } }
                fn to_bit(&self) -> Self::Bit { (1 << self.id()) as Self::Bit }
                fn first() -> Self { Self::#enum_first_variant }

                fn next(&self) -> ::core::option::Option<Self> {
                    match self {
                        Self::#enum_last_variant => None,
                        _ => {
                            let next = self.id() + 1;
                            Some(Self::from_id(next))
                        }
                    }
                }

                fn should_extract(&self) -> bool {
                    match self {
                        #( Self::#extract_idents => true, )*
                        _ => false
                    }
                }

                fn lexer<'s>(source: &'s str) -> ::core::result::Result<Self::Lexer<'s>, #teleparse::GrammarError> {
                    use #teleparse::__priv::logos::Logos;
                    Ok(#teleparse::lex::LogosLexerWrapper::new( DerivedLogos::lexer(source)))
                }
            }
        };
    };

    Ok(out)
}

struct RootAttr {
    ignore: Vec<syn::LitStr>,
    terminal_parse: bool,
}

fn parse_root_attributes(input: &mut syn::DeriveInput) -> syn::Result<RootAttr> {
    let root_metas = parse_strip_root_meta_optional(input)?;
    let mut ignore = None;
    let mut terminal_parse = false;
    if let Some(root_metas) = root_metas {
        for meta in root_metas {
            match meta {
                syn::Meta::Path(meta) => {
                    if meta.is_ident("terminal_parse") {
                        terminal_parse = true;
                        continue;
                    }
                    syn_error!(meta, "Unknown attribute for derive_lexicon");
                }
                syn::Meta::List(meta) => {
                    if meta.path.is_ident("ignore") {
                        if ignore.is_some() {
                            syn_error!(meta, "Multiple `ignore` attributes found for derive_lexicon! You can put all regexes into the same `ignore` and separate them with comma");
                        }
                        ignore = Some(meta.parse_args_with(
                            Punctuated::<syn::LitStr, syn::Token![,]>::parse_terminated,
                        )?);
                        continue;
                    }
                    syn_error!(meta, "Unknown attribute for derive_lexicon");
                }
                _ => syn_error!(meta, "Unknown attribute for derive_lexicon"),
            }
        }
    }

    let ignore = match ignore {
        Some(ignore) => ignore.into_iter().collect::<Vec<_>>(),
        None => Vec::new(),
    };
    Ok(RootAttr { ignore, terminal_parse })
}

fn check_enum_precondition(enum_data: &syn::DataEnum, variant_attrs: &[Vec<syn::Attribute>]) -> syn::Result<()> {
    for (variant, attrs) in std::iter::zip(enum_data.variants.iter(), variant_attrs.iter()) {
        if !matches!(variant.fields, syn::Fields::Unit) {
            syn_error!(variant, "derive_lexicon must be used with enums with only unit variants. The integer values will be generated");
        }
        if attrs.is_empty() {
            syn_error!(
                variant,
                "derive_lexicon needs an `{}` attribute to derive the lexer",
                CRATE
            );
        }
    }

    Ok(())
}

/// Derive terminal struct and SyntaxTree implementation
fn derive_terminal(
    terminal_parse: bool,
    doc: &str,
    vis: &syn::Visibility,
    enum_ident: &syn::Ident,
    variant_ident: &syn::Ident,
    terminal_ident: &syn::Ident,
    match_lit: Option<&syn::LitStr>,
) -> TokenStream2 {
    let teleparse = crate_ident();
    let terminal_ident_str = terminal_ident.to_string();
    let match_option_impl = match match_lit {
        Some(match_lit) => quote! {Some(#match_lit) },
        None => quote! {None },
    };
    let match_parse_impl = match match_lit {
        Some(literal) => quote! {
            let follow = meta.follow.get(&Self::type_id());
            parser.parse_token_lit(#enum_ident::#variant_ident, #literal, follow).map(Self::from)
        },
        None => quote! {
            parser.parse_token(#enum_ident::#variant_ident).map(Self::from)
        }
    };
    let terminal_parse_impl = if terminal_parse {
        Some(root::expand(&terminal_ident, &terminal_ident))
    } else {
        None
    };
    quote! {
        #[doc = #doc]
        #[derive(Clone, Copy, PartialEq, Eq, Hash, #teleparse::ToSpan)]
        #vis struct #terminal_ident(pub #teleparse::Token<#enum_ident>);
        const _: () = {
            #[automatically_derived]
            impl ::core::convert::From<#teleparse::Token<#enum_ident>> for #terminal_ident {
                fn from(token: #teleparse::Token<#enum_ident>) -> Self {
                    Self(token)
                }
            }
            // Make the terminal struct transparent in the debug output
            #[automatically_derived]
            impl ::core::fmt::Debug for #terminal_ident {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    self.0.fmt(f)
                }
            }
            #[automatically_derived]
            impl #teleparse::AbstractSyntaxTree for #terminal_ident {
                type L = #enum_ident;
                fn debug() -> ::std::borrow::Cow<'static, str> { ::std::borrow::Cow::Borrowed(#terminal_ident_str) }
                fn build_first(builder: &mut #teleparse::syntax::FirstBuilder<Self::L>) {
                    let t = Self::type_id();
                    if builder.visit(t, #terminal_ident_str) {
                        let expr = #teleparse::syntax::FirstRel::insert_token(
                            t, 
                            #enum_ident::#variant_ident,
                            #match_option_impl
                        );
                        builder.add(expr);
                    }
                }
                fn check_left_recursive(
                    _seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
                    _stack: &mut ::std::vec::Vec<::std::string::String>, 
                    _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
                    _first: &#teleparse::syntax::First<Self::L>
                ) -> ::core::result::Result<(), #teleparse::GrammarError> {
                    // a terminal has no recursive rules
                    Ok(())
                }
                fn check_first_conflict(
                    _seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>, 
                    _first: &#teleparse::syntax::First<Self::L>
                ) -> ::core::result::Result<(), #teleparse::GrammarError> {
                    // a terminal has no recursive rules and therefore no conflicts
                    Ok(())
                }
                fn build_follow(_builder: &mut #teleparse::syntax::FollowBuilder<Self::L>) {
                    // no FOLLOW rules are produced from a terminal
                }
                fn check_first_follow_conflict(
                    _seen: &mut std::collections::BTreeSet<::core::any::TypeId>, 
                    _first: &#teleparse::syntax::First<Self::L>, 
                    _follow: &#teleparse::syntax::Follow<Self::L>
                ) -> ::core::result::Result<(), #teleparse::GrammarError> {
                    // terminals don't produce epsilon and therefore has no FIRST/FOLLOW conflict
                    Ok(())
                }
                fn build_jump(
                    _seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>, 
                    _first: &#teleparse::syntax::First<Self::L>,
                    _jump: &mut #teleparse::syntax::Jump<Self::L>
                ) {
                    // no parse table needed
                }
                #[inline]
                fn parse_ast<'s>(
                    parser: &mut #teleparse::Parser<'s, Self::L>, 
                    meta: &#teleparse::syntax::Metadata<Self::L>,
                ) -> #teleparse::syntax::Result<Self, Self::L> {
                    #match_parse_impl
                }
            }
            #[automatically_derived]
            impl #teleparse::ParseTree for #terminal_ident {
                type AST = Self;
                fn from_ast<'s>(ast: Self, _: &mut #teleparse::Parser<'s, #enum_ident>) -> Self {
                    ast
                }
            }
            #terminal_parse_impl
        };
    }
}
