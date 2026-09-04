# Writing this harness's instruction file

Generated from `references/cursor-baseline.json`. Do not edit:
the next render overwrites it, and the baseline is where a correction
belongs.

## Where it goes

`~/.cursor/rules/` -- a **directory**, one file per rule

Decided by: https://cursor.com/docs/rules

## What the record says about it

The product's own rule-creation picker offers two scopes, and the second is this one:
  {value:"user", label:"User Rule", hint:"Applies to all your projects", path: join(homedir(), ".cursor", "rules")}
Declared 2026-08-28. It had been declined as *"No global rules directory exists"*, which was true of the vendor's page and false of the product. `instruction` routes here and nowhere else on this harness -- cursor's CLI has no global instruction *file*, which is why this provider's plugin carries its rule through a manifest.

**And the CLI does not load a plugin's rules, measured 2026-08-29.** This provider's setups used to carry the working floor as a rule inside a local plugin, on the reasoning recorded above -- *a plugin rather than a loose rule file, so what this provider owns is one directory it created rather than a directory the product also writes to.* The reasoning was sound and the mechanism does not exist: in the pinned `2026.08.25-3e8eec8` bundle the listing switch reads `case "rules": t = k(R)` and consults nothing else, while `case "skills"` explicitly merges `getAllEnabledPlugins()`. Searched with that as the control -- `for … of <plugin>.skills` has three real loaders, `for … of <plugin>.rules` has four and **all four are in the bundled JSON-schema validator**, about keyword rules.

So a rule shipped through a plugin manifest reached the model through nothing. The setups write `rules/<name>.mdc` now -- this surface, the one the product's own picker writes to and `k(R)` reads, filtering `.mdc`. `bytes` and not `ran`: no credential-free command of this CLI lists resolved rules, and `about` prints *Not logged in*.

## Where the other harnesses keep theirs

| harness | path | shape |
|---|---|---|
| `antigravity` | `config/rules` | directory |
| `claude` | `CLAUDE.md` | file |
| `codex` | `AGENTS.md` | file |
| **this one** | `rules` | directory |
| `grok` | `AGENTS.md` | file |
| `opencode` | `AGENTS.md` | file |
| `pi` | `AGENTS.md` | file |

**They are not interchangeable, and the difference is not only the
name.** One of the seven takes a *directory* of rules rather than a
single document, so a file moved between the two is not a rename.

**Some products read a neighbour's.** `references/surfaces.md` records
every such cross-read this estate has measured, on the declined rows:
a file written for one product can change what a second one sees, and
removing a setup can change what a third one sees. That is a property
of the products, not of this program, and it is the reason the declined
list is worth reading before writing here.

## Before you write one

- **This file is the floor, not the ceiling.** A repository's own
  instructions sit above it; write what is true everywhere and leave
  the rest to the project.
- **Read it back where the product reads it**, not where the install
  put it. Several of these products resolve a home through an override
  chain, and the two are not always the same directory.

