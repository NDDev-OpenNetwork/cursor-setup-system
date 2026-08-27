# Configuration And Postures

Use this reference when creating, changing, or reviewing what a setup writes.

## Native Cursor locations

- Global CLI config: `~/.cursor/cli-config.json`.
- Project CLI config: `<project>/.cursor/cli.json`.
- Manager launch override: `CURSOR_CONFIG_DIR=<absolute-target>` points Cursor
  Agent at the target-local `cli-config.json` for the child process only.

Do not claim that Cursor discovers `AGENTS.md` through `CURSOR_CONFIG_DIR`.
Workspace instructions are covered in `skills-instructions.md`.

## Public module model

- A setup is a directory: `setups/<id>/setup.json` beside `setups/<id>/home/`,
  copied verbatim into the target.
- `setup.json` carries the identity, the description, and the vendor pages that
  decided the format of any configuration file the setup writes.
- There are no profiles. Three setups -- `baseline`, `minimal`, `full-auto` --
  are the postures, and every setup system offers the same three.
- This program writes `NDDEV-CURSOR-PROVIDER.json` into a target it manages.
  `NDDEV-CURSOR-CLI-SETUP.json` is the *predecessor* stamp, written by the
  frozen estate, and `adopt` is what takes such a target over.

Ask `list` for the setup ids rather than restating them anywhere.

## Config shape

`cli-config.json` is a JSON object of native Cursor CLI fields. Every one below
is documented by Cursor, and a setup that writes any of them records the page in
its `sources`:

- `version`
- `editor.vimMode`
- `permissions.allow`
- `permissions.deny`
- `approvalMode`
- `sandbox.mode`
- `sandbox.networkAccess`
- `network.useHttp1ForAgent`
- `hints`
- `notifications`

What each posture sets is in its own `setup.json` and the file beside it. Read
those rather than a mapping table here: a second copy of a setup's content is
the copy that goes stale, and this file has been that copy before.

## Review checklist

- Preserve every entry this program does not own, through install, select and
  remove. A sibling overlay is never touched.
- An unmanaged target holding Cursor state is **not** refused: the state is
  captured into a backup slot first, and `restore` returns it exactly. Refusing
  would leave someone unable to configure their own directory; capturing is what
  makes the change reversible instead.
- A target that is a *neighbouring product's* home is a different question and
  is refused by name -- see the `foreign_homes` declaration, which is empty for
  Cursor because no near neighbour has been measured.
- A target the frozen estate stamped is readable by `status` and taken over by
  `adopt`; nothing here migrates it behind the person who owns it.
