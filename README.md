# Unified Local Async Trait

Use this instead of `#[async_trait::async_trait]` for significantly faster compile times, but it comes with a few restrictions:
1. All associated types must be stated in fully-qualified form (no `Self::Error`, instead `<Self as Trait>::Error`)
2. The `impl` block must be `for` a type that you can define inherent methods on
3. You must have no named lifetimes

Each one of these restrictions might be able to be lifted with some careful testing and validation to ensure that we don't lose the faster compile times of this crate, but for now they are what you must adhere to to obtain the faster compile times.

### Wanna know more?

The inspiration from this came from seeing [this PR](https://github.com/apache/datafusion/pull/24325/changes) for Datafusion, which cut compile times by a factor of 6 for a specific crate by refactoring its `async_trait`-generated code in a very automatable way. So I just took the changes that they made and extracted them into a crate.

I don't fully understand why these changes make it easier for the compiler to work with (yes, the original PR has a very simple explanation, but doesn't get into why the caching that it describes happens), and would like to look more into it.

If you'd like to see what the generated code looks like, feel free to take a look at the `*.expanded.rs` files in `tests/expand`.

### Contributing

Please contribute! Note, however, that no AI-generated or AI-assisted or AI-viewed or AI-anything'ed code is allowed in this repo.

<a href="https://samvieten.itch.io/no-ai"><img src="https://img.itch.zone/aW1nLzE3NDE1OTczLnBuZw==/original/pEKFnT.png" width="80px" height="64px"></img></a>
