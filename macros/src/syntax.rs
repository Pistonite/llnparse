use quote::{quote_spanned, ToTokens};

use crate::*;

pub fn expand(input: &mut syn::DeriveInput) -> syn::Result<TokenStream2> {
    let root_attr = parse_root_attributes(input)?;

    let teleparse = crate_ident();

    let (extra_derive, output) = match &mut input.data {
        syn::Data::Struct(data) => {
            (None, expand_struct(input.ident.clone(), data, &root_attr)?)
        }
        syn::Data::Enum(data) => {
            (Some(
                quote! {#[derive(#teleparse::ToSpan)]}
            ), expand_enum(input.ident.clone(), data, &root_attr)?)
        }
        _ => syn_error!(input, "derive_syntax can only be used with structs or enums")
    };

    let root_test = if !root_attr.root || root_attr.no_test {
        None
    } else {
        Some(root::expand_test(&input.ident))
    };

    let output = quote! {
        #extra_derive
        #input
        #output
        #root_test
    };

    Ok(output)
}

fn expand_struct(ident: syn::Ident, input: &mut syn::DataStruct, root_attr: &RootAttr) -> syn::Result<TokenStream2> {
    match &mut input.fields {
        syn::Fields::Unnamed(fields) => {
            expand_struct_unnamed(ident, fields, root_attr)
        }
        syn::Fields::Named(fields) => {
            expand_struct_named(ident, fields, root_attr)
        }
        syn::Fields::Unit => {
            syn_error!(ident, "derive_syntax does not support unit structs");
        }
    }
}

fn expand_struct_unnamed(ident: syn::Ident, input: &mut syn::FieldsUnnamed, root_attr: &RootAttr) -> syn::Result<TokenStream2> {
    if input.unnamed.is_empty() {
        syn_error!(input, "derive_syntax does not support struct with no fields");
    }
    let teleparse = crate_ident();
    let mut field_attrs = Vec::with_capacity(input.unnamed.len());
    for field in &mut input.unnamed {
        field_attrs.push(strip_take_attrs(&mut field.attrs));
    }
    let mut apply_semantic_impl = TokenStream2::new();
    for ((i, field), attrs) in std::iter::zip(input.unnamed.iter().enumerate(), field_attrs.into_iter()) {
        let field_ident = format_ident!("field_{}", i);
        let field_attr = parse_field_attributes(field, attrs)?;
        if let Some(semantic) = field_attr.semantic {
            apply_semantic_impl.extend(quote! {
                parser.apply_semantic(
                    &#field_ident,
                    #teleparse::token_set!(<Self::AST as #teleparse::syntax::AbstractSyntaxTree>::L{
                        #( #semantic )|*
                    })
                );
            });
        }
    }
    let pt_ty = input.unnamed.iter().map(|f| &f.ty).collect::<Vec<_>>();
    let pt_ident = (0..pt_ty.len()).map(|i| format_ident!("field_{}", i)).collect::<Vec<_>>();
    let last = syn::Index::from(pt_ty.len()-1);
    let indices = (0..pt_ty.len()).map(syn::Index::from).collect::<Vec<_>>();
    let root_derive = if root_attr.root {
        Some(root::expand(quote! { DerivedAST }, &ident))
    } else {
        None
    };
    let output = quote! {
        #[#teleparse::__priv::derive_ast(#ident)]
        struct DerivedAST(#( #teleparse::parser::AstOf< #pt_ty > ),*);
        #[automatically_derived]
        impl #teleparse::ToSpan for DerivedAST {
            fn span(&self) -> #teleparse::Span {
                #teleparse::Span::new(self.0.span().lo, self.#last.span().hi)
            }
        }
        #[automatically_derived]
        impl #teleparse::ToSpan for #ident {
            fn span(&self) -> #teleparse::Span {
                #teleparse::Span::new(self.0.span().lo, self.#last.span().hi)
            }
        }
        #[automatically_derived]
        impl #teleparse::parser::ParseTree for #ident {
            type AST = DerivedAST;
            fn from_ast<'s>(
                ast: Self::AST, 
                parser: &mut #teleparse::parser::Parser<'s, <Self::AST as #teleparse::syntax::AbstractSyntaxTree>::L>
            ) -> Self {
                let (#(#pt_ident),*) = ( #( <#pt_ty>::from_ast(ast.#indices, parser)),* );
                #apply_semantic_impl
                Self(#( #pt_ident),* )
            }
        }
        #root_derive
    };

    Ok(anon_const_block(output))
}

fn expand_struct_named(ident: syn::Ident, input: &mut syn::FieldsNamed, root_attr: &RootAttr) -> syn::Result<TokenStream2> {
    if input.named.is_empty() {
        syn_error!(input, "derive_syntax does not support struct with no fields");
    }
    let teleparse = crate_ident();

    let mut field_attrs = Vec::with_capacity(input.named.len());
    for field in &mut input.named {
        field_attrs.push(strip_take_attrs(&mut field.attrs));
    }
    let mut apply_semantic_impl = TokenStream2::new();
    for (field, attrs) in std::iter::zip(input.named.iter(), field_attrs.into_iter()) {
        let field_ident = field.ident.as_ref().unwrap();
        let field_attr = parse_field_attributes(field, attrs)?;
        if let Some(semantic) = field_attr.semantic {
            apply_semantic_impl.extend(quote! {
                parser.apply_semantic(
                    &#field_ident, 
                    #teleparse::token_set!(<Self::AST as #teleparse::syntax::AbstractSyntaxTree>::L{
                        #( #semantic )|*
                    })
                );
            });
        }
    }

    let pt_ty = input.named.iter().map(|f| &f.ty).collect::<Vec<_>>();
    let pt_ident = input.named.iter().map(|f| &f.ident).collect::<Vec<_>>();
    let first_ident = pt_ident.first().unwrap();
    let last_ident = pt_ident.last().unwrap();
    let last = syn::Index::from(pt_ty.len()-1);
    let indices = (0..pt_ty.len()).map(syn::Index::from);


    let root_derive = if root_attr.root {
        Some(root::expand(quote! { DerivedAST }, &ident))
    } else {
        None
    };
    let output = quote! {
        #[#teleparse::__priv::derive_ast(#ident)]
        struct DerivedAST(#( #teleparse::parser::AstOf< #pt_ty > ),*);
        #[automatically_derived]
        impl #teleparse::ToSpan for DerivedAST {
            fn span(&self) -> #teleparse::Span {
                #teleparse::Span::new(self.0.span().lo, self.#last.span().hi)
            }
        }
        #[automatically_derived]
        impl #teleparse::ToSpan for #ident {
            fn span(&self) -> #teleparse::Span {
                #teleparse::Span::new(self.#first_ident.span().lo, self.#last_ident.span().hi)
            }
        }
        #[automatically_derived]
        impl #teleparse::parser::ParseTree for #ident {
            type AST = DerivedAST;
            fn from_ast<'s>(
                ast: Self::AST, 
                parser: &mut #teleparse::parser::Parser<'s, <Self::AST as #teleparse::syntax::AbstractSyntaxTree>::L>
            ) -> Self {
                let ( #(#pt_ident),* ) = ( #( <#pt_ty>::from_ast(ast.#indices, parser)),*);
                #apply_semantic_impl
                Self { #(#pt_ident),* }
            }
        }
        #root_derive
    };
    Ok(anon_const_block(output))
}

fn expand_enum(ident: syn::Ident, input: &mut syn::DataEnum, root_attr: &RootAttr) -> syn::Result<TokenStream2> {
    let mut pt_ident = Vec::with_capacity(input.variants.len());
    let mut pt_ty: Vec<syn::Type> = Vec::with_capacity(input.variants.len());
    let mut variant_attrs = Vec::with_capacity(input.variants.len());
    for variant in &mut input.variants {
        if variant.discriminant.is_some() {
            syn_error!(variant, "derive_syntax does not support enums with discriminants");
        }
        variant_attrs.push(strip_take_attrs(&mut variant.attrs));
        pt_ident.push(variant.ident.clone());
        match &mut variant.fields {
            syn::Fields::Named(fields) => {
                syn_error!(fields, "derive_syntax does not support named fields in enums. Please extract them into a struct");
            }
            syn::Fields::Unnamed(fields) => {
                let mut iter = fields.unnamed.iter();
                let first = match iter.next() {
                    Some(x) => x,
                    None => {
                        syn_error!(fields, "enum variant in derive_syntax must either be unit or have a single unnamed field");
                    }
                };
                if iter.next().is_some() {
                    syn_error!(fields, "enum variant in derive_syntax must either be unit or have a single unnamed field");
                }
                pt_ty.push(first.ty.clone());
            }
            unit => {
                let id = &variant.ident;
                let t = quote_spanned! { id.span() => #id(#id) };
                let v = syn::parse2::<syn::Variant>(t).expect("internal error in derive_syntax: fail to parse enum variant");
                *unit = v.fields;
                let ty = ident_to_type(id);
                pt_ty.push(ty);
            }
        }
    }

    let teleparse = crate_ident();

    let mut apply_semantic_impl = Vec::with_capacity(input.variants.len());
    for (variant, attrs) in std::iter::zip(input.variants.iter(), variant_attrs.into_iter()) {
        let variant_attr = parse_field_attributes(variant, attrs)?;
        if let Some(semantic) = variant_attr.semantic {
            apply_semantic_impl.push(quote! {
                parser.apply_semantic(
                    &ast, 
                    #teleparse::token_set!(<Self::AST as #teleparse::syntax::AbstractSyntaxTree>::L{
                        #( #semantic )|*
                    })
                );
            });
        } else {
            apply_semantic_impl.push(quote! {});
        }
    }

    let root_derive = if root_attr.root {
        Some(root::expand(quote! { DeriveAST }, &ident))
    } else {
        None
    };
    
    let output = quote! {
        #[#teleparse::__priv::derive_ast(#ident)]
        #[derive(#teleparse::ToSpan)]
        #[doc(hidden)]
        enum DerivedAST {
            #( #pt_ident(#teleparse::parser::AstOf< #pt_ty >), )*
        }
        #[automatically_derived]
        impl #teleparse::parser::ParseTree for #ident {
            type AST = DerivedAST;

            fn from_ast<'s>(
                ast: Self::AST, 
                parser: &mut #teleparse::parser::Parser<'s, <Self::AST as #teleparse::syntax::AbstractSyntaxTree>::L>
            ) -> Self {
                match ast {
                    #(
                        DerivedAST::#pt_ident(ast) => {
                            let ast = <#pt_ty>::from_ast(ast, parser);
                            #apply_semantic_impl
                            Self::#pt_ident(ast)
                        }
                    )*
                }
            }
        }
        #root_derive
    };

    Ok(anon_const_block(output))
}

struct RootAttr {
    root: bool,
    no_test: bool,
}

fn parse_root_attributes(input: &mut syn::DeriveInput) -> syn::Result<RootAttr> {
    let root_metas = parse_strip_root_meta_optional(input)?;
    let mut root = false;
    let mut no_test = false;
    if let Some(root_metas) = root_metas {
        for meta in root_metas {
            match meta {
                syn::Meta::Path(path) => {
                    if path.is_ident("root") {
                        root = true;
                    } else if path.is_ident("no_test") {
                        no_test = true;
                    } else {
                        syn_error!(path, "Unknown attribute");
                    }
                }
                _ => {
                    syn_error!(meta, "Unknown attribute");
                }
            }
        }
    }
    if no_test && !root {
        syn_error!(input, "no_test can only be used with root");
    }
    Ok(RootAttr { root, no_test })
}

struct FieldAttr {
    semantic: Option<Vec<syn::Ident>>,
    hook: Option<syn::Ident>,
}

fn parse_field_attributes<T: ToTokens>(field: &T, attrs: Vec<syn::Attribute>) -> syn::Result<FieldAttr> {
    let mut semantic = None;
    let mut hook = None;
    let attr = match ensure_one(attrs) {
        EnsureOne::None => return Ok(FieldAttr { semantic, hook }),
        EnsureOne::More => syn_error!(field, "Multiple {} attributes found! You might want to merge them.", CRATE),
        EnsureOne::One(attr) => attr,
    };
    for meta in parse_crate_attr_meta(&attr)? {
        match meta {
            syn::Meta::List(meta) => {
                if meta.path.is_ident("semantic") {
                    if semantic.is_some() {
                        syn_error!(meta, "Duplicated `semantic` attribute. You might want to merge them.");
                    }
                    semantic = Some(meta.parse_args_with(Punctuated::<syn::Ident, syn::Token![,]>::parse_terminated)?
                    .into_iter().collect::<Vec<_>>());
                    continue;
                }
                if meta.path.is_ident("hook") {
                    if hook.is_some() {
                        syn_error!(meta, "Duplicated `hook` attribute. There can only be one hook per field. You can wrap the hooks in one function.");
                    }
                    hook = Some(meta.parse_args::<syn::Ident>()?);
                    continue;
                }
            }
            _ => syn_error!(meta, "Invalid attribute format. Expected <attr>(<args>)"),
        }
    }

    Ok(FieldAttr { semantic, hook })
}

