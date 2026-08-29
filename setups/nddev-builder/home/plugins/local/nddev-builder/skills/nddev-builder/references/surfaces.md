# What This Harness Owns

Generated from `references/cursor-baseline.json` by
`tools/build_nddev_builder.py`. Do not edit: the next render overwrites
it, and the baseline is where a correction belongs.

Every row below was decided by a source, and the source is named. Where
this file and the binary disagree, the binary is right -- ask it with
`cursor-setup-system provider-info`.

**Configuration home**: `~/.cursor`
**Environment override**: `CURSOR_CONFIG_DIR`
**And**: XDG_CONFIG_HOME moves cli-config.json to $XDG_CONFIG_HOME/cursor and moves nothing else this build owns

## The configuration file

`cli-config.json` is **json**, and the parser does not accept comments.

Strict JSON, and the vendor's own configuration reference says so explicitly: *"JSON (no comments allowed)"*. No schema published for this file -- SchemaStore carries `cursor-sandbox.json` and nothing for `cli-config.json`, searched 2026-08-28.

## Owned surfaces

| path | kinds | shape | decided by | exercised by |
|---|---|---|---|---|
| `cli-config.json` | setting | file | <https://cursor.com/docs/cli/reference/configuration> | read its bytes |
| `plugins` | *(routes no kind)* | directory | <https://cursor.com/docs/plugins> | read its bytes |
| `plugins/local` | plugin | directory | <https://cursor.com/docs/plugins> | *nothing — a page* |
| `rules` | instruction | directory | <https://cursor.com/docs/rules> | read its bytes |
| `commands` | command | directory | <https://cursor.com/docs/reference/plugins> | read its bytes |
| `hooks.json` | hook | file | <https://cursor.com/docs/hooks> | read its bytes |
| `mcp.json` | mcp | file | <https://cursor.com/docs/mcp> | read its bytes |
| `skills` | skill | directory | measured from the pinned 2026.08.25-3e8eec8 bundle: src/utils/skill-path-utils.ts and the skill-root table in index.js, 2026-08-28 | read its bytes |

**A citation is not a measurement.** `decided by` says where a row came from; `exercised by` says whether anybody made the product demonstrate it. Where a row records no method the answer is a page and nothing else, because absence of a record of measurement is not evidence of measurement.

Here that is **0 run**, **7 read from the product's own bytes**, and **1 resting on a page alone**. The last number is the one worth acting on: a row in it is not wrong, it is untested, and the two are indistinguishable from here.

A surface that routes no kind is owned deliberately: a backup captures
it and a restore returns it, and no component is routed there because
the kind it would carry already routes somewhere else. One kind on two
surfaces makes a consumer's route ambiguous, and the guard in
`harness_runtime::surfaces` refuses it by name.

## Considered and not owned

14 rows. Each records what was searched, so the next reader does not repeat the search:

- **`AGENTS.md`** — The CLI reads AGENTS.md at the project root and upward, not from ~/.cursor. Global user rules are set in the application under Customize -> Rules and have no file under the config home; the absence is a standing community request.
- **`agents`** — A plugin manifest key. The directory form `join(this.workspacePath, ".cursor", "agents")` is workspace-scoped only -- unlike `rules`, `commands`, `hooks.json` and `mcp.json`, which all resolve against the home directory as well. measured in the pinned 2026.08.25-3e8eec8 linux/x86_64 bytes (sha256:7a212e5a...), digest verified before reading, and it is the one of the five where the original reason survives the measurement.
- **`hooks`** — **Corrected 2026-08-28: a user-level file exists.** This row read "a plugin manifest key, not a directory under the config home". The product resolves `userConfigPath: join(homedir(), ".cursor", "hooks.json")`, alongside an enterprise path and the manifest key. Not owned, for the same reason as `rules` and `commands`. Raised.
- **`NDDEV-CURSOR-PROVIDER.json`** — This provider's own state file: which setup is applied, the identity it recorded, and which slot reverses the last operation. Written by every operation and excluded from target identity, because counting it would leave a target different from the identity the operation just wrote. Not a projection surface and never ownable as one.
- **`.cursor-setup-system`** — This provider's own control directory: the target lock, the backup slots and their payloads. Kept out of the declaration for the same reason as the state file, and recorded here because the declined list is where a reader looks before opening a file to find out what it is.
- **`plugins/cache`** — The product's own plugin cache, a sibling of the owned `plugins/local`. Named in the same joins. It matters because this provider owns the parent `plugins` during the transition window, so a `remove` takes this with it.
- **`plugins/marketplaces`** — Where the product records the marketplaces a person added, sibling to `plugins/local`. Taken by a `remove` of the owned parent, which is the concrete cost of the transition window.
- **`plugins/local-marketplaces.json`** — The product's record of locally added marketplaces. Same sibling relationship and the same consequence.
- **`plugins/installed_plugins.json`** — The product's own record of what it installed. A `remove` of the owned parent takes it, and the product then no longer knows about plugins a person installed by hand. Recoverable from the capture that runs first, and the sharpest single reason the window should close when the consumer's corpus objects naming `plugins` retire.
- **`cli-runtime-state`** — One row for the subtree a run leaves behind: `agent-cli-state.json`, `ai-tracking/ai-code-tracking.db`, `cli-workspaces.json`, `ide_state.json`, `plans`, `projects`. The product's own lifetime.
- **`policy.json`** — An enterprise policy file, sibling to `managed/active-team-hooks/hooks.json`. Administrator-pushed and never a setup's to write, for the reason grok's `managed_config.toml` row gives at greater length.
- **`skills-cursor`** — The same skill-root table flags this one `builtin: true` -- it is the product's own shipped skills, not a place a person or a consumer writes. Owning it would put this provider's backup and remove across bytes the product manages for itself. Recorded rather than left absent because the directory is real, sits beside the owned `skills`, and a reader who found it would otherwise have to repeat this search.
- **`cloud-skills`** — Listed as a skill path in the same bundle and filled from the account rather than from disk. Nothing this provider installs belongs there, and a backup of it would capture someone's server-side state under a local name.
- **`cursor-compile-cache`** — Not a path in the target: the product writes `~/.cache/cursor-compile-cache/<node-version>-<hash>-<uid>` **outside its configuration home**, measured 2026-08-28 by running the pinned `2026.08.25-3e8eec8` binary in a clean `HOME`. A bare `--version` was enough to create it.
