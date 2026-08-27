# Support

## Before opening anything

`--help` states what this build does and does not do. `status --target <dir>
--json` reports what it found in a target without changing it, and its output is
safe to share: it carries identities and digests, never secret values.

## Where to go

| You have | Go to |
| --- | --- |
| A defect | [Issues](../../issues) — use the defect template |
| A question about behaviour | [Issues](../../issues) — a blank issue is fine |
| A vulnerability | [Security advisories](../../security/advisories/new), privately |

Never open a public issue for a vulnerability, and never paste credentials,
tokens, or the contents of a backup slot anywhere in this repository. A backup
slot holds whatever the target held when it was captured.

## What this build does, and what it does not

The software lifecycle — installing, updating and removing the product
itself — is declared and does work. `plan` names the exact bytes offline,
whoever holds the network fetches them, and `apply` verifies and installs
with the network gone.

`launch` is declared. It starts the exact executable a software install
placed under `--prefix`, never a name found on `PATH`, and points the
product at `--target` through the environment variable its own
documentation names.

A provider that advertised an operation it cannot perform would let a caller ask
for something that cannot be honoured, which is worse than not offering it.

All five core operations do work: `backup`, `restore`, `remove`, `install` and
`replace`, both from the local setup catalog and from an `ai-stp-bundle/1`
arriving over the wire.

## What this build owns inside a target

Everything else in the target is a sibling overlay and is preserved
verbatim. Each row cites the vendor page it was read from, and the same
table is bound to the declaration by a test, so this cannot drift from
what `provider-info` publishes.

Configuration home as the product documents it: `~/.cursor`.

| Path | Component kinds routed here | Decided by |
| --- | --- | --- |
| `cli-config.json` | `setting` | [source](https://cursor.com/docs/cli/reference/configuration) |
| `plugins` | `plugin` | [source](https://cursor.com/docs/reference/plugins) |

A path routing no component kind is owned so a setup can carry it;
nothing compiles a component to it.

### Considered and not owned

Everything named here is left exactly as it was found, like any
other file beside a target.

**`AGENTS.md`** -- The CLI reads AGENTS.md at the project root and upward, not from ~/.cursor. Global user rules are set in the application under Customize -> Rules and have no file under the config home; the absence is a standing community request. ([source](https://cursor.com/docs/cli/using))

**`rules`** -- Rules are .mdc files in a project's .cursor/rules, or the rules key of a plugin manifest. No global rules directory exists. ([source](https://cursor.com/docs/rules))

**`skills`** -- A plugin manifest key, not a directory under the config home. ([source](https://cursor.com/docs/skills))

**`agents`** -- A plugin manifest key, not a directory under the config home. ([source](https://cursor.com/docs/subagents))

**`commands`** -- A plugin manifest key, not a directory under the config home. ([source](https://cursor.com/docs/reference/plugins))

**`hooks`** -- A plugin manifest key, not a directory under the config home. ([source](https://cursor.com/docs/hooks))

**`mcp.json`** -- MCP servers are an mcpServers key in configuration or a plugin manifest. No ownable file surface under ~/.cursor, so mcp is not declared. ([source](https://cursor.com/docs/mcp))

## Response

One maintainer. Defects are triaged as time allows; security reports are
acknowledged first.
