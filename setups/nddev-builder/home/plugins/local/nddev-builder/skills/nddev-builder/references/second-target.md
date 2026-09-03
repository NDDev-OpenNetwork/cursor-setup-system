# The second target this harness owns

## `target_scope: user_root`, rooted at `~/.agents`

**`~/.agents` is not this product's configuration home.** It is a
different target, reached by a consumer naming the scope on the
request, and every path below is relative to that root rather than
to the home -- writing the root into the path again would nest it
twice, which is a mistake this estate has made and shipped.

| path | routes | decided by | exercised by |
|---|---|---|---|
| `skills` | skill | measured from the 2026.08.25-3e8eec8 bundle | read its bytes |
### `skills`, as measured

The bundle carries a table of skill roots -- `{configDir: ".agents", subdir: "skills", thirdParty: false}` beside `.cursor`, `.claude`, `.codex` and `.grok` -- and a mapper `Gr(t).map(t => ({dirPath: join(e, t.configDir, t.subdir), scope: "user"}))` that the skill scan calls with `homedir()`. `.agents` is *not* marked third-party, so it is scanned whether or not third-party roots are enabled.

**This row did not exist, and the baseline that measured the fact is the one that did not record it**: the `skills` row's own note already quoted the table. Found with a control -- an invented root searched for in the same bytes and absent.

**`bytes` and not `ran`, and what was tried.** The pinned bundle was extracted and every credential-free subcommand of `cursor-agent` exercised: `about`, `status`, `models`, `mcp` and `plugin`. None reports the resolved skill set, and the ones that would need an account -- `about` prints *User Email: Not logged in*. The read stands on the product's own bytes, which is the strongest evidence available here without a credential.

The one root in this estate that belongs to a convention rather than to a product. `$HOME/.agents/skills` is a *sibling* of this product's configuration home, not a child, so nothing declared against this provider's own target can reach it -- that is what `user_root` exists for.

**Owning a shared root, and the reason this record used to decline it.** Five of the seven products read this root, and the decline said: *a namespace is removed whole, so a second declaration would make either provider's remove take the other's skills.* That sentence was true when it was written and stopped being true when `written_paths` shipped -- `remove` under this scope takes the files this provider recorded writing and refuses rather than widening when it cannot read the record, and each harness carries its own state file, so they coexist under one root. The reason was not re-read when the thing it described changed.

Relative to this scope's own root the path is `skills`, not `.agents/skills`: the root is what the scope names, and writing it into the path again would put the skills at `~/.agents/.agents/skills`.


**A setup cannot carry one of these.** A setup is installed into one
target and its payload is relative to that target, so a component
for this scope is installed by the consumer against that root -- not
by a setup aimed at the configuration home. If you are looking for
where to put one by hand, it is the path above joined to the root
above, and nowhere under the home.

**The root is shared, and that changes what removal means.** Several
products read it. Under this scope `remove`, the backup and a
restore act on the files this provider recorded writing rather than
on the directory whole, so a neighbour's files are never captured
into a slot here and never reverted out of one.

## `target_scope: project`, rooted at `.cursor`

**`.cursor` is not this product's configuration home.** It is a
different target, reached by a consumer naming the scope on the
request, and every path below is relative to that root rather than
to the home -- writing the root into the path again would nest it
twice, which is a mistake this estate has made and shipped.

| path | routes | decided by | exercised by |
|---|---|---|---|
| `.cursor/rules` | instruction | <https://cursor.com/docs/context/rules> | read its bytes |
| `.cursor/commands` | command | <https://cursor.com/docs/cli/reference/slash-commands> | read its bytes |
| `.cursor/hooks.json` | hook | <https://cursor.com/docs/hooks> | read its bytes |
| `.cursor/mcp.json` | mcp | <https://cursor.com/docs/context/mcp> | read its bytes |
| `.cursor/agents` | agent | <https://cursor.com/docs/subagents> | read its bytes |
| `.cursor/skills` | skill | <https://cursor.com/docs/skills> | read its bytes |
### `.cursor/rules`, as measured

`LocalCursorRulesService.loadRulesFromDirAndAncestors` joins `.cursor/rules` to the workspace path and walks up through every ancestor; `loadRulesFromDirectory` reads `**/*.mdc` under it; `computeFoldersToWatch` adds the same join for each workspace path. measured 2026-09-02 in the 2026.08.31-4057e58 linux/x64 bundle (sha256:7e306db5..., digest verified against the artifact table before reading), every one of the package's 137 JavaScript members and not only `index.js`. The home surface is the separate *User Rule* scope (`join(homedir(), ".cursor", "rules")` in the rule-creation UI), already owned by the global profile.

### `.cursor/commands`, as measured

`loadCommandsFromDirectory(join(e, ".cursor", "commands"), "workspace")` for each workspace path, in the same loader that then reads the home directory as `"user"` (member `7569.index.js`). measured 2026-09-02 in the 2026.08.31-4057e58 linux/x64 bundle (sha256:7e306db5..., digest verified against the artifact table before reading), every one of the package's 137 JavaScript members and not only `index.js`. The first read of this build looked only in `index.js`, found no `commands` join at all, and would have declined a surface the product reads at both scopes: the loader lives in another member. A measurement of one file stated as a fact about the package.

### `.cursor/hooks.json`, as measured

The hooks path table (member `190.index.js`) names four tiers: `enterpriseConfigPath` (`/etc/cursor/hooks.json`, `/Library/Application Support/Cursor/hooks.json`, `C:\\ProgramData\\Cursor\\hooks.json`), a team file under `.cursor/managed/active-team-hooks`, `userConfigPath: join(homedir(), ".cursor", "hooks.json")` and `projectConfigPath: join(e, ".cursor", "hooks.json")`. This row is the fourth; the third is the global profile's. measured 2026-09-02 in the 2026.08.31-4057e58 linux/x64 bundle (sha256:7e306db5..., digest verified against the artifact table before reading), every one of the package's 137 JavaScript members and not only `index.js`. Re-cited 2026-09-02: the CLI reference page for hooks answered 404 within hours of being cited (control_sweep's citations check caught it); the page the global `hooks.json` row cites, `cursor.com/docs/hooks`, answers 200 and describes the same four tiers.

### `.cursor/mcp.json`, as measured

`join(projectRoot, ".cursor", "mcp.json")` is read beside `join(homedir(), ".cursor", "mcp.json")` wherever MCP servers are loaded -- the loader's own source table is `[{source: "project", ...}, {source: "user", ...}]` (member `9185.index.js`), and the CLI's own help text says *"configured in .cursor/mcp.json or ~/.cursor/mcp.json"*. measured 2026-09-02 in the 2026.08.31-4057e58 linux/x64 bundle (sha256:7e306db5..., digest verified against the artifact table before reading), every one of the package's 137 JavaScript members and not only `index.js`.

### `.cursor/agents`, as measured

`computeAgentsDirs()` resolves `join(resolve(this.workspacePath), ".cursor", "agents")` and, with third-party extensibility on, `join(workspace, ".claude", "agents")`; the loader accepts `.md`, `.mdc` and `.markdown` and parses each file's front matter into a subagent. measured 2026-09-02 in the 2026.08.31-4057e58 linux/x64 bundle (sha256:7e306db5..., digest verified against the artifact table before reading), every one of the package's 137 JavaScript members and not only `index.js`. This is the surface the consumer's cursor#94 measured with globs and asked for at the home; the globs are workspace-index rules and the join is workspace-only, so the home answer stays declined and the workspace gets its scope.

### `.cursor/skills`, as measured

The skill-root table's first row `{configDir: ".cursor", subdir: "skills", thirdParty: false}` is joined to each workspace path by `Hr(e, t)` with `scope: "project", source: "workspace"`, the same table the home read maps through `Yr` with `scope: "user"`. measured 2026-09-02 in the 2026.08.31-4057e58 linux/x64 bundle (sha256:7e306db5..., digest verified against the artifact table before reading), every one of the package's 137 JavaScript members and not only `index.js`.


**A setup cannot carry one of these.** A setup is installed into one
target and its payload is relative to that target, so a component
for this scope is installed by the consumer against that root -- not
by a setup aimed at the configuration home. If you are looking for
where to put one by hand, it is the path above joined to the root
above, and nowhere under the home.

**The root is shared, and that changes what removal means.** Several
products read it. Under this scope `remove`, the backup and a
restore act on the files this provider recorded writing rather than
on the directory whole, so a neighbour's files are never captured
into a slot here and never reverted out of one.

