# Validation Before Handing Off

Use this reference before handing off work on a setup system.

## Which repository you are in decides what you can run

This setup ships in two places and the gate below exists in only one of them.
**It belongs to the private authoring monorepo, the source workspace,
which renders this public tree.** A checkout of this public repository carries
`crates/`, `setups/`, `references/` and `scripts/evidence.py` -- and neither
`scripts/gate.sh` nor `tools/`.

That is not a gap to fill. A rendered tree is generated: the fix for anything
here is a change in the authoring repository and a re-render, never an edit to
this checkout. What a reader of *this* tree can run is `cargo fmt --all
--check`, `cargo clippy --workspace --all-targets -- -D warnings` and
`cargo test --workspace`, which is what its own CI runs.

Naming a command a reader cannot run used to be the whole of this page, and the
reader is a model, which will try it and then work around the failure rather
than say so.

## The gate, in the authoring repository

One entry point, from its root:

```bash
scripts/gate.sh
```

It exists rather than four bare `cargo` commands because the workspace pins a
toolchain that a local `cargo` earlier on `PATH` will shadow, and a green run
under the wrong compiler is worse than a red one. **`gate.sh` is the list of
what it checks** -- it names each one as it runs, and this page does not carry a
second copy. This paragraph used to enumerate them, and named five of the seven.

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
