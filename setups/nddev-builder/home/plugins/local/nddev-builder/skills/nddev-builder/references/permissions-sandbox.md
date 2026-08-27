# Permissions, Sandbox, And Network

Use this reference when changing approval, sandbox, network, or command
permission behavior.

## Native permission rules

Cursor CLI permission arrays contain strings such as:

- `Shell(commandBase)`
- `Read(pathOrGlob)`
- `Write(pathOrGlob)`
- `WebFetch(domainOrPattern)`
- `Mcp(server:tool)`

Deny rules take precedence over allow rules. Keep rule strings narrow and
deterministic. Do not add a broad write, shell, network or MCP permission
without a reason a reader can check.

## Where a posture's permissions actually live

In the setup that carries them, and nowhere else:

- `setups/cursor/baseline/home/cli-config.json` -- a conservative floor
- `setups/cursor/full-auto/home/cli-config.json` -- `approvalMode` unrestricted
  and the sandbox disabled
- `setups/cursor/minimal/` -- no `cli-config.json` at all; the product keeps its
  own defaults

Each setup's `setup.json` records the vendor pages its keys came from. Read the
file; a mapping table repeated here is the copy that goes stale, and this
section was that copy.

## Launch protections

`launch` starts the exact executable a software install placed under
`--prefix`, never a name found on `PATH`, and points the product at `--target`
through the environment variable the product's own documentation names.
Arguments after a bare `--` are passed through verbatim, because `-p`,
`--help` and `--version` mean something to the product and nothing here.

There is no block list. A launch that filtered the product's own arguments
would be this program deciding what the product may be asked, which is not
what it owns.

## Review checklist

- Postures are **setups**, not a second axis. `baseline`, `minimal` and
  `full-auto` are three setups a caller selects between; there is no profile
  dimension crossing them, and adding one would make a target two things at
  once.
- Keep provider secrets out of installer and launch environments.
- Preserve target-owned rollback and backup behavior when profile values change.
- Update tests or validators when a managed config key changes.
