#![expect(dead_code, renamed_and_removed_lints)]

#[async_trait::async_trait]
trait UsesAssocTypes {
    type Assoc;

    async fn takes_ty(a: Self::Assoc);
    async fn gives_ty() -> Self::Assoc;
}

struct AssocIsStaticStr;

#[ul_async_trait::async_trait]
impl UsesAssocTypes for AssocIsStaticStr {
    type Assoc = &'static str;

    async fn takes_ty(_a: Self::Assoc) {}
    async fn gives_ty() -> Self::Assoc { "" }
}

struct AssocIsUsize;

#[async_trait::async_trait]
impl UsesAssocTypes for AssocIsUsize {
    type Assoc = usize;

    async fn takes_ty(_a: Self::Assoc) {}
    async fn gives_ty() -> Self::Assoc { 0 }
}

fn main() {}
