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

## Using this against a home you already have

**An owned namespace is removed whole.** The table below says what this build
owns; `remove` deletes each of those paths entirely, and a backup slot holds
what was there first. That includes content this build never wrote -- if the
product itself put a key in a configuration file this provider owns, `remove`
takes the file, not the keys this provider added to it.

Measured, with the real product: launching Codex through `launch` and running
`mcp add` writes `~/.codex/config.toml` with an `[mcp_servers.*]` entry; a
later `install` captures that file into a slot and replaces it; a later
`remove` deletes it. The entry is not lost -- `backups` lists the slot as
*before install, setup none*, and restoring it returns the file byte for byte
-- but it is not in the target either.

So: point `--target` at a home you are willing to have managed. `backups
--target <dir>` names every earlier state and which setup each preceded, and
`restore --backup <ref>` returns any of them exactly.

## When conformance says this provider is malformed

`ai-stp provider conformance --protocol-version 3` reports each case by name.
If the one that fails is `provider_info_v3_closed`, with a detail about fields
differing from the closed schema, **check the version of the checker before
suspecting this build**.

The v3 capability schema is compared as an exact field set, so a provider that
declares a field the checker predates is reported as malformed rather than as
newer. `scoped_projection_profiles` (`ADR-0125`) is the field this applies to,
and it is omitted entirely when empty -- so a build that declares no scope
satisfies an older checker by accident, and a build that declares one does not.

Two versions, two different answers, both measured:

| checker | result |
| --- | --- |
| `ai-stp-cli` 0.0.3 | five pass; Codex and Antigravity report `conforms=false`, detail *fields differ from the closed v3 schema* |
| `ai-stp-cli` 0.0.7 | six pass 23 of 23; Codex reports `conforms=false`, detail *a scoped projection profile names an unknown target scope* |
| `ai-stp-cli` 0.0.8 | **all seven pass**, 27 to 29 cases each |

The middle row was never a defect in this build, and the third row is how that
was settled: **it closed with no change on this side.** `0.0.7` carried the
field but its scope enum was `["project"]` alone, while the provider kit this
program vendors and verifies byte-for-byte gave `["project", "user_root"]`. The
kit is the artifact a provider is told to build against, so a build declaring
`user_root` was right by the document it was handed and wrong by the checker
shipped beside it. `0.0.8` shipped the enum, and a declaration that had been
correct for a month started being read as correct.

**Withdrawing a correct declaration to make a lagging instrument print green is
never the answer here.** The three rows above are the argument for that, and
they are also the argument for the rule this section exists for.

Which is the general rule this section exists for: **check the version of the
checker before suspecting this build**, and prefer the newest, because an older
one reports a wider failure than the one it found.

## What `status` reports, and what it does not

`state` answers **who manages this target**, and never *whether a setup is
installed*. Three values, and the distinction matters most for the fourth
situation, which is not a fourth value:

| | |
| --- | --- |
| `missing` | the directory is empty |
| `unmanaged` | it holds content, none of it this provider's |
| `managed` | this provider's state file is present and current |

`missing` used to be looser -- it asked whether this provider owned anything,
so a directory full of another product's files reported `missing`. A consumer
reads this to decide what it is looking at, and being told a populated
directory is empty invites it to treat the place as free. Emptiness is about
the directory, not about us.

**After a `remove`, `state` stays `managed`, and that is the honest answer.**
The setup is gone -- no file a product reads survives it -- but the control
directory and a backup slot remain, and that slot is what makes the removal
reversible: `restore` brings the setup back. A target reported as `missing`
while a restore is pending would be a lie in the direction that costs someone
their data.

Whether a setup is installed is carried by `setup_stable_id`, which is `null`
exactly when none is. That is the field to test, not this word.
`target_identity_digest` corroborates it -- after a remove it is the digest of
an empty tree -- but the field is the direct answer and the digest is not.

## The network, stated exactly

**This artifact does not link the network, and no local phase can spawn
anything that could.** Two lints hold it rather than a promise: `std::net` is
refused outright, and `std::process::Command` is refused everywhere but two
named places -- the `launch` command, which is declared in `provider-info` and
absent from builds that do not declare it, and a lifecycle probe that drives
this binary's own executable. Adding a `tar` shell-out to ordinary code fails
the build with *only `launch` may spawn, and it is declared*. Every crate that
may be linked is named in `deny.toml`, so a transitive dependency cannot arrive
unread.

Those are claims about the source, and a lint can be wrong, bypassed, or simply
disbelieved. So `ci` reads the shipped binary too: a `boundary` job asks the
import table of the artifact this build produces whether any network symbol is
present, and whether a build declaring no `launch` imports anything that could
spawn. You can run it yourself against a downloaded release --
`nm -D --undefined-only <binary>` on Linux, `nm -u` on macOS -- and it needs no
part of this repository to be trusted.

**What that does not buy, said plainly because the stronger claim is the
tempting one.** This is a dynamically linked program: it imports `syscall` from
libc like any other, so no property of the binary can prove a socket is
unreachable to code that is determined to open one. What is proven is narrower
and still worth having: no code path here reaches for the network, none can be
added without the build refusing, and no local phase can hand the job to a
child process. If your threat model needs the guarantee rather than the
absence, run `plan` and `apply` under whatever sandbox you already trust; both
phases are offline by design, and `apply` verifies the digests it was given
with the network gone.

## What this build owns inside a target

Everything else in the target is a sibling overlay and is preserved
verbatim. Each row cites the vendor page it was read from, and the same
table is bound to the declaration by a test, so this cannot drift from
what `provider-info` publishes.

Configuration home as the product documents it: `~/.cursor`.

| Path | Component kinds routed here | Decided by |
| --- | --- | --- |
| `cli-config.json` | `setting` | [source](https://cursor.com/docs/cli/reference/configuration; filename and default object read from the pinned 2026.08.25-3e8eec8 bundle) |
| `plugins` | -- | [source](https://cursor.com/docs/plugins; anchored literal measured in the pinned artifact by scripts/evidence.py) |
| `plugins/local` | `plugin` | [source](https://cursor.com/docs/plugins) |
| `rules` | `instruction` | [source](https://cursor.com/docs/rules; measured in the pinned 2026.08.25-3e8eec8 bundle, digest verified before reading) |
| `commands` | `command` | [source](https://cursor.com/docs/reference/plugins; measured in the pinned 2026.08.25-3e8eec8 bundle, digest verified before reading) |
| `hooks.json` | `hook` | [source](https://cursor.com/docs/hooks; measured in the pinned 2026.08.25-3e8eec8 bundle, digest verified before reading) |
| `mcp.json` | `mcp` | [source](https://cursor.com/docs/mcp; measured in the pinned 2026.08.25-3e8eec8 bundle, digest verified before reading) |
| `skills` | `skill` | [source](measured from the pinned 2026.08.25-3e8eec8 bundle: src/utils/skill-path-utils.ts and the skill-root table in index.js, 2026-08-28) |

A path routing no component kind is owned so a setup can carry it;
nothing compiles a component to it.

### A second target: `target_scope: user_root`

Rooted at `~/.agents`, which is not the configuration home
above. A consumer reaches it by naming the scope on the request, and
every path below is relative to that root.

| Path | Component kinds routed here | Decided by |
| --- | --- | --- |
| `skills` | `skill` | measured from the product's own bytes |

This root is read by several products at once, so under this scope
`remove`, the backup and a restore act on the files this program
recorded writing rather than on the directory whole. A neighbour's
files are never captured into a backup slot here, and never reverted
by a restore.

### Considered and not owned

Everything named here is left exactly as it was found, like any
other file beside a target.

**`AGENTS.md`** -- The CLI reads AGENTS.md at the project root and upward, not from ~/.cursor. Global user rules are set in the application under Customize -> Rules and have no file under the config home; the absence is a standing community request. ([source](https://cursor.com/docs/cli/using))

**`agents`** -- A plugin manifest key. The directory form `join(this.workspacePath, ".cursor", "agents")` is workspace-scoped only -- unlike `rules`, `commands`, `hooks.json` and `mcp.json`, which all resolve against the home directory as well. measured in the pinned 2026.08.25-3e8eec8 linux/x86_64 bytes (sha256:7a212e5a...), digest verified before reading, and it is the one of the five where the original reason survives the measurement.

**The vendor's page says the opposite, and the shipping binary decides.** cursor.com/docs/subagents, read 2026-08-29, lists user-level locations explicitly -- `~/.cursor/agents/` *"All projects for current user"*, beside `~/.claude/agents/` and `~/.codex/agents/` as compatibility paths. Measured against the pinned 2026.08.25-3e8eec8 bytes, which the pin refresher confirms is the current release: `computeAgentsDirs()` resolves `join(resolve(this.workspacePath), ".cursor", "agents")` and nothing else. Every other occurrence of the path is a glob or an ignore rule tagged `{type:"workspace"}`, and a search for a home-joined form returns zero while the positive controls return hits and an invented path returns none.

So the page documents a directory the current product does not read. Recorded at length because the next reader will find that page and take this declaration for an oversight: **a page can promise more than the product does, and declaring on it would have this provider claim a path nothing reads.** Re-measure when the pin moves rather than when the page changes. ([source](https://cursor.com/docs/subagents))

**`hooks`** -- **Corrected 2026-08-28: a user-level file exists.** This row read "a plugin manifest key, not a directory under the config home". The product resolves `userConfigPath: join(homedir(), ".cursor", "hooks.json")`, alongside an enterprise path and the manifest key. Not owned, for the same reason as `rules` and `commands`. Raised. ([source](https://cursor.com/docs/hooks; measured in the pinned 2026.08.25-3e8eec8 linux/x86_64 bytes (sha256:7a212e5a...), digest verified before reading))

**`NDDEV-CURSOR-PROVIDER.json`** -- This provider's own state file: which setup is applied, the identity it recorded, and which slot reverses the last operation. Written by every operation and excluded from target identity, because counting it would leave a target different from the identity the operation just wrote. Not a projection surface and never ownable as one. ([source](this provider's own contract; no vendor page is involved))

**`.cursor-setup-system`** -- This provider's own control directory: the target lock, the backup slots and their payloads. Kept out of the declaration for the same reason as the state file, and recorded here because the declined list is where a reader looks before opening a file to find out what it is. ([source](this provider's own contract; no vendor page is involved))

**`plugins/cache`** -- The product's own plugin cache, a sibling of the owned `plugins/local`. Named in the same joins. It matters because this provider owns the parent `plugins` during the transition window, so a `remove` takes this with it. ([source](measured in the pinned 2026.08.25-3e8eec8 linux/x86_64 bytes (sha256:7a212e5a...), digest verified before reading))

**`plugins/marketplaces`** -- Where the product records the marketplaces a person added, sibling to `plugins/local`. Taken by a `remove` of the owned parent, which is the concrete cost of the transition window. ([source](measured in the pinned 2026.08.25-3e8eec8 linux/x86_64 bytes (sha256:7a212e5a...), digest verified before reading))

**`plugins/local-marketplaces.json`** -- The product's record of locally added marketplaces. Same sibling relationship and the same consequence. ([source](measured in the pinned 2026.08.25-3e8eec8 linux/x86_64 bytes (sha256:7a212e5a...), digest verified before reading))

**`plugins/installed_plugins.json`** -- The product's own record of what it installed. A `remove` of the owned parent takes it, and the product then no longer knows about plugins a person installed by hand. Recoverable from the capture that runs first, and the sharpest single reason the window should close when the consumer's corpus objects naming `plugins` retire. ([source](measured in the pinned 2026.08.25-3e8eec8 linux/x86_64 bytes (sha256:7a212e5a...), digest verified before reading))

**`cli-runtime-state`** -- One row for the subtree a run leaves behind: `agent-cli-state.json`, `ai-tracking/ai-code-tracking.db`, `cli-workspaces.json`, `ide_state.json`, `plans`, `projects`. The product's own lifetime. ([source](measured from the pinned 2026.08.25-3e8eec8 bundle))

**`policy.json`** -- An enterprise policy file, sibling to `managed/active-team-hooks/hooks.json`. Administrator-pushed and never a setup's to write, for the reason grok's `managed_config.toml` row gives at greater length. ([source](measured from the pinned 2026.08.25-3e8eec8 bundle))

**`skills-cursor`** -- The same skill-root table flags this one `builtin: true` -- it is the product's own shipped skills, not a place a person or a consumer writes. Owning it would put this provider's backup and remove across bytes the product manages for itself. Recorded rather than left absent because the directory is real, sits beside the owned `skills`, and a reader who found it would otherwise have to repeat this search. ([source](measured from the pinned 2026.08.25-3e8eec8 bundle's skill-root table, 2026-08-28))

**`cloud-skills`** -- Listed as a skill path in the same bundle and filled from the account rather than from disk. Nothing this provider installs belongs there, and a backup of it would capture someone's server-side state under a local name. ([source](measured from the pinned 2026.08.25-3e8eec8 bundle: src/utils/skill-path-utils.ts, 2026-08-28))

**`cursor-compile-cache`** -- Not a path in the target: the product writes `~/.cache/cursor-compile-cache/<node-version>-<hash>-<uid>` **outside its configuration home**, measured 2026-08-28 by running the pinned `2026.08.25-3e8eec8` binary in a clean `HOME`. A bare `--version` was enough to create it.

It gets a name here rather than a path because every recorded path is relative to the target and this one is not; the `rooted_elsewhere` guard refuses such a row. Recorded so a reader looking for everything the product writes does not stop at `~/.cursor`. ([source](measured by running the pinned 2026.08.25-3e8eec8 binary in a clean HOME, 2026-08-28))

## Response

One maintainer. Defects are triaged as time allows; security reports are
acknowledged first.
