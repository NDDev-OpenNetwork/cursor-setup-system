# Agents And Subagents

Use this reference when writing the agent file itself. **Where an agent reaches
this product from is a plugin fact**, and it is in
`references/authoring-plugins.md` under the manifest's `agents` key.

This page is hand-written because there is no generated one: the generator
writes an `authoring-<kind>.md` only for a kind the baseline routes, and
`references/cursor-baseline.json` **declines** the `agents` directory. Its reason
is measured rather than assumed — `join(this.workspacePath, ".cursor", "agents")`
is workspace-scoped only, unlike `rules`, `commands` and `hooks`, which resolve
under the configuration home as well.

## Where an agent can live

| place | scope | owned by this provider |
| --- | --- | --- |
| `<plugin>/agents/*.md`, named by the manifest's `agents` key | wherever the plugin is installed | **yes**, as part of the plugin |
| `.cursor/agents/*.md` | the workspace only | no |
| `~/.cursor/agents/*.md` | the user, and the CLI does not resolve it | no — declined by name |

So a setup that wants to ship an agent ships a plugin that names one. That is
what this toolkit does: `agents/nddev-builder.md`, declared by `"agents":
"./agents"` in `.cursor-plugin/plugin.json`.

## The file

Markdown with YAML frontmatter, then the prompt body. The vendor's reference
names two fields:

| field | what it is |
| --- | --- |
| `name` | Agent identifier, lowercase kebab-case. |
| `description` | What the agent is for. |

Decided by <https://cursor.com/docs/reference/plugins>.

Cursor accepts further fields on its own agents — model, tool and background
settings among them. Set one only when the behaviour is deliberately owned: a
field a product reads past is decoration, and this estate has paid for the
difference more than once.

## Before you ship one

- Keep each agent narrow enough that automatic routing is predictable.
- Say plainly whether it reviews, creates, checks or releases.
- No credentials, no private repository paths, no generated evidence.
- Point at code-owned files for versions, pins, setup ids and profiles rather
  than restating them here, where nothing would notice them going stale.

## What this page used to say, and why it is worth recording

It said the builder *"projects its agent files under the isolated launch home at
`.nddev-cursor-home/...`"*. **There is no such path anywhere in this estate.** It
describes the shape of the program this one replaced, inherited when this
toolkit was written by hand, and it survived because no generated page ever
superseded it and no check reads English. This provider writes wherever
`--target` names, and nowhere else.

Two sibling pages carried the same class of claim — a *"machine-readable
projection contract owned by `config/nddev-contract.json` and
`build/manifest.json`"*, two files that exist nowhere — and they were removed
when the generated references superseded them.
