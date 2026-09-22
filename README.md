# Unified Local Async Trait

Use this instead of `#[async_trait::async_trait]` for significantly compile times, but it comes with a few restrictions:
1. All associated types must be stated in fully-qualified form (no `Self::Error`, instead `<Self as Trait>::Error`)
2. The `impl` block must be `for` a type that you can define inherent methods on
3. You must have no named lifetimes

Each one of these restrictions might be able to be lifted with some careful testing and validation to ensure that we don't lose the faster compile times of this crate, but for now they are what you must adhere to to obtain the faster compile times.

<img src="https://img.itch.zone/aW1nLzE3NDE1OTczLnBuZw==/original/pEKFnT.png" width="80px" height="64px"></img>
