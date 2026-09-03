# Validation Before Handing Off

Use this reference before handing off work on a setup system.

Run the checks this tree's CI runs, in order, and report what each one said
rather than that it passed.

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

If a command here is not present, say so rather than working around it.

## A lifecycle smoke test against a disposable target

Never against a live configuration home. A temporary directory outside the
repository:

```bash
target="$(mktemp -d)/cursor-target"
mkdir -p "$target"
cursor-setup-system install baseline    --target "$target"
cursor-setup-system status              --target "$target"
cursor-setup-system select full-auto    --target "$target"
cursor-setup-system diff                --target "$target"
cursor-setup-system backups             --target "$target"
cursor-setup-system restore             --target "$target"
cursor-setup-system remove              --target "$target"
```

The same sequence runs as a test in every published tree, on ubuntu, macos and
windows, against the binary that tree builds -- so a change that breaks it fails
before anyone types it.

## Conformance against the consumer

The wire surface is checked by the consumer's own runner, not by anything here:

```bash
ai-stp provider conformance --harness cursor \
  --executable target/release/cursor-setup-system \
  --target <empty-dir> --protocol-version 3 --json
```

An empty target and a populated one are different questions. A defect that only
appears against a real home is the kind this project has already shipped.

## What this toolkit does not do

- It does not push, tag, or release.
- It does not write a live configuration home.
- It does not install software or start a product.
