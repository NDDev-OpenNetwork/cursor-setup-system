# Installation And Lifecycle

Use this reference when changing how a target is installed, observed, restored
or removed.

Every command below is one this program answers. An earlier version of this file
described a different program entirely -- a Python manager from the estate that
came before this one, with verbs this binary has never had and backup slots
numbered `0..9`. Ask the binary rather than this file if the two ever disagree:
`cursor-setup-system` with no arguments prints every command it has.

## The commands

```text
list                                          every setup this build carries
status    --target <dir>                      what a target holds, changing nothing
install   <setup> --target <dir>              write a setup into a target
select    <setup> --target <dir>              reach a different setup's complete state
reinstall --target <dir>                      write the applied setup again
diff      --target <dir>                      what drifted since it was applied
backups   --target <dir>                      the slots, newest first
restore   [--backup <ref>] --target <dir>     the last backup, or a named one
hold      --backup <ref> [--reason <why>] --target <dir>
release   --backup <ref> --target <dir>
remove    --target <dir>                      everything this program owns
adopt     --target <dir>                      take over a target the old estate stamped
software  --prefix <dir>                      which product versions a prefix holds
rollback  --to <version> --prefix <dir>       point the command at one already there
```

There is no `--json` on these. JSON is the *provider* surface -- `provider-info`,
`status --target <dir> --json`, `validate-bundle`, `plan-operation`,
`apply-operation`, `recover-operation` -- and a consumer calls those.

## Invariants

- **The target is named, never guessed.** Absolute, existing, a directory, and
  its final component not a symbolic link. Nothing is inferred from `$HOME`, the
  working directory, or the documented configuration home.
- **A backup is captured before every change**, so `restore` always has
  something to return to. `restore` with no reference means the most recent
  backup that existed when you asked, not the one the restore just took.
- **Selecting a setup reaches its complete state, not a merge.** A file the
  setup you leave owned and the one you choose does not is removed. A target is
  always exactly one setup plus whatever this program never claimed.
- **Retention is bounded and a hold suspends it.** Ten slots rotate; a held slot
  is not reclaimed and is not counted against the bound, so holding one does not
  quietly shorten the window. The last reclaimable slot cannot be held.
- **Software and configuration are separate.** `--prefix` holds the program,
  `--target` holds its configuration, and one prefix can serve many targets.

## Provenance

The artifact table each build compiles in is transcribed from
`references/cursor-baseline.json` by a tool, and a test re-reads that baseline
and compares field by field. Do not copy a version, a URL or a digest into
prose: ask `software --prefix <dir>` what is installed, and read the baseline
for what is published.
