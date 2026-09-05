---
name: nddev-builder
description: Build, review, or validate a Cursor CLI setup: permissions, lifecycle, plugins, rules, skills, agents, commands, MCP and release surfaces. Use when changing or checking cursor-setup-system behaviour or the native Cursor artifacts a setup carries.
---

# NDDev Builder

Use this skill as the entry point for `cursor-setup-system` work. Keep edits
target-explicit, reversible, and backed by this tree's checks.

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
5. Run this tree's CI checks before handing off -- see
   `references/validation-release.md` for the exact commands.

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
- **Writing a skill**: read `references/authoring-skills.md`. Generated from the
  vendor's own reference, with the field table and what each field does on the
  six harnesses next door.
- **Writing a command**: read `references/authoring-commands.md`. Also
  generated.
- **Writing a hook**: read `references/authoring-hooks.md`, which carries the
  full event list.
- **Writing the plugin manifest**: read `references/authoring-plugins.md`. This
  is also where **agents** reach the product, through the manifest's `agents`
  key -- the directory under the configuration home is workspace-scoped only.
- **Writing an MCP server entry**: read `references/authoring-mcp.md`.
- **Agents and subagents**, for what an agent file itself looks like: read
  `references/agents-subagents.md`.
- **Official install artifact, target-owned runtime, launch, migration, restore,
  and removal**: read `references/installation-lifecycle.md`.
- **The ai-stp CLI lifecycle: scaffold, compose, install, release, publish**:
  read `references/ai-stp-lifecycle.md`.
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
