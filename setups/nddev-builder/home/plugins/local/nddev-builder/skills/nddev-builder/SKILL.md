---
name: nddev-builder
description: Create, improve or review a complete Cursor CLI setup -- a native collection of tools. Use to select and author components, compose exact setups, explain capabilities, and validate native placement, installation and recovery.
---

# NDDev Builder

Start with `references/ai-stp-lifecycle.md` to turn the user's tasks into a complete setup: inventory components, fill capability gaps, compose, evaluate and deliver a usable tool collection.

Read the native references below for this harness's formats and activation rules. Provider implementation changes use the additional provider checks; ordinary setup authoring needs no Rust checkout.

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
- **Additional roots this harness owns and how a setup coordinates them**: read `references/second-target.md`. Generated from the
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

- Validate with disposable homes, prefixes and targets. Exercise native
  activation only for the components and accounts authorized by the task.
- Publishing or applying to a live target requires that effect in the task;
  authoring alone does not imply it. Use the exact provider lifecycle and
  preserve the running agent's own active configuration.
- A target the frozen estate still stamps is taken over by `adopt`, which is a
  command someone types and never something `install` does behind them. Nothing
  is deleted: the old stamp moves aside and the pre-adoption state is one `mv`
  away, on top of the backup adoption captures first.
