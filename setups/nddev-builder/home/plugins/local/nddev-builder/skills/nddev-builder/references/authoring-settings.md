# Writing this harness's configuration

Generated from `references/cursor-baseline.json`. Do not edit:
the next render overwrites it, and the baseline is where a correction
belongs.

## The file

| | |
|---|---|
| path | `~/.cursor/cli-config.json` |
| grammar | **json** |
| comments | **do not parse** |
| home moved by | `CURSOR_CONFIG_DIR` |

Strict JSON, and the vendor's own configuration reference says so explicitly: *"JSON (no comments allowed)"*. No schema published for this file -- SchemaStore carries `cursor-sandbox.json` and nothing for `cli-config.json`, searched 2026-08-28.

## The home is not one path

XDG_CONFIG_HOME moves cli-config.json to $XDG_CONFIG_HOME/cursor and moves nothing else this build owns

A person reading a single path in a document and pointing `--target`
at it is the failure this note exists to prevent.

## The same question on the other harnesses

| harness | file | grammar | comments |
|---|---|---|---|
| `antigravity` | `antigravity-cli/settings.json` | json | no |
| `claude` | `settings.json` | json | no |
| `codex` | `config.toml` | toml | yes |
| **this one** | `cli-config.json` | json | no |
| `grok` | `config.toml` | toml | yes |
| `opencode` | `opencode.json` | jsonc | yes |
| `pi` | `settings.json` | json | no |

**A comment is not a stylistic choice.** In a strict-JSON file a `//` is
a parse error, and the product does not start rather than starting
without your setting. Two of the seven take comments; the rest do not,
and one of those takes them at two spellings of the same file.

## Before you write one

- **Ask what the product resolved, not what the file says.** Write the
  key, start the product, and read its own answer back. A key the
  product does not know is usually accepted in silence -- which reads
  as configured and does nothing.
- **Put an invented key beside yours.** If the product complains about
  neither, the run discriminates nothing and *the key survived* says
  nothing at all. That control is what separates a file that is parsed
  from a file that is merely read.
- **A value here may not be the effective one.** Where an administrator
  layer exists it clamps everything below it, so a setup can install,
  verify and restore cleanly on a managed machine and change nothing.
  `references/surfaces.md` records which layers this product has and
  what was searched for the ones it does not.

