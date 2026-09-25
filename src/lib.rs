use core::fmt::Display;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::format_ident;
use syn::{
    AngleBracketedGenericArguments, AssocType, Attribute, Block, BlockModifiers, ConstParam, Expr,
    ExprAsync, ExprBlock, ExprCall, ExprIf, ExprLet, ExprPath, ExprReturn, FnArg, FnModifiers,
    GenericArgument, GenericParam, Generics, Ident, ImplItem, ImplItemConst, ImplItemFn,
    ImplItemType, Item, ItemFn, ItemImpl, Lifetime, LifetimeParam, Local, LocalInit,
    LocalModifiers, Meta, MetaList, Pat, PatIdent, PatPath, PatTupleStruct, PatType, Path,
    PathArguments, PathSegment, PredicateLifetime, PredicateType, QSelf, ReceiverKind, ReturnType,
    Signature, Stmt, TraitBound, TraitBoundModifiers, Type, TypeParam, TypeParamBound, TypePath,
    TypeReference, TypeTraitObject, TypeTuple, WhereClause, WherePredicate, parse_macro_input,
    punctuated::Punctuated,
    token::{
        As, Async, Brace, Bracket, Colon, Comma, Dyn, Eq, Fn, Gt, If, Let, Lt, Move, Mut, Paren,
        PathSep, Pound, RArrow, Return, SelfType, SelfValue, Semi, Where,
    },
    visit::Visit,
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

fn path_is_self_value(p: &Path) -> bool {
    p.leading_colon.is_none()
        && p.segments
            .last()
            .is_some_and(|l| l.ident == Ident::from(SelfValue::default()))
        && p.segments.len() == 1
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

#[expect(clippy::large_enum_variant)]
enum ChangeToTraitItem {
    No,
    Ty(Type),
    ConstOrFn,
}

struct SelfFixerUpper<'a> {
    change_self_to_ty: &'a Type,
    // TODO: These are necessary to change `Ty::Assoc` to `<Ty as Trait>::Assoc`, which won't be
    // necessary if https://github.com/rust-lang/rust/issues/104119 gets closed
    trait_items: &'a [ImplItem],
    trait_name: &'a Path,
}

enum PathWrapper<'a> {
    Expr(&'a mut ExprPath),
    Type(&'a mut Type),
    Pat(&'a mut PatTupleStruct),
}

impl PathWrapper<'_> {
    fn replace_with_ty(self, new_ty: Type) {
        match self {
            Self::Expr(ExprPath { qself, path, .. })
            | Self::Pat(PatTupleStruct { qself, path, .. }) => {
                *qself = Some(QSelf {
                    lt_token: Lt::default(),
                    ty: Box::new(new_ty),
                    position: 0,
                    as_token: None,
                    gt_token: Gt::default(),
                });
                *path = Path {
                    leading_colon: None,
                    segments: Punctuated::new(),
                };
            }
            Self::Type(ty) => *ty = new_ty,
        }
    }
}

impl SelfFixerUpper<'_> {
    fn ident_is_trait_item(&self, given_ident: &Ident) -> ChangeToTraitItem {
        for item in self.trait_items {
            match item {
                ImplItem::Const(ImplItemConst { ident, .. }) if ident == given_ident => {
                    return ChangeToTraitItem::ConstOrFn;
                }
                ImplItem::Type(ImplItemType { ident, ty, .. }) if ident == given_ident => {
                    return ChangeToTraitItem::Ty(ty.clone());
                }
                ImplItem::Fn(ImplItemFn { sig, .. }) if sig.ident == *given_ident => {
                    return ChangeToTraitItem::ConstOrFn;
                }
                _ => (),
            }
        }

        ChangeToTraitItem::No
    }

    fn fixup_simple_path(&mut self, wrapper: PathWrapper<'_>) {
        let (qself, path) = match wrapper {
            PathWrapper::Expr(ExprPath { qself, path, .. }) => (qself, path),
            PathWrapper::Type(Type::Path(TypePath { qself, path, .. })) => (qself, path),
            PathWrapper::Pat(PatTupleStruct { qself, path, .. }) => (qself, path),
            PathWrapper::Type(_) => return,
        };

        if path.leading_colon.is_none()
            && path
                .segments
                .first()
                .is_some_and(|seg| seg.ident == Ident::from(SelfType::default()))
        {
            match path.segments.get(1) {
                // if path is just `Self`, then we can just completely replace the type
                None => wrapper.replace_with_ty(self.change_self_to_ty.clone()),
                // If there's more, then it's like `Self::AssocType`. `assoc_item` is `AssocType`.
                Some(assoc_item) => {
                    // otherwise, we want to make it a qself.
                    let mut new_qself = QSelf {
                        lt_token: Lt::default(),
                        ty: Box::new(self.change_self_to_ty.clone()),
                        position: 0,
                        as_token: None,
                        gt_token: Gt::default(),
                    };

                    // Even if there's another trait in-scope which has the same associated type, if
                    // it's in the body of this trait fn, it uses the correct one without asking, so we
                    // can comfortably replace them here. See:
                    // https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=0ab59d006a37db90ce97ee2f7bdca781

                    match self.ident_is_trait_item(&assoc_item.ident) {
                        // if the assoc item is not something that we know of, then we want to
                        // change the type to `<Ty>::Assoc`. Need to use QSelf in case `Ty` isn't a
                        // path ty
                        ChangeToTraitItem::No => match self.change_self_to_ty {
                            Type::Path(ty_path) => {
                                // TODO: ugh i guess we have to figure out a way to pipe attributes
                                // attrs.extend(ty_path.attrs);
                                *qself = None;
                                *path = Path {
                                    segments: ty_path
                                        .path
                                        .segments
                                        .iter()
                                        .chain(path.segments.iter().skip(1))
                                        .cloned()
                                        .collect(),
                                    leading_colon: ty_path.path.leading_colon,
                                };
                            }
                            _ => {
                                *qself = Some(new_qself);
                                *path = Path {
                                    segments: path.segments.iter().skip(1).cloned().collect(),
                                    leading_colon: Some(PathSep::default()),
                                };
                            }
                        },
                        // if it's a const or a fn that we're aware of, we can't resolve it to a
                        // concrete type. so we want to make it `<Ty as ::module::Trait>::fn_name` or
                        // `<Ty or ::module::Trait>::CONST`
                        ChangeToTraitItem::ConstOrFn => {
                            new_qself.as_token = Some(As::default());
                            new_qself.position = self.trait_name.segments.len();
                            *qself = Some(new_qself);
                            *path = Path {
                                segments: Punctuated::from_iter(
                                    self.trait_name
                                        .segments
                                        .iter()
                                        .chain(path.segments.iter().skip(1))
                                        .cloned(),
                                ),
                                leading_colon: self.trait_name.leading_colon,
                            };
                        }
                        // if the path is e.g. `Self::Assoc::OtherAssoc`, then we want to turn it
                        // into `<AssocType>::OtherAssoc`. But if it's `Self::Assoc` then we want to
                        // turn it into `AssocType`
                        ChangeToTraitItem::Ty(ty) => {
                            if path.segments.len() > 2 {
                                // need to just change QSelf, with `ty` as base
                                *qself = Some(QSelf {
                                    lt_token: Lt::default(),
                                    ty: Box::new(ty),
                                    position: 0,
                                    as_token: None,
                                    gt_token: Gt::default(),
                                });
                                path.segments = path.segments.iter().skip(2).cloned().collect();
                            } else {
                                wrapper.replace_with_ty(ty);
                            }
                        }
                    }
                }
            }
        }
    }
}

impl VisitMut for SelfFixerUpper<'_> {
    fn visit_expr_path_mut(&mut self, i: &mut syn::ExprPath) {
        if path_is_self_value(&i.path) {
            i.path.segments = Punctuated::from_iter([path_seg("slf")])
        }
        self.fixup_simple_path(PathWrapper::Expr(i));

        syn::visit_mut::visit_expr_path_mut(self, i);
    }

    fn visit_pat_ident_mut(&mut self, i: &mut syn::PatIdent) {
        if i.ident == Ident::from(SelfValue::default()) {
            i.ident = format_ident!("slf");
        }
    }

    fn visit_pat_tuple_struct_mut(&mut self, i: &mut syn::PatTupleStruct) {
        self.fixup_simple_path(PathWrapper::Pat(i));
        syn::visit_mut::visit_pat_tuple_struct_mut(self, i);
    }

    // Need to transform:
    // 1. `Self::Assoc` to `<Ty>::Assoc`
    // 2. `<Self as Thing>::Assoc` to `<Ty as Thing>::Assoc` (No brackets)
    // 3. `Self` to `Ty` (no brackets)
    //
    // So. if the type is path of length 1 and only `Self`, then we gotta just replace the type
    // entirely. otherwise, replace qself.
    fn visit_type_mut(&mut self, i: &mut syn::Type) {
        self.fixup_simple_path(PathWrapper::Type(i));

        syn::visit_mut::visit_type_mut(self, i);
    }

    fn visit_fn_arg_mut(&mut self, i: &mut syn::FnArg) {
        if let FnArg::Receiver(rcv) = i {
            let replace_ty_with = match &rcv.kind {
                ReceiverKind::Reference(and_token, lt, mut_tok) => {
                    let change_to_ty = Box::new(self.change_self_to_ty.clone());
                    Type::Reference(TypeReference {
                        attrs: Vec::new(),
                        and_token: *and_token,
                        lifetime: lt.clone(),
                        mutability: *mut_tok,
                        elem: change_to_ty,
                    })
                }
                ReceiverKind::Typed(_, ty) => {
                    let mut new_ty = (**ty).clone();
                    self.visit_type_mut(&mut new_ty);
                    new_ty
                }
                ReceiverKind::Value | _ => self.change_self_to_ty.clone(),
            };
            *i = FnArg::Typed(PatType {
                attrs: rcv.attrs.clone(),
                pat: Box::new(Pat::Ident(PatIdent {
                    attrs: Vec::new(),
                    by_ref: None,
                    mutability: None,
                    ident: format_ident!("slf"),
                    subpat: None,
                })),
                colon_token: Colon::default(),
                ty: Box::new(replace_ty_with),
            })
        }
        syn::visit_mut::visit_fn_arg_mut(self, i);
    }
}

#[proc_macro_attribute]
pub fn async_trait(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemImpl);

    let Some((ref trait_name, _)) = input.trait_ else {
        return quote::quote! {
            compile_error!("#[fast_async_trait::async_trait] may only be used on trait implementations");
        }.into();
    };

    let async_trait_lt = lt(format_ident!("async_trait"));
    let fut_lifetime = lt(format_ident!("a"));

    let inner_fn_ident = format_ident!("inner");
    let trait_items = input.items.clone();

    for item in &mut input.items {
        let ImplItem::Fn(f) = item else {
            continue;
        };

        if f.sig.asyncness.is_none() {
            continue;
        };

        // Reset its asyncness
        f.sig.asyncness = None;

        let orig_return_ty = match f.sig.output {
            ReturnType::Default => Type::Tuple(TypeTuple {
                attrs: Vec::new(),
                paren_token: Paren::default(),
                elems: Punctuated::new(),
            }),
            ReturnType::Type(_, ref ty) => (**ty).clone(),
        };

        let inner_fn_where_clause = Some(WhereClause {
            where_token: Where::default(),
            predicates: f
                .sig
                .generics
                .where_clause
                .iter()
                .flat_map(|w| &w.predicates)
                .chain(
                    input
                        .generics
                        .where_clause
                        .iter()
                        .flat_map(|w| &w.predicates),
                )
                .cloned()
                .collect(),
        });

        let mut inner_fn_generics = Generics {
            lt_token: None,
            params: Punctuated::from_iter([GenericParam::Lifetime(LifetimeParam {
                attrs: Vec::new(),
                lifetime: fut_lifetime.clone(),
                colon_token: None,
                bounds: Punctuated::new(),
            })]),
            gt_token: None,
            where_clause: inner_fn_where_clause,
        };

        if !f.sig.generics.params.is_empty() || !input.generics.params.is_empty() {
            inner_fn_generics.lt_token = Some(Lt::default());
            inner_fn_generics.params.extend(
                input
                    .generics
                    .params
                    .iter()
                    .chain(&f.sig.generics.params)
                    .cloned(),
            );
            inner_fn_generics.gt_token = Some(Gt::default());
        }

        // we're not specifying any args 'cause they're unified and we want them to be implied
        let inner_fn_generic_args = inner_fn_generics
            .params
            .iter()
            .filter_map(|p| match p {
                GenericParam::Lifetime(_) => None,
                GenericParam::Type(TypeParam { ident, .. })
                | GenericParam::Const(ConstParam { ident, .. }) => {
                    Some(GenericArgument::Type(ident_to_ty_path(ident.clone())))
                }
            })
            .collect();

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
                                            args: Punctuated::from_iter([GenericArgument::Type(
                                                orig_return_ty.clone(),
                                            )]),
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
                ty: Box::new(orig_return_ty.clone()),
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

        let mut new_fn_sig = Signature {
            constness: f.sig.constness,
            asyncness: None,
            safety: f.sig.safety.clone(),
            abi: f.sig.abi.clone(),
            fn_token: Fn::default(),
            ident: inner_fn_ident.clone(),
            generics: inner_fn_generics,
            paren_token: Paren::default(),
            inputs: f.sig.inputs.clone(),
            variadic: f.sig.variadic.clone(),
            output: {
                let mut unified_lt_output = f.sig.output.clone();
                transform_sig_output(&mut unified_lt_output, &fut_lifetime);
                unified_lt_output
            },
        };
        LifetimeUnifier.visit_signature_mut(&mut new_fn_sig);

        let mut async_block_stmts = new_fn_sig
            .inputs
            .iter()
            .flat_map(|i| {
                struct IdentCollector(Vec<(Option<Mut>, Ident)>);
                impl Visit<'_> for IdentCollector {
                    fn visit_pat_ident(&mut self, i: &'_ syn::PatIdent) {
                        self.0.push((i.mutability, i.ident.clone()))
                    }

                    fn visit_receiver(&mut self, i: &'_ syn::Receiver) {
                        self.0.push((i.mutability, Ident::from(i.self_token)));
                    }
                }

                let mut collector = IdentCollector(Vec::new());
                collector.visit_fn_arg(i);

                collector.0.into_iter().map(|(mutability, ident)| {
                    Stmt::Local(Local {
                        attrs: Vec::new(),
                        let_token: Let::default(),
                        modifiers: LocalModifiers::default(),
                        pat: Pat::Ident(PatIdent {
                            attrs: Vec::new(),
                            by_ref: None,
                            mutability,
                            ident: ident.clone(),
                            subpat: None,
                        }),
                        init: Some(LocalInit {
                            eq_token: Eq::default(),
                            expr: Box::new(Expr::Path(ExprPath {
                                attrs: Vec::new(),
                                qself: None,
                                path: Path {
                                    leading_colon: None,
                                    segments: Punctuated::from_iter([PathSegment {
                                        ident,
                                        arguments: PathArguments::None,
                                    }]),
                                },
                            })),
                            diverge: None,
                        }),
                        semi_token: Semi::default(),
                    })
                })
            })
            .collect::<Vec<_>>();

        async_block_stmts.extend([orig_type_hint_stmt, local_stmt, return_stmt]);
        make_inputs_not_mut_pats(&mut new_fn_sig.inputs);

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
                        tokens: quote::quote! { clippy::async_yields_async, clippy::diverging_sub_expression },
                    }),
                }],
                async_token: Async::default(),
                capture: Some(Move::default()),
                modifiers: BlockModifiers::default(),
                block: Block {
                    brace_token: Brace::default(),
                    stmts: async_block_stmts,
                },
            })]),
        });

        let mut new_fn_inner = Stmt::Item(Item::Fn(ItemFn {
            attrs: vec![Attribute {
                pound_token: Pound::default(),
                style: syn::AttrStyle::Outer,
                bracket_token: Bracket::default(),
                meta: Meta::List(MetaList {
                    path: ident_to_path(format_ident!("allow")),
                    delimiter: syn::MacroDelimiter::Paren(Paren::default()),
                    tokens: quote::quote! { clippy::type_complexity },
                }),
            }],
            vis: syn::Visibility::Inherited,
            modifiers: FnModifiers::default(),
            sig: new_fn_sig,
            block: Box::new(Block {
                brace_token: Brace::default(),
                stmts: vec![Stmt::Expr(call_expr, None)],
            }),
        }));

        SelfFixerUpper {
            change_self_to_ty: &input.self_ty,
            trait_items: &trait_items,
            trait_name,
        }
        .visit_stmt_mut(&mut new_fn_inner);

        // And then change its return type to what async-trait does
        transform_sig_output(&mut f.sig.output, &async_trait_lt);
        add_lifetime_bounds(&mut f.sig, &async_trait_lt);
        make_inputs_not_mut_pats(&mut f.sig.inputs);

        let mut new_args = Punctuated::new();
        for arg in &f.sig.inputs {
            let ident = match arg {
                FnArg::Receiver(s) => Ident::from(s.self_token),
                FnArg::Typed(t) => match &*t.pat {
                    Pat::Ident(i) => i.ident.clone(),
                    _ => return quote::quote! {
                        compile_error!("all types on a ul_async_trait fn must be bare identifiers; no patterns or destructuring allowed");
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
                qself: None,
                path: Path {
                    leading_colon: None,
                    segments: Punctuated::from_iter([PathSegment {
                        ident: inner_fn_ident.clone(),
                        arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                            colon2_token: Some(PathSep::default()),
                            lt_token: Lt::default(),
                            args: inner_fn_generic_args,
                            gt_token: Gt::default(),
                        }),
                    }]),
                },
            })),
            paren_token: Paren::default(),
            args: new_args,
        };

        f.block.stmts = vec![new_fn_inner, Stmt::Expr(Expr::Call(call_new_fn), None)];
    }

    quote::quote! {
        #input
    }
    .into()
}
