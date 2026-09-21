use core::hash::{Hash, Hasher};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::format_ident;
use std::fmt::Write;
use syn::{
    AngleBracketedGenericArguments, AssocType, Expr, ExprCall, ExprPath, FnArg, GenericArgument,
    GenericParam, Generics, Ident, ImplItem, ItemImpl, Lifetime, LifetimeParam, Pat, Path,
    PathArguments, PathSegment, PredicateLifetime, PredicateType, QSelf, ReceiverKind, ReturnType,
    Signature, Stmt, TraitBound, TraitBoundModifiers, Type, TypeParamBound, TypePath,
    TypeTraitObject, TypeTuple, WhereClause, WherePredicate, parse_macro_input,
    punctuated::Punctuated,
    token::{As, Colon, Dyn, Eq, Gt, Lt, Paren, PathSep, RArrow, SelfType, Where},
    visit_mut::VisitMut,
};

// TODO: We should be able to change the generated `std::boxed` references to `alloc::boxed`, right?

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

fn fut_arg_with_output(ty: Type, async_trait_lt: &Lifetime) -> PathArguments {
    PathArguments::AngleBracketed(AngleBracketedGenericArguments {
        colon2_token: Some(PathSep::default()),
        lt_token: Lt::default(),
        args: Punctuated::from_iter([GenericArgument::Type(Type::TraitObject(TypeTraitObject {
            attrs: Vec::default(),
            dyn_token: Some(Dyn::default()),
            bounds: Punctuated::from_iter([
                TypeParamBound::Trait(TraitBound {
                    paren_token: None,
                    lifetimes: None,
                    modifiers: TraitBoundModifiers::default(),
                    maybe: None,
                    path: Path {
                        leading_colon: Some(PathSep::default()),
                        segments: Punctuated::from_iter([
                            PathSegment {
                                ident: format_ident!("core"),
                                arguments: PathArguments::None,
                            },
                            PathSegment {
                                ident: format_ident!("future"),
                                arguments: PathArguments::None,
                            },
                            PathSegment {
                                ident: format_ident!("Future"),
                                arguments: PathArguments::AngleBracketed(
                                    AngleBracketedGenericArguments {
                                        colon2_token: None,
                                        lt_token: Lt::default(),
                                        args: Punctuated::from_iter([GenericArgument::AssocType(
                                            AssocType {
                                                ident: format_ident!("Output"),
                                                generics: None,
                                                eq_token: Eq::default(),
                                                ty,
                                            },
                                        )]),
                                        gt_token: Gt::default(),
                                    },
                                ),
                            },
                        ]),
                    },
                }),
                TypeParamBound::Trait(TraitBound {
                    paren_token: None,
                    lifetimes: None,
                    modifiers: TraitBoundModifiers::default(),
                    maybe: None,
                    path: Path {
                        leading_colon: Some(PathSep::default()),
                        segments: Punctuated::from_iter([
                            PathSegment {
                                ident: format_ident!("core"),
                                arguments: PathArguments::None,
                            },
                            PathSegment {
                                ident: format_ident!("marker"),
                                arguments: PathArguments::None,
                            },
                            PathSegment {
                                ident: format_ident!("Send"),
                                arguments: PathArguments::None,
                            },
                        ]),
                    },
                }),
                TypeParamBound::Lifetime(async_trait_lt.clone()),
            ]),
        }))]),
        gt_token: Gt::default(),
    })
}

fn lt_pred(lt_ident: Ident, async_trait_lt: &Lifetime) -> WherePredicate {
    WherePredicate::Lifetime(PredicateLifetime {
        attrs: Vec::new(),
        lifetime: Lifetime {
            apostrophe: Span::call_site(),
            ident: lt_ident,
        },
        colon_token: Colon::default(),
        bounds: Punctuated::from_iter([async_trait_lt.clone()]),
    })
}

fn transform_sig_output(output: &mut ReturnType, async_trait_lt: &Lifetime) {
    let inner_ty = match output.clone() {
        ReturnType::Default => Type::Tuple(TypeTuple {
            attrs: Vec::new(),
            paren_token: Paren::default(),
            elems: Punctuated::default(),
        }),
        ReturnType::Type(_, ret) => *ret,
    };

    *output = ReturnType::Type(
        RArrow::default(),
        Box::new(Type::Path(TypePath {
            attrs: Vec::new(),
            qself: None,
            path: Path {
                leading_colon: Some(PathSep::default()),
                segments: Punctuated::from_iter([
                    PathSegment {
                        ident: format_ident!("core"),
                        arguments: PathArguments::None,
                    },
                    PathSegment {
                        ident: format_ident!("pin"),
                        arguments: PathArguments::None,
                    },
                    PathSegment {
                        ident: format_ident!("Pin"),
                        arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                            colon2_token: Some(PathSep::default()),
                            lt_token: Lt::default(),
                            args: Punctuated::from_iter([GenericArgument::Type(Type::Path(
                                TypePath {
                                    attrs: Vec::new(),
                                    qself: None,
                                    path: Path {
                                        leading_colon: Some(PathSep::default()),
                                        segments: Punctuated::from_iter([
                                            PathSegment {
                                                ident: format_ident!("std"),
                                                arguments: PathArguments::None,
                                            },
                                            PathSegment {
                                                ident: format_ident!("boxed"),
                                                arguments: PathArguments::None,
                                            },
                                            PathSegment {
                                                ident: format_ident!("Box"),
                                                arguments: fut_arg_with_output(
                                                    inner_ty,
                                                    async_trait_lt,
                                                ),
                                            },
                                        ]),
                                    },
                                },
                            ))]),
                            gt_token: Gt::default(),
                        }),
                    },
                ]),
            },
        })),
    );
}

fn add_lifetime_bounds(sig: &mut Signature, async_trait_lt: &Lifetime) {
    sig.generics.lt_token = Some(Lt::default());
    sig.generics.gt_token = Some(Gt::default());

    for generic in sig.generics.params.pairs() {
        let GenericParam::Type(ty) = generic.into_value() else {
            continue;
        };

        let pred = WherePredicate::Type(PredicateType {
            attrs: Vec::new(),
            lifetimes: None,
            bounded_ty: Type::Path(TypePath {
                attrs: Vec::new(),
                qself: None,
                path: Path {
                    leading_colon: None,
                    segments: Punctuated::from_iter([PathSegment {
                        ident: ty.ident.clone(),
                        arguments: PathArguments::None,
                    }]),
                },
            }),
            colon_token: Colon::default(),
            bounds: Punctuated::from_iter([TypeParamBound::Lifetime(async_trait_lt.clone())]),
        });

        match &mut sig.generics.where_clause {
            Some(clause) => clause.predicates.push(pred),
            None => {
                sig.generics.where_clause = Some(WhereClause {
                    where_token: Where::default(),
                    predicates: Punctuated::from_iter([pred]),
                })
            }
        }
    }

    let mut lifetime_modifier = LifetimeModifier {
        found_lifetimes: 0,
        bound_lts: Vec::new(),
        async_trait_lt,
        generics: &mut sig.generics,
    };

    let args = &mut sig.inputs;
    for pair in args.pairs_mut() {
        let arg = pair.into_value();

        lifetime_modifier.visit_fn_arg_mut(arg);
    }

    sig.generics
        .params
        .push(GenericParam::Lifetime(LifetimeParam {
            attrs: Vec::new(),
            lifetime: async_trait_lt.clone(),
            colon_token: None,
            bounds: Punctuated::new(),
        }));
}

struct LifetimeModifier<'a, 'b> {
    found_lifetimes: usize,
    bound_lts: Vec<Lifetime>,
    async_trait_lt: &'a Lifetime,
    generics: &'b mut Generics,
}

impl<'a, 'b> LifetimeModifier<'a, 'b> {
    fn visit_maybe_lifetime_mut(&mut self, maybe: &mut Option<Lifetime>) {
        let lt = match maybe {
            None => {
                let lt = Lifetime {
                    apostrophe: Span::call_site(),
                    ident: format_ident!("life{}", self.found_lifetimes),
                };

                self.generics
                    .params
                    .push(GenericParam::Lifetime(LifetimeParam {
                        attrs: Vec::new(),
                        lifetime: Lifetime {
                            apostrophe: Span::call_site(),
                            ident: lt.ident.clone(),
                        },
                        colon_token: None,
                        bounds: Punctuated::new(),
                    }));
                self.found_lifetimes += 1;

                maybe.insert(lt)
            }
            Some(lt) => {
                if self.bound_lts.contains(lt) {
                    return;
                }

                self.bound_lts.push(lt.clone());
                lt
            }
        };

        // println!("🐈 lt: {i:#?}");
        match self.generics.where_clause.as_mut() {
            None => {
                self.generics.where_clause = Some(WhereClause {
                    where_token: Where::default(),
                    predicates: Punctuated::from_iter([lt_pred(
                        lt.ident.clone(),
                        self.async_trait_lt,
                    )]),
                })
            }
            Some(generics) => {
                generics
                    .predicates
                    .push(lt_pred(lt.ident.clone(), self.async_trait_lt));
            }
        }
    }
}

impl<'a, 'b> VisitMut for LifetimeModifier<'a, 'b> {
    fn visit_type_reference_mut(&mut self, i: &mut syn::TypeReference) {
        self.visit_maybe_lifetime_mut(&mut i.lifetime);
    }

    fn visit_receiver_mut(&mut self, rec: &mut syn::Receiver) {
        let pred = WherePredicate::Type(PredicateType {
            attrs: Vec::new(),
            lifetimes: None,
            bounded_ty: Type::Path(TypePath {
                attrs: Vec::new(),
                qself: None,
                path: Path {
                    leading_colon: None,
                    segments: Punctuated::from_iter([PathSegment {
                        ident: Ident::from(SelfType::default()),
                        arguments: PathArguments::None,
                    }]),
                },
            }),
            colon_token: Colon::default(),
            bounds: Punctuated::from_iter([TypeParamBound::Lifetime(self.async_trait_lt.clone())]),
        });

        match &mut self.generics.where_clause {
            Some(clause) => clause.predicates.push(pred),
            None => {
                self.generics.where_clause = Some(WhereClause {
                    where_token: Where::default(),
                    predicates: Punctuated::from_iter([pred]),
                })
            }
        }

        if let ReceiverKind::Reference(_, ref mut lt, _) = rec.kind {
            self.visit_maybe_lifetime_mut(lt);
        }
    }
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

    let async_trait_lt = Lifetime {
        apostrophe: Span::call_site(),
        ident: format_ident!("async_trait"),
    };

    for item in &mut input.items {
        if let ImplItem::Fn(f) = item
            && f.sig.asyncness.is_some()
        {
            add_lifetime_bounds(&mut f.sig, &async_trait_lt);
        }
    }

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

        // Reset its asyncness
        f.sig.asyncness = None;

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

        // And then change its return type to what async-trait does
        transform_sig_output(&mut f.sig.output, &async_trait_lt);

        let fn_name = &f.sig.ident;

        let mut new_args = Punctuated::new();
        for pair in f.sig.inputs.pairs() {
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

            new_args.push(Expr::Path(ExprPath {
                attrs: Vec::new(),
                qself: None,
                path: Path {
                    leading_colon: None,
                    segments: Punctuated::from_iter([PathSegment {
                        ident,
                        arguments: PathArguments::default(),
                    }]),
                },
            }))
        }

        let call_new_trait_fn = Expr::Call(ExprCall {
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

        let call_expr = Expr::Call(ExprCall {
            attrs: Vec::new(),
            func: Box::new(Expr::Path(ExprPath {
                attrs: Vec::new(),
                qself: None,
                path: Path {
                    leading_colon: Some(PathSep::default()),
                    segments: Punctuated::from_iter([
                        PathSegment {
                            ident: format_ident!("std"),
                            arguments: PathArguments::None,
                        },
                        PathSegment {
                            ident: format_ident!("boxed"),
                            arguments: PathArguments::None,
                        },
                        PathSegment {
                            ident: format_ident!("Box"),
                            arguments: PathArguments::None,
                        },
                        PathSegment {
                            ident: format_ident!("pin"),
                            arguments: PathArguments::None,
                        },
                    ]),
                },
            })),
            paren_token: Paren::default(),
            args: Punctuated::from_iter([call_new_trait_fn]),
        });

        f.block.stmts = vec![Stmt::Expr(call_expr, None)];
    }

    quote::quote! {
        #new_trait_ts

        // #[::async_trait::async_trait]
        #input
    }
    .into()
}
