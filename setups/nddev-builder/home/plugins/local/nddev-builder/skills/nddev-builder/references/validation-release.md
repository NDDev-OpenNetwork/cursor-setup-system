# Validation Before Handing Off

Use this reference before handing off work on a setup system.

## The gate

One entry point, run from the repository root:

```bash
scripts/gate.sh
```

It runs the artifact-table transcription check, the estate readme check,
`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D
warnings` and `cargo test --workspace`. It exists rather than four bare `cargo`
commands because the workspace pins a toolchain that a local `cargo` earlier on
`PATH` will shadow, and a green run under the wrong compiler is worse than a red
one.

```bash
scripts/gate.sh --render
```

also proves the published trees are what this source renders.

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
