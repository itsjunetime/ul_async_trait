use core::hash::{Hash, Hasher};
use proc_macro::TokenStream;
use quote::format_ident;
use std::fmt::Write;
use syn::{
    AngleBracketedGenericArguments, Expr, ExprAwait, ExprCall, ExprPath, FnArg, GenericArgument,
    GenericParam, Ident, ImplItem, ItemImpl, Pat, Path, PathArguments, PathSegment, QSelf, Stmt,
    Type, TypePath, parse_macro_input,
    punctuated::Punctuated,
    token::{As, Await, Comma, Dot, Gt, Lt, Paren, PathSep, SelfType},
};

fn ident_to_path(ident: Ident) -> Path {
    Path {
        leading_colon: None,
        segments: Punctuated::from_iter([PathSegment {
            ident,
            arguments: PathArguments::None,
        }]),
    }
}

fn ident_to_expr_path(ident: Ident) -> Expr {
    Expr::Path(ExprPath {
        attrs: Vec::new(),
        qself: None,
        path: ident_to_path(ident),
    })
}

fn ident_to_ty_path(ident: Ident) -> Type {
    Type::Path(TypePath {
        attrs: Vec::new(),
        qself: None,
        path: ident_to_path(ident),
    })
}

#[proc_macro_attribute]
pub fn async_trait(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemImpl);

    let Some((ref trait_name, _)) = input.trait_ else {
        return quote::quote! {
            compiler_error!("#[fast_async_trait::async_trait] may only be used on trait implementations");
        }.into();
    };

    let mut hasher = rustc_hash::FxHasher::default();
    input.self_ty.hash(&mut hasher);
    let trait_suffix = hasher.finish();

    let async_fn_sigs = input.items.iter().filter_map(|item| match item {
        ImplItem::Fn(f) if f.sig.asyncness.is_some() => {
            let mut f_sig = f.sig.clone();
            f_sig.ident = format_ident!("{}_{trait_suffix}", f_sig.ident);
            Some(f_sig)
        }
        _ => None,
    });

    let mut trait_str = String::new();
    for seg in &trait_name.segments {
        if !trait_str.is_empty() {
            trait_str.push('_');
        }
        write!(trait_str, "{}", seg.ident).unwrap();
    }

    let new_trait_name = quote::format_ident!("__async_bodies_of_{trait_str}_{trait_suffix}");

    let trait_arguments = &trait_name.segments.last().unwrap().arguments;
    let new_trait_impl = {
        let mut new_trait_impl = input.clone();
        let new_trait_ref = new_trait_impl.trait_.as_mut().unwrap();
        new_trait_ref.0.segments = Punctuated::from_iter([PathSegment {
            ident: new_trait_name.clone(),
            arguments: trait_arguments.clone(),
        }]);

        new_trait_impl.items.retain_mut(|i| match i {
            ImplItem::Fn(i) if i.sig.asyncness.is_some() => {
                i.sig.ident = format_ident!("{}_{trait_suffix}", i.sig.ident);
                true
            }
            _ => false,
        });
        new_trait_impl
    };

    let new_trait_ident = &new_trait_impl.trait_.as_ref().unwrap().0;
    let new_trait_ts = quote::quote! {
        trait #new_trait_ident {
            #(#async_fn_sigs;)*
        }

        #new_trait_impl
    };

    for item in &mut input.items {
        let ImplItem::Fn(f) = item else {
            continue;
        };

        if f.sig.asyncness.is_none() {
            continue;
        };

        let fn_name = &f.sig.ident;
        let args = &f.sig.inputs;

        let mut new_args = Punctuated::new();
        for pair in args.pairs() {
            if !new_args.is_empty() {
                new_args.push_punct(Comma::default());
            }

            let arg = pair.into_value();

            let ident = match arg {
                FnArg::Receiver(s) => Ident::from(s.self_token),
                FnArg::Typed(t) => match &*t.pat {
                    Pat::Ident(i) => i.ident.clone(),
                    _ => return quote::quote! {
                        compiler_error!("all types on a faster_async_trait fn must be bare identifiers; no patterns or destructuring allowed");
                    }.into(),
                }
            };

            let mut punct = Punctuated::new();
            punct.push_value(PathSegment {
                ident,
                arguments: PathArguments::default(),
            });

            new_args.push_value(Expr::Path(ExprPath {
                attrs: Vec::new(),
                qself: None,
                path: Path {
                    leading_colon: None,
                    segments: punct,
                },
            }))
        }

        let fn_path_args = if f.sig.generics.params.is_empty() {
            PathArguments::None
        } else {
            PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                colon2_token: Some(PathSep::default()),
                lt_token: Lt::default(),
                args: Punctuated::from_iter(f.sig.generics.params.pairs().map(|p| {
                    match p.into_value() {
                        GenericParam::Lifetime(lt) => {
                            GenericArgument::Lifetime(lt.lifetime.clone())
                        }
                        GenericParam::Type(t) => {
                            GenericArgument::Type(ident_to_ty_path(t.ident.clone()))
                        }
                        GenericParam::Const(c) => {
                            GenericArgument::Const(ident_to_expr_path(c.ident.clone()))
                        }
                    }
                })),
                gt_token: Gt::default(),
            })
        };

        let call_expr = Expr::Call(ExprCall {
            attrs: Vec::new(),
            func: Box::new(Expr::Path(ExprPath {
                attrs: Vec::new(),
                qself: Some(QSelf {
                    lt_token: Lt::default(),
                    ty: Box::new(ident_to_ty_path(Ident::from(SelfType::default()))),
                    position: 1,
                    as_token: Some(As::default()),
                    gt_token: Gt::default(),
                }),
                path: Path {
                    leading_colon: None,
                    segments: Punctuated::from_iter([
                        PathSegment {
                            ident: new_trait_name.clone(),
                            arguments: trait_arguments.clone(),
                        },
                        PathSegment {
                            ident: format_ident!("{fn_name}_{trait_suffix}"),
                            arguments: fn_path_args,
                        },
                    ]),
                },
            })),
            paren_token: Paren::default(),
            args: new_args,
        });
        f.block.stmts = vec![Stmt::Expr(
            Expr::Await(ExprAwait {
                attrs: Vec::new(),
                base: Box::new(call_expr),
                dot_token: Dot::default(),
                await_token: Await::default(),
            }),
            None,
        )];
    }

    quote::quote! {
        #new_trait_ts

        #[::async_trait::async_trait]
        #input
    }
    .into()
}
