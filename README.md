# xoon-proto-gen-rust

Generate Rust code from [xoon-proto](https://github.com/pyama2000/xoon-proto)

## Prerequisites

- Rust 1.62.0

## Setup

```shell
# Clone with submodule
git clone --recursive git@github.com:pyama2000/xoon-proto-gen-rust.git

# Apply Git hooks
git config core.hooksPath ./scripts/githooks
```

## Release flow

1. Update [xoon-proto](https://github.com/pyama2000/xoon-proto) when needed
2. Fix build.rs when needed
3. Update crate version to `X.Y.Z`
4. Create release pull request with branch name `release/X.Y.Z`
5. Merge release pull request and tag `X.Y.Z`
