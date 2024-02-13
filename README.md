# cargo-deduplicate-warnings

This crate is a binary which deduplicates warning messages in the json output of
`cargo`.

Normally, cargo does this for you when using its normal output. However, when
you have other tools that consume cargo's json output (e.g. `clippy-sarif`)
then the duplicate warnings show up downstream. This crate allows you to
avoid the issue by stripping them out.

## Usage
```bash
cargo clippy --message-format json | cargo deduplicate-warnings | ...
```
