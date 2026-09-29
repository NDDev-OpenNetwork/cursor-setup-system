# What This Harness Owns

Generated from `references/cursor-baseline.json`. Do not edit:
the next render overwrites it, and the baseline is where a correction
belongs.

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
| `plugins/local` | plugin | directory | <https://cursor.com/docs/plugins> | read its bytes |
| `rules` | instruction | directory | <https://cursor.com/docs/rules> | read its bytes |
| `commands` | command | directory | <https://cursor.com/docs/reference/plugins> | read its bytes |
| `hooks.json` | hook | file | <https://cursor.com/docs/hooks> | read its bytes |
| `mcp.json` | mcp | file | <https://cursor.com/docs/mcp> | read its bytes |
| `skills` | skill | directory | measured from the 2026.08.25-3e8eec8 bundle: src/utils/skill-path-utils.ts and the skill-root table in index.js, 2026-08-28 | read its bytes |

**A citation is not a measurement.** `decided by` says where a row came from; `exercised by` says whether anybody made the product demonstrate it. Where a row records no method the answer is a page and nothing else, because absence of a record of measurement is not evidence of measurement.

Here that is **0 run**, **7 read from the product's own bytes**, and **0 resting on a page alone**. The last number is the one worth acting on: a row in it is not wrong, it is untested, and the two are indistinguishable from here.

A surface that routes no kind is owned deliberately: a backup captures
it and a restore returns it, and no component is routed there because
the kind it would carry already routes somewhere else. One kind on two
surfaces makes a consumer's route ambiguous, and the guard in
`harness_runtime::surfaces` refuses it by name.

## A second target: `target_scope: user_root`

Rooted at `~/.agents`, which is **not** this product's configuration
home. A consumer reaches it by naming the scope on the request, and
every path below is relative to that root rather than to the home
above -- writing the root into the path again would nest it twice.

| path | routes | shape | decided by | exercised by |
| --- | --- | --- | --- | --- |
| `skills` | skill | directory | measured from the 2026.08.25-3e8eec8 bundle, digest verified before reading (dist-package/index.js, the skill-root table and its home-rooted mapper), 2026-08-29 | read its bytes |

**Under a scope the namespace is the permission and the recorded
files are the inventory.** A root like this one is read by several
products at once, so `remove`, the capture and a restore all act on
the files this provider recorded writing -- never on the namespace
whole, which would take or revert a neighbour's work.

## A second target: `target_scope: project`

Rooted at `.cursor`, which is **not** this product's configuration
home. A consumer reaches it by naming the scope on the request, and
every path below is relative to that root rather than to the home
above -- writing the root into the path again would nest it twice.

| path | routes | shape | decided by | exercised by |
| --- | --- | --- | --- | --- |
| `.cursor/rules` | instruction | directory | <https://cursor.com/docs/context/rules> | read its bytes |
| `.cursor/commands` | command | directory | <https://cursor.com/docs/cli/reference/slash-commands> | read its bytes |
| `.cursor/hooks.json` | hook | file | <https://cursor.com/docs/hooks> | read its bytes |
| `.cursor/mcp.json` | mcp | file | <https://cursor.com/docs/context/mcp> | read its bytes |
| `.cursor/agents` | agent | directory | <https://cursor.com/docs/subagents> | read its bytes |
| `.cursor/skills` | skill | directory | <https://cursor.com/docs/skills> | read its bytes |

**Under a scope the namespace is the permission and the recorded
files are the inventory.** A root like this one is read by several
products at once, so `remove`, the capture and a restore all act on
the files this provider recorded writing -- never on the namespace
whole, which would take or revert a neighbour's work.

## Considered and not owned

18 rows. Each records what was searched, so the next reader does not repeat the search:

- **`AGENTS.md`** — The CLI reads AGENTS.md at the project root and upward, not from ~/.cursor. Global user rules are set in the application under Customize -> Rules and have no file under the config home; the absence is a standing community request.
- **`agents`** — A plugin manifest key. The directory form `join(this.workspacePath, ".cursor", "agents")` is workspace-scoped only -- unlike `rules`, `commands`, `hooks.json` and `mcp.json`, which all resolve against the home directory as well. measured in the 2026.08.25-3e8eec8 linux/x86_64 bytes (sha256:7a212e5a...), digest verified before reading, and it is the one of the five where the original reason survives the measurement.
- **`hooks`** — **Corrected 2026-09-29: the comparison this ended on no longer holds.** The product resolves `userConfigPath: join(homedir(), ".cursor", "hooks.json")`, and the file form `hooks.json` is owned -- as are `rules` and `commands`, added 2026-08-28. What stays declined is the *directory* `hooks` this row names: ownership is by exact surface, and the directory form the manifest key implies is not a path this provider writes.
- **`NDDEV-CURSOR-PROVIDER.json`** — This provider's own state file: which setup is applied, the identity it recorded, and which slot reverses the last operation. Written by every operation and excluded from target identity, because counting it would leave a target different from the identity the operation just wrote. Not a projection surface and never ownable as one.
- **`.cursor-setup-system`** — This provider's own control directory: the target lock, the backup slots and their payloads. Kept out of the declaration for the same reason as the state file, and recorded here because the declined list is where a reader looks before opening a file to find out what it is.
- **`plugins/cache`** — The product's own plugin cache, a sibling of the owned `plugins/local`. Corrected 2026-09-29: the parent `plugins` is no longer owned -- owning it made `remove` take the product's own siblings, which is the cost that removed the declaration -- so a `remove` leaves this where it is.
- **`plugins/marketplaces`** — Where the product records the marketplaces a person added, sibling to `plugins/local`. Corrected 2026-09-29: the parent `plugins` is no longer owned, so `remove` no longer takes this with it.
- **`plugins/local-marketplaces.json`** — The product's record of locally added marketplaces. Same sibling relationship and, after 2026-09-29, the same standing: the parent is not owned, so `remove` leaves it alone.
- **`plugins/installed_plugins.json`** — The product's own record of what it installed. Corrected 2026-09-29: the parent `plugins` is no longer owned, so `remove` leaves this record -- and the product's knowledge of hand-installed plugins -- alone.
- **`cli-runtime-state`** — One row for the subtree a run leaves behind: `agent-cli-state.json`, `ai-tracking/ai-code-tracking.db`, `cli-workspaces.json`, `ide_state.json`, `plans`, `projects`. The product's own lifetime.
- **`policy.json`** — An enterprise policy file, sibling to `managed/active-team-hooks/hooks.json`. Administrator-pushed and never a setup's to write, for the reason grok's `managed_config.toml` row gives at greater length.
- **`skills-cursor`** — The same skill-root table flags this one `builtin: true` -- it is the product's own shipped skills, not a place a person or a consumer writes. Owning it would put this provider's backup and remove across bytes the product manages for itself. Recorded rather than left absent because the directory is real, sits beside the owned `skills`, and a reader who found it would otherwise have to repeat this search.
- **`cloud-skills`** — Listed as a skill path in the same bundle and filled from the account rather than from disk. Nothing this provider installs belongs there, and a backup of it would capture someone's server-side state under a local name.
- **`cursor-compile-cache`** — Not a path in the target: the product writes `~/.cache/cursor-compile-cache/<node-version>-<hash>-<uid>` **outside its configuration home**, measured 2026-08-28 by running the pinned `2026.08.25-3e8eec8` binary in a clean `HOME`. A bare `--version` was enough to create it.
- **`sandbox.json`** — **A ninth surface under this product's home, measured 2026-08-30 in the 2026.08.25-3e8eec8 linux/x86_64 bytes** (sha256 checked against this baseline's own table before extracting). Two independent joins in the bundle:
- **`plugins`** — Real, and deliberately not owned. This provider's payload lives one level down at `plugins/local`, which is declared. Owning the parent as well made it exact state, and the product writes `plugins/local-marketplaces.json` there itself -- a person's marketplace sources, a sibling of ours rather than a child. Reproduced 2026-08-31 with the released binary: `select nddev-builder` then `select minimal` removed that file, an unrelated file beside it, and the directory. The parent was declared when the plugin kind routed one level up and outlived the move by four releases. The repository's namespace-shape check refuses the shape for all seven now.
- **`plugin.json $schema`** — **A published schema this estate deliberately does not name, and naming it would break the plugin.** Cursor is the only one of the four unschemaed harnesses that publishes a machine-readable manifest schema: `https://agent-plugins.org/schemas/1.0.0/plugin.schema.json`, reachable (200) and referenced once in the pinned artifact.
- **`permissions.json`** — The product's own permission store: `load` reads `{permissions, autoRunInstructions}` from it, i.e. the allow/deny decisions a person made in the UI, remembered across runs. It is state the product writes, not configuration a setup ships -- a replace would erase remembered grants, and the posture's own permissions travel in `cli-config.json`'s `permissions` object.
