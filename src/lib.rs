use core::{
    fmt::Display,
    hash::{Hash, Hasher},
};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::format_ident;
use std::fmt::Write;
use syn::{
    AngleBracketedGenericArguments, AssocType, Attribute, Block, BlockModifiers, ConstParam, Expr,
    ExprAsync, ExprBlock, ExprCall, ExprIf, ExprLet, ExprPath, ExprReturn, FnArg, FnModifiers,
    GenericArgument, GenericParam, Generics, Ident, ImplItem, ItemImpl, ItemTrait, Lifetime,
    LifetimeParam, Local, LocalInit, LocalModifiers, Meta, MetaList, Pat, PatIdent, PatPath,
    PatTupleStruct, PatType, Path, PathArguments, PathSegment, PredicateLifetime, PredicateType,
    QSelf, ReceiverKind, ReturnType, Signature, Stmt, TraitBound, TraitBoundModifiers, TraitItem,
    TraitItemFn, TraitModifiers, Type, TypeParam, TypeParamBound, TypePath, TypeTraitObject,
    TypeTuple, WhereClause, WherePredicate, parse_macro_input,
    punctuated::Punctuated,
    token::{
        As, Async, Brace, Bracket, Colon, Comma, Dyn, Eq, For, Gt, If, Let, Lt, Move, Paren,
        PathSep, Pound, RArrow, Return, SelfType, Semi, Trait, Where,
    },
    visit_mut::VisitMut,
};

// TODO: We should be able to change the generated `std::boxed` references to `alloc::boxed`, right?

fn path_seg(p: impl Display) -> PathSegment {
    PathSegment {
        ident: format_ident!("{p}"),
        arguments: PathArguments::None,
    }
}

fn lt(ident: Ident) -> Lifetime {
    Lifetime {
        ident,
        apostrophe: Span::call_site(),
    }
}

fn ident_to_path(ident: Ident) -> Path {
    Path {
        leading_colon: None,
        segments: Punctuated::from_iter([path_seg(ident)]),
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
                            path_seg("core"),
                            path_seg("future"),
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
                            path_seg("core"),
                            path_seg("marker"),
                            path_seg("Send"),
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
        lifetime: lt(lt_ident),
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
                    path_seg("core"),
                    path_seg("pin"),
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
                                            path_seg("std"),
                                            path_seg("boxed"),
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

    for generic in &sig.generics.params {
        let GenericParam::Type(ty) = generic else {
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
                    segments: Punctuated::from_iter([path_seg(&ty.ident)]),
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

    for arg in &mut sig.inputs {
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

fn generic_params_to_args(params: &Punctuated<GenericParam, Comma>) -> PathArguments {
    if params.is_empty() {
        PathArguments::None
    } else {
        PathArguments::AngleBracketed(AngleBracketedGenericArguments {
            colon2_token: Some(PathSep::default()),
            lt_token: Lt::default(),
            args: Punctuated::from_iter(params.pairs().map(|p| match p.into_value() {
                GenericParam::Lifetime(lt) => GenericArgument::Lifetime(lt.lifetime.clone()),
                GenericParam::Type(t) => GenericArgument::Type(ident_to_ty_path(t.ident.clone())),
                GenericParam::Const(c) => {
                    GenericArgument::Const(ident_to_expr_path(c.ident.clone()))
                }
            })),
            gt_token: Gt::default(),
        })
    }
}

struct LifetimeUnifier;

impl LifetimeUnifier {
    fn change_lifetime_to_a(&mut self, l: &mut Option<Lifetime>) {
        *l = Some(lt(format_ident!("a")));
    }
}

impl VisitMut for LifetimeUnifier {
    fn visit_type_reference_mut(&mut self, i: &mut syn::TypeReference) {
        self.change_lifetime_to_a(&mut i.lifetime);
    }

    fn visit_receiver_kind_mut(&mut self, i: &mut syn::ReceiverKind) {
        if let ReceiverKind::Reference(_, lt, _) = i {
            self.change_lifetime_to_a(lt);
        }
    }
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
                let l = lt(format_ident!("life{}", self.found_lifetimes));

                self.generics
                    .params
                    .push(GenericParam::Lifetime(LifetimeParam {
                        attrs: Vec::new(),
                        lifetime: Lifetime {
                            apostrophe: Span::call_site(),
                            ident: l.ident.clone(),
                        },
                        colon_token: None,
                        bounds: Punctuated::new(),
                    }));
                self.found_lifetimes += 1;

                maybe.insert(l)
            }
            Some(lt) => {
                if self.bound_lts.contains(lt) {
                    return;
                }

                self.bound_lts.push(lt.clone());
                lt
            }
        };

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
                    segments: Punctuated::from_iter([path_seg(Ident::from(SelfType::default()))]),
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

fn make_inputs_not_mut_pats(args: &mut Punctuated<FnArg, Comma>) {
    for input in args {
        if let FnArg::Typed(pat_ty) = input
            && let Pat::Ident(ident) = &mut *pat_ty.pat
        {
            ident.mutability = None;
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

    let all_orig_trait_args = trait_name
        .segments
        .iter()
        .fold(Vec::new(), |mut args, seg| {
            match &seg.arguments {
                PathArguments::None => (),
                PathArguments::AngleBracketed(bracketed) => args.extend(bracketed.args.clone()),
                PathArguments::Parenthesized(parenthesized) => args.extend(
                    parenthesized
                        .inputs
                        .iter()
                        .map(|arg| GenericArgument::Type(arg.ty.clone())),
                ),
            }
            args
        });

    let mut hasher = rustc_hash::FxHasher::default();
    input.self_ty.hash(&mut hasher);
    all_orig_trait_args.hash(&mut hasher);
    trait_name.hash(&mut hasher);
    let trait_suffix = hasher.finish();

    let async_trait_lt = lt(format_ident!("async_trait"));
    let new_trait_name = format_ident!("__async_impl_{trait_suffix}");

    let mut trait_str = String::new();
    for seg in &trait_name.segments {
        if !trait_str.is_empty() {
            trait_str.push('_');
        }
        write!(trait_str, "{}", seg.ident).unwrap();
    }

    let fut_lifetime = lt(format_ident!("a"));

    // TODO: Preserve these spans from the original angle-bracketed args if there were any
    let new_trait_path_args = if !all_orig_trait_args.is_empty() {
        PathArguments::AngleBracketed(AngleBracketedGenericArguments {
            colon2_token: None,
            lt_token: Lt::default(),
            args: Punctuated::from_iter(all_orig_trait_args.clone()),
            gt_token: Gt::default(),
        })
    } else {
        PathArguments::None
    };

    let new_trait_path = Path {
        leading_colon: None,
        segments: Punctuated::from_iter([PathSegment {
            ident: new_trait_name.clone(),
            arguments: new_trait_path_args.clone(),
        }]),
    };

    let mut new_async_sigs = Vec::new();
    let new_impl = {
        let mut new_impl = input.clone();
        new_impl.trait_ = Some((new_trait_path.clone(), For::default()));

        new_impl.items.retain_mut(|i| match i {
            ImplItem::Fn(f) if f.sig.asyncness.is_some() => {
                f.sig.ident = format_ident!("{}_{trait_suffix}", f.sig.ident);
                f.sig.asyncness = None;

                LifetimeUnifier.visit_signature_mut(&mut f.sig);
                f.sig
                    .generics
                    .params
                    .push(GenericParam::Lifetime(LifetimeParam {
                        attrs: Vec::new(),
                        lifetime: fut_lifetime.clone(),
                        colon_token: None,
                        bounds: Punctuated::new(),
                    }));

                let orig_return_ty = match f.sig.output {
                    ReturnType::Default => Type::Tuple(TypeTuple {
                        attrs: Vec::new(),
                        paren_token: Paren::default(),
                        elems: Punctuated::new(),
                    }),
                    ReturnType::Type(_, ref ty) => (**ty).clone(),
                };
                transform_sig_output(&mut f.sig.output, &fut_lifetime);

                let local_return_ident = format_ident!("ret");
                let orig_type_hint_stmt = Stmt::Expr(
                    Expr::If(ExprIf {
                        attrs: Vec::new(),
                        if_token: If::default(),
                        cond: Box::new(Expr::Let(ExprLet {
                            attrs: Vec::new(),
                            let_token: Let::default(),
                            pat: Box::new(Pat::TupleStruct(PatTupleStruct {
                                // `::core::option::Some(ret)`
                                attrs: Vec::new(),
                                qself: None,
                                path: Path {
                                    leading_colon: Some(PathSep::default()),
                                    segments: Punctuated::from_iter([
                                        path_seg("core"),
                                        path_seg("option"),
                                        path_seg("Option"),
                                        path_seg("Some"),
                                    ]),
                                },
                                paren_token: Paren::default(),
                                elems: Punctuated::from_iter([Pat::Path(PatPath {
                                    attrs: Vec::new(),
                                    qself: None,
                                    path: ident_to_path(local_return_ident.clone()),
                                })]),
                            })),
                            eq_token: Eq::default(),
                            expr: Box::new(Expr::Path(ExprPath {
                                // ::core::Option::<orig_return_ty>::None
                                attrs: Vec::new(),
                                qself: None,
                                path: Path {
                                    leading_colon: Some(PathSep::default()),
                                    segments: Punctuated::from_iter([
                                        path_seg("core"),
                                        path_seg("option"),
                                        path_seg("Option"),
                                        PathSegment {
                                            ident: format_ident!("None"),
                                            arguments: PathArguments::AngleBracketed(
                                                AngleBracketedGenericArguments {
                                                    colon2_token: Some(PathSep::default()),
                                                    lt_token: Lt::default(),
                                                    args: Punctuated::from_iter([
                                                        GenericArgument::Type(
                                                            orig_return_ty.clone(),
                                                        ),
                                                    ]),
                                                    gt_token: Gt::default(),
                                                },
                                            ),
                                        },
                                    ]),
                                },
                            })),
                        })),
                        then_branch: Block {
                            brace_token: Brace::default(),
                            stmts: vec![Stmt::Expr(
                                Expr::Return(ExprReturn {
                                    attrs: Vec::new(),
                                    return_token: Return::default(),
                                    expr: Some(Box::new(Expr::Path(ExprPath {
                                        attrs: Vec::new(),
                                        qself: None,
                                        path: ident_to_path(local_return_ident.clone()),
                                    }))),
                                }),
                                None,
                            )],
                        },
                        else_branch: None,
                    }),
                    Some(Semi::default()),
                );

                let orig_block = f.block.clone();

                let local_stmt = Stmt::Local(Local {
                    attrs: Vec::new(),
                    let_token: Let::default(),
                    modifiers: LocalModifiers::default(),
                    pat: Pat::Type(PatType {
                        attrs: Vec::new(),
                        pat: Box::new(Pat::Ident(PatIdent {
                            attrs: Vec::new(),
                            by_ref: None,
                            mutability: None,
                            ident: local_return_ident.clone(),
                            subpat: None,
                        })),
                        colon_token: Colon::default(),
                        ty: Box::new(orig_return_ty),
                    }),
                    init: Some(LocalInit {
                        eq_token: Eq::default(),
                        expr: Box::new(Expr::Block(ExprBlock {
                            attrs: Vec::new(),
                            label: None,
                            block: orig_block,
                        })),
                        diverge: None,
                    }),
                    semi_token: Semi::default(),
                });

                let return_stmt = Stmt::Expr(
                    Expr::Path(ExprPath {
                        attrs: vec![Attribute {
                            pound_token: Pound::default(),
                            style: syn::AttrStyle::Outer,
                            bracket_token: Bracket::default(),
                            meta: Meta::List(MetaList {
                                path: ident_to_path(format_ident!("allow")),
                                delimiter: syn::MacroDelimiter::Paren(Paren::default()),
                                tokens: quote::quote! { unreachable_code },
                            }),
                        }],
                        qself: None,
                        path: ident_to_path(local_return_ident),
                    }),
                    None,
                );

                let call_expr = Expr::Call(ExprCall {
                    attrs: Vec::new(),
                    func: Box::new(Expr::Path(ExprPath {
                        attrs: Vec::new(),
                        qself: None,
                        path: Path {
                            leading_colon: Some(PathSep::default()),
                            segments: Punctuated::from_iter([
                                path_seg("std"),
                                path_seg("boxed"),
                                path_seg("Box"),
                                path_seg("pin"),
                            ]),
                        },
                    })),
                    paren_token: Paren::default(),
                    args: Punctuated::from_iter([Expr::Async(ExprAsync {
                        attrs: vec![Attribute {
                            pound_token: Pound::default(),
                            style: syn::AttrStyle::Outer,
                            bracket_token: Bracket::default(),
                            meta: Meta::List(MetaList {
                                path: ident_to_path(format_ident!("allow")),
                                delimiter: syn::MacroDelimiter::Paren(Paren::default()),
                                // this async lint doesn't fire on async functions that yield
                                // awaitable types, but does fire on async bodies that do.
                                tokens: quote::quote! { clippy::async_yields_async, clippy::diverging_sub_expression},
                            }),
                        }],
                        async_token: Async::default(),
                        capture: Some(Move::default()),
                        modifiers: BlockModifiers::default(),
                        block: Block {
                            brace_token: Brace::default(),
                            stmts: vec![orig_type_hint_stmt, local_stmt, return_stmt],
                        },
                    })]),
                });

                let mut new_async_sig = f.sig.clone();
                make_inputs_not_mut_pats(&mut new_async_sig.inputs);
                new_async_sigs.push(new_async_sig);

                f.block.stmts = vec![Stmt::Expr(call_expr, None)];
                true
            }
            _ => false,
        });

        new_impl
    };

    let new_trait_generic_params = input
        .generics
        .params
        .iter()
        .filter(|param| match param {
            GenericParam::Const(ConstParam { ident, .. })
            | GenericParam::Type(TypeParam { ident, .. }) => {
                all_orig_trait_args.iter().any(|arg| {
                    matches!(
                        arg,
                        GenericArgument::Type(Type::Path(TypePath { path, .. }))
                            if path.segments.last().is_some_and(|s| s.ident == *ident)
                                && path.segments.len() == 1
                    )
                })
            }
            GenericParam::Lifetime(LifetimeParam { lifetime, .. }) => {
                all_orig_trait_args.iter().any(|arg| {
                    matches!(
                        arg,
                        GenericArgument::Lifetime(lt) if lt.ident == lifetime.ident
                    )
                })
            }
        })
        .cloned()
        .collect::<Punctuated<GenericParam, Comma>>();

    let new_trait = ItemTrait {
        attrs: Vec::new(),
        vis: syn::Visibility::Inherited,
        modifiers: TraitModifiers::default(),
        unsafety: None,
        trait_token: Trait::default(),
        ident: new_trait_name.clone(),
        generics: Generics {
            lt_token: Some(Lt::default()),
            params: new_trait_generic_params,
            gt_token: Some(Gt::default()),
            // TODO: copy Where-clause over
            where_clause: None,
        },
        colon_token: None,
        supertraits: Punctuated::from_iter([TypeParamBound::Trait(TraitBound {
            paren_token: None,
            lifetimes: None,
            modifiers: TraitBoundModifiers::default(),
            maybe: None,
            path: trait_name.clone(),
        })]),
        brace_token: Brace::default(),
        items: new_async_sigs
            .into_iter()
            .map(|sig| {
                TraitItem::Fn(TraitItemFn {
                    attrs: Vec::new(),
                    modifiers: FnModifiers::default(),
                    sig,
                    default: None,
                    semi_token: Some(Semi::default()),
                })
            })
            .collect(),
    };

    let new_trait_ts = quote::quote! {
        #new_trait

        #new_impl
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

        let fn_path_args = generic_params_to_args(&f.sig.generics.params);

        // And then change its return type to what async-trait does
        transform_sig_output(&mut f.sig.output, &async_trait_lt);
        add_lifetime_bounds(&mut f.sig, &async_trait_lt);
        make_inputs_not_mut_pats(&mut f.sig.inputs);

        let fn_name = &f.sig.ident;

        let mut new_args = Punctuated::new();
        for arg in &f.sig.inputs {
            let ident = match arg {
                FnArg::Receiver(s) => Ident::from(s.self_token),
                FnArg::Typed(t) => match &*t.pat {
                    Pat::Ident(i) => i.ident.clone(),
                    _ => return quote::quote! {
                        compiler_error!("all types on a ul_async_trait fn must be bare identifiers; no patterns or destructuring allowed");
                    }.into(),
                }
            };

            new_args.push(Expr::Path(ExprPath {
                attrs: Vec::new(),
                qself: None,
                path: Path {
                    leading_colon: None,
                    segments: Punctuated::from_iter([path_seg(ident)]),
                },
            }))
        }

        let call_new_fn = ExprCall {
            attrs: Vec::new(),
            func: Box::new(Expr::Path(ExprPath {
                attrs: Vec::new(),
                qself: Some(QSelf {
                    lt_token: Lt::default(),
                    ty: Box::new(Type::Path(TypePath {
                        attrs: Vec::new(),
                        qself: None,
                        path: ident_to_path(Ident::from(SelfType::default())),
                    })),
                    position: 1,
                    as_token: Some(As::default()),
                    gt_token: Gt::default(),
                }),
                path: Path {
                    leading_colon: None,
                    segments: Punctuated::from_iter([
                        PathSegment {
                            ident: new_trait_name.clone(),
                            arguments: new_trait_path_args.clone(),
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
        };

        f.block.stmts = vec![Stmt::Expr(Expr::Call(call_new_fn), None)];
    }

    quote::quote! {
        #new_trait_ts

        #input
    }
    .into()
}
