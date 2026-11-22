# How to run

The compile-fail tests use [trybuild](https://docs.rs/trybuild). To run them, use the following
command on the root of the project:

```
cargo test --package cortex-m-rt --test compile_fail --features device
```

Each `tests/compile-fail/*.rs` file must fail to compile with exactly the diagnostics in the
`.stderr` file next to it. After adding a test or intentionally changing a diagnostic, regenerate
the `.stderr` files with:

```
TRYBUILD=overwrite cargo test --package cortex-m-rt --test compile_fail --features device
```

and review the result before committing. The expected output is tied to the current stable
compiler, so new Rust releases may require regenerating it.
