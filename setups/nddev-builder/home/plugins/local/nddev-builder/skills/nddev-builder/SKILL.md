---
name: nddev-builder
description: Build, review, or validate a Cursor CLI setup: permissions, lifecycle, plugins, rules, skills, agents, commands, MCP and release surfaces. Use when changing or checking cursor-setup-system behaviour or the native Cursor artifacts a setup carries.
---

# NDDev Builder

Use this skill as the entry point for `cursor-setup-system` work. Keep edits
target-explicit, reversible, and backed by the repository's own gate.

## Workflow

1. Identify the native surface being changed.
2. Read only the routed reference files below that match the work.
3. Prefer what the program answers over a prose copy of it. Ask the installed
   binary: `list`, `status --target <dir>`, `software --prefix <dir>`. In a
   checkout, read the harness facts in `crates/<tool>-setup-system/src/main.rs`
   and the baseline they are bound to. There is no `--json` on the human
   commands; JSON is the provider surface a consumer calls.
4. Keep versions, artifact pins and setup ids machine-owned. Ask the binary or
   read the baseline; a list restated in prose is the copy that goes stale, and
   several sections of this toolkit have been exactly that.
5. Run `scripts/gate.sh` before handing off -- see
   `references/validation-release.md` for what it covers and why it exists
   rather than four bare `cargo` commands.

## Routing

- **What this harness owns, what it declines, and what decided each row**:
  read `references/surfaces.md`. It is generated from the baseline, so it
  is the one file here that cannot go stale against the declaration.
- **The configuration file itself -- its grammar, whether comments parse, and
  whether what you write is the effective value**: read
  `references/authoring-settings.md`. Generated from the baseline, and the
  cross-harness half of it is the part no vendor page carries.
- **The instruction file, and which products read a neighbour's**: read
  `references/authoring-instructions.md`. Also generated.
- **The second target this harness owns, and why a setup cannot carry a
  component for it**: read `references/second-target.md`. Generated from the
  baseline's scoped block.
- **Configuration and setup/profile model**: read
  `references/configuration-profiles.md`.
- **Permissions, approval, sandbox, and network policy**: read
  `references/permissions-sandbox.md`.
- **Agents and subagents**: read `references/agents-subagents.md`.
- **Skills, rules, instructions, and AGENTS.md behavior**: read
  `references/skills-instructions.md`.
- **Plugins, local installation, commands, and marketplace boundary**: read
  `references/plugins-marketplace.md`.
- **Hooks**: read `references/hooks.md`.
- **MCP**: read `references/mcp.md`.
- **Official install artifact, target-owned runtime, launch, migration, restore,
  and removal**: read `references/installation-lifecycle.md`.
- **Creator/checker/release validation workflow**: read
  `references/validation-release.md`.

## Boundaries

- Do not write private harness artifacts or live user configuration from this
  public toolkit.
- Do not install software, start MCP servers, activate hooks, approve MCPs, push,
  tag, or mutate team marketplace state.
- A target the frozen estate still stamps is taken over by `adopt`, which is a
  command someone types and never something `install` does behind them. Nothing
  is deleted: the old stamp moves aside and the pre-adoption state is one `mv`
  away, on top of the backup adoption captures first.
