//! The Cursor CLI setup system.
//!
//! This file is the harness's *facts*. Every command over them lives in
//! [`harness_runtime`], shared with every other setup system, so a change to
//! behaviour lands once and a change to Cursor CLI's surface lands here.
//!
//! The owner assigned this harness the program lifecycle as well, and it is
//! declared. Cursor is the one product with a genuinely large artifact: 569
//! entries including 127 directories and a bundled `node`, carried in a GNU tar
//! whose long-name headers no other vendor uses.
//!
//! `src/software.rs` is the list, and this paragraph deliberately does not
//! restate it. It said *"the four platforms it publishes"* and *"Windows is
//! not among them"* until `0b9207e`, when the vendor shipped Windows in a
//! different container: there are six now, and the two Windows members are
//! `Shape::Zip` beside four `Shape::GzipTar`. A vendor's platform list is true
//! of the day it was read, so the generated table is the only place it is
//! written down.

use std::process::ExitCode;

mod software;

use harness_runtime::{Harness, LaunchBinding, Scoped};
use provider_v3::{ComponentKind, ProjectionKind, TargetScope};

/// Everything specific to Cursor CLI, verified against `cursor-baseline.json`.
pub const CURSOR: Harness = Harness {
    harness_id: "cursor",
    provider_id: "cursor-setup-system",
    version: env!("CARGO_PKG_VERSION"),
    product: "Cursor CLI",
    vendor: "Anysphere",
    documented_config_home: "~/.cursor",
    config_home_env: "CURSOR_CONFIG_DIR",
    // **Partial, and this is the one the inference got wrong.** This baseline's
    // own note has said since 2026-08-28 that `cli-config.json` is *"one of the
    // eight this build owns"* that follows the variable: `rules`, `commands`,
    // `hooks.json`, `mcp.json` and the plugin pair are built from a literal
    // join to the process home in `cursor-config/dist/paths.js` and reach no
    // resolver at all. The declaration said launch anyway, because the rule
    // asked whether a variable existed rather than what it moved.
    //
    // A launch here assembled a session from the caller's own rules, hooks and
    // MCP servers and the target's settings file -- executable surfaces the
    // selected setup never carried.
    launch_binding: LaunchBinding::Partial {
        unbound: "rules, commands, hooks.json, mcp.json, plugins/local and skills, which \
                  the product joins to the process home rather than to this variable",
    },
    // Not measured. The two artifacts this estate has read for this question are
    // claude's, which carries `DISABLE_UPDATES`, and codex's, which carries no
    // such literal. This product has been asked nothing, and an empty value here
    // says the launch environment is untouched rather than that the product
    // leaves the bytes alone.
    updates_off_env: "",
    // Measured at the line in the pinned bundle, and the first version of
    // this note was wrong in a way worth recording.
    //
    // `cursor-config/dist/paths.js` exports two roots. The config root
    // reads `CURSOR_CONFIG_DIR`, then `XDG_CONFIG_HOME` joined with
    // `cursor` -- not `.cursor` -- then falls back to the home. The data
    // root reads `CURSOR_DATA_DIR` and is not XDG-aware.
    //
    // **Of the eight namespaces this build owns, exactly one goes through
    // either.** `cli-config.json` is `join(configRoot(), "cli-config.json")`.
    // `commands`, `rules`, `hooks.json`, `mcp.json` and the `plugins` pair
    // are built from a literal `join(homedir(), ".cursor", ...)` and go
    // through neither resolver, so XDG does not move them. What the config
    // root does carry is `acp-config.json`, `acp-sessions`, `chats`,
    // `permissions.json` and `statsig-cache.json` -- none of them ours.
    //
    // The first note said the product reads `$XDG_CONFIG_HOME/cursor`
    // without qualification. True of the resolver, false of seven of the
    // eight paths this provider writes: a measurement of one thing stated
    // as a fact about another.
    config_home_note: "XDG_CONFIG_HOME moves cli-config.json to $XDG_CONFIG_HOME/cursor and moves nothing else this build owns",
    control_directory: ".cursor-setup-system",
    state_file: "NDDEV-CURSOR-PROVIDER.json",
    predecessor_state_file: "NDDEV-CURSOR-CLI-SETUP.json",
    profile_id: "cursor/native-and-plugins/1",
    // Everything outside this list is a sibling overlay preserved verbatim.
    // This list was eight and is three, and the six that went were the largest
    // untrue statement in the estate. Measured 2026-08-27 against Cursor's own
    // documentation: the CLI names `cli-config.json` under this home; rules are
    // `.mdc` files in a project's `.cursor/rules`, or the `rules` key of a plugin
    // manifest; and skills, agents, commands and hooks are *plugin manifest keys*
    // -- which `references/cursor-baseline.json` had already recorded, one field
    // away from the declaration that contradicted it.
    //
    // `AGENTS.md` went with them. The CLI reads an `AGENTS.md` at a project root
    // and upward, never from `~/.cursor`; global user rules are set in the
    // application and have no file at all. So the setups here carry their
    // instructions the way the product actually reads one: as a plugin whose
    // manifest names its `rules`.
    //
    // `plugins/local` is where the product reads a local plugin from, and where
    // those setups write. It is declared *beside* `plugins`, not instead of it,
    // and the redundancy is deliberate. Ownership here is by prefix, so `plugins`
    // already covers it -- but the consumer validates a compiler's route by exact
    // membership in this list, so a release declaring only one of the two refuses
    // every install against a CLI that names the other. Declaring both is the
    // only state in which either side may move first.
    // Four were added 2026-08-28: `rules`, `commands`, `hooks.json` and
    // `mcp.json`. All four had been declined on the strength of vendor pages
    // that do not mention them, and all four are in the product. Its own
    // rule-creation code offers a *User Rule* scope at
    // `join(homedir(), ".cursor", "rules")`; it calls
    // `loadCommandsFromDirectory(join(userHomeDirectory, ".cursor",
    // "commands"))`; it resolves `userConfigPath` for `hooks.json` and a user
    // path for `mcp.json`.
    //
    // Widening is safe in the direction a consumer reads: it matches a route by
    // membership in this list, so a larger set makes more routes valid and none
    // that were valid invalid. Narrowing is the move that refuses things, which
    // is why `plugins` stays beside `plugins/local`.
    native_namespaces: &[
        "cli-config.json",
        "plugins",
        "plugins/local",
        "rules",
        "commands",
        "hooks.json",
        "mcp.json",
        // Added 2026-08-28 from the product's own bytes. `skill-path-utils.ts`
        // in the pinned bundle carries a table of skill roots -- `.cursor/skills`
        // beside `.claude/skills`, `.codex/skills`, `.grok/skills`,
        // `.agents/skills` -- and a mapper that joins each to the home
        // directory with `scope: "user"`. The product's own ignore file names
        // the directory in prose: *"# User's personal skills"*.
        //
        // It had been *declined* on `cursor.com/docs/skills`, which describes
        // the plugin-manifest key and does not mention the directory. That is
        // the same defect as the nine surfaces found on 2026-08-28: declined on
        // a page that does not discuss them, and present in the product.
        "skills",
    ],
    // The product's own: credentials, session history and runtime caches. Never
    // read, never written, and never copied into a backup slot.
    // Nothing measured. This product's alternate spellings, if it has
    // any, have not been asked for -- empty here says nobody looked,
    // not that the product reads one name.
    shadowing_names: &[],
    never_touch: &["auth.json", "sessions"],
    // No near neighbour measured for this product. A marker listed here is a
    // refusal waiting to happen, so nothing is listed without evidence.
    foreign_homes: &[],
    permission_profiles: &["default"],
    // Two, because two surfaces exist. Everything Cursor can be taught -- rules,
    // skills, agents, commands, hooks, MCP servers -- arrives inside a plugin,
    // so the plugin is the component and the rest are its manifest's keys.
    //
    // Declaring the other six promised a rollback for six things this provider
    // could not install. Conformance passed throughout: its route case asks for
    // one compilable kind, not every one.
    // `Instruction`, `Command`, `Hook` and `Mcp` joined 2026-08-28 with the
    // namespaces above. Each is a promise of a rollback and each can be kept:
    // every one of the four surfaces is written by materialising a bundle's
    // bytes verbatim, captured whole by a backup, and returned whole by a
    // restore. `hooks.json` and `mcp.json` are files rather than directories,
    // which is only a problem where a component is not the file -- the reason
    // claude-code's `Plugin` was withdrawn. Here the component *is* the file.
    component_kinds: &[
        ComponentKind::Plugin,
        ComponentKind::Setting,
        ComponentKind::Instruction,
        ComponentKind::Command,
        ComponentKind::Hook,
        ComponentKind::Mcp,
        // `Skill` joined 2026-08-28 with the `skills` namespace above, on the
        // product's bytes rather than on its skills page. Two of the roots in
        // that table are *not* ours and are recorded as declined instead:
        // `skills-cursor` is flagged `builtin` -- the product's own -- and
        // `cloud-skills` is filled from the account.
        ComponentKind::Skill,
    ],
    projection_kinds: &[
        ProjectionKind::NativeFiles,
        // `Marketplace` was declared here and named nothing this provider owns.
        // Every path a marketplace is registered in is in this harness's own
        // *declined* list -- `plugins/marketplaces`,
        // `plugins/local-marketplaces.json`, `plugins/installed_plugins.json`
        // and `plugins/cache` -- so the declaration promised a package family
        // that could not land. `Plugin` stays, because `plugins/local` is owned
        // and is where the vendor tells a person to put one.
        //
        // A declaration with one real half and one naming nothing is worse than
        // two wrong halves with a note explaining them, because the note is the
        // thing that gets re-read. Withdrawn 2026-08-29 after the consumer
        // counted its published corpus: `marketplace` is requested by nothing,
        // anywhere, so this narrows a promise rather than stranding a component.
        ProjectionKind::Plugin,
    ],
    // **Two scopes.** The second is `~/.agents`, the one root in this estate
    // that belongs to a convention rather than to a product: a *sibling* of
    // this product's configuration home, not a child, so nothing declared
    // against the target above can reach it. That is what `user_root` is for.
    //
    // The pinned bundle's skill-root table carries `{configDir: ".agents", subdir: "skills", thirdParty: false}` and its mapper joins each to `homedir()` with `scope: "user"`.
    //
    // **This was a declined row until now, and the reason it carried had
    // stopped being true.** It read *a namespace is removed whole, so a second
    // declaration would make either provider's remove take the other's
    // skills.* Correct when written; false since `written_paths` shipped.
    // Under a scope every verb acts on the files this provider recorded
    // writing -- the removal refuses rather than widening when it cannot read
    // the record, the capture takes ours and not a neighbour's, and a restore
    // leaves a neighbour's file as it was. Five of the seven products read
    // this root and one declared it; the reason was simply not re-read when
    // the thing it described changed.
    scoped_projections: &[Scoped {
        target_scope: TargetScope::UserRoot,
        // Distinct from the global identity, because the digest binds a
        // declaration together with the scope it owns.
        profile_id: "cursor/native-files/user-root/1",
        component_kinds: &[ComponentKind::Skill],
        projection_kinds: &[ProjectionKind::NativeFiles],
        // Relative to `~/.agents`, which is the target this scope names -- so a
        // skill is `skills/<name>` rather than `.agents/skills/<name>`. Writing
        // the root into the path would put the skills at
        // `~/.agents/.agents/skills`.
        native_namespaces: &["skills"],
    }],
    max_files: 8192,
    max_bytes: 64 * 1024 * 1024,
    kit_identity: include_str!("../../../provider-kit/v3/KIT-IDENTITY.json"),
    // Generated by `build.rs` from this harness's `setups/` directory, so the
    // binary carries the catalog it is named after instead of hoping to find
    // one on a disk it was never shipped to.
    embedded_setups: include!(concat!(env!("OUT_DIR"), "/embedded_setups.rs")),
    software: Some(software::SOFTWARE),
};

fn main() -> ExitCode {
    harness_runtime::run(&CURSOR, std::env::args().skip(1).collect())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic)]

    use super::*;

    /// The directory name this harness's setups live under in the workspace.
    const TOOL: &str = "cursor";
    /// The declaration under test, named once so the shared test below reads
    /// the same in all seven crates.
    const HARNESS: Harness = CURSOR;

    /// `build.rs` put the whole catalog in, under the paths it will be read by.
    ///
    /// This does **not** test for staleness, and an earlier version of this
    /// comment claimed it did. It cannot: `build.rs` declares
    /// `rerun-if-changed` on the catalog directory, so editing a setup rebuilds
    /// the table before this runs, and the test would be comparing the tree
    /// with itself. Observed — a deliberately edited setup left it green.
    ///
    /// What it does test is the build script, against a walk written
    /// independently of it: every file present, none invented, bytes exact, and
    /// paths relative and slash-separated. That last one is the one that would
    /// really break — `join("/")` is the only reason these keys are usable on
    /// Windows, and a path built with the platform separator would still look
    /// perfectly correct in the generated source.
    /// The bytes this harness ships, pinned so they cannot change unseen.
    ///
    /// A setup's `definition_digest` is what makes two setups the same setup,
    /// and it appears in `list`, in a plan and in provider state -- and until
    /// this, nothing compared it to anything. A stray character in a setup file
    /// changed what the estate installs and every test stayed green.
    ///
    /// One aggregate rather than one per setup, because the claim is about the
    /// catalogue: sorted definition digests, joined by a newline, hashed. A
    /// deliberate change to a setup updates the line in the baseline, which is
    /// the point -- the peer calls this a golden and it earns itself the first
    /// time a row moves without anyone meaning it to.
    ///
    /// **And it is the three-OS check nothing else makes.** The setups are
    /// embedded with `include_bytes!`, so whatever the checkout holds is what
    /// ships; `.gitattributes` pins `eol=lf` to keep a Windows checkout from
    /// rewriting them, and this is the assertion that would notice if it ever
    /// stopped working. The matrix runs it on all three systems, so a digest
    /// that differed by platform could not stay hidden.
    #[test]
    fn the_catalogue_this_harness_ships_is_the_one_the_baseline_records() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let mut digests: Vec<String> = catalog
            .list()
            .unwrap()
            .iter()
            // **Both digests, because one of them holds nothing a person
            // reads.** `definition_digest` is the payload tree; the manifest --
            // `id`, `sources`, `description` -- was covered by no digest in this
            // estate, and those three are what a consumer renders on the surface
            // that precedes an install. A description was rewritten and the
            // whole gate stayed clean, which is how this was found.
            .map(|setup| format!("{}\n{}", setup.definition_digest, setup.manifest_digest))
            .collect();
        digests.sort();
        let joined = digests.join("\n");
        let aggregate = harness_runtime::digest_of_bytes(&joined);
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../references")
            .join(format!("{TOOL}-baseline.json"));
        let baseline: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        let recorded = baseline["setup_catalogue_digest"].as_str().unwrap_or("");
        assert_eq!(
            aggregate, recorded,
            "the setups this binary ships are not the ones {TOOL}-baseline.json \
             records; if the change was meant, put this digest there"
        );
    }

    #[test]
    fn the_catalog_this_binary_carries_is_the_one_in_the_tree() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        // The workspace holds one directory per harness; a rendered public tree
        // ships one harness and holds it flat. Same two candidates `build.rs`
        // chooses between, asked the same way.
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };

        // Only the setup directories, which is what the reader lists and what
        // `build.rs` embeds. A rendered public tree also carries a
        // `setups/README.md` at the catalog root, which belongs to no setup.
        let mut on_disk = Vec::new();
        let mut stack: Vec<std::path::PathBuf> = std::fs::read_dir(&root)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.join("setup.json").is_file())
            .collect();
        while let Some(directory) = stack.pop() {
            for entry in std::fs::read_dir(&directory).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    stack.push(path);
                } else {
                    on_disk.push(path);
                }
            }
        }

        assert_eq!(
            HARNESS.embedded_setups.len(),
            on_disk.len(),
            "the binary carries {} files and the tree holds {}",
            HARNESS.embedded_setups.len(),
            on_disk.len()
        );

        for (relative, bytes) in HARNESS.embedded_setups {
            assert!(
                !relative.contains('\\') && !relative.starts_with('/'),
                "{relative:?} is not a relative slash path; a key built with the \
                 platform separator reads correctly on Unix and finds nothing on Windows"
            );
            let path = root.join(relative);
            let found = std::fs::read(&path)
                .unwrap_or_else(|e| panic!("{relative} is compiled in but not in the tree: {e}"));
            assert_eq!(
                &found, bytes,
                "{relative} differs between the binary and the tree"
            );
        }
    }

    #[test]
    fn the_declaration_is_valid_and_names_this_host() {
        let info = CURSOR.provider_info().unwrap();
        assert_eq!(info.provider_id, env!("CARGO_PKG_NAME"));
        assert_eq!(info.harness_id, "cursor");
        assert_eq!(info.protocol_version, 3);
        assert!(info.supports_this_host());
    }

    #[test]
    fn no_namespace_is_both_owned_and_disclaimed() {
        for name in CURSOR.never_touch {
            assert!(
                !CURSOR.native_namespaces.contains(name),
                "{name} is claimed and disclaimed"
            );
        }
    }

    /// Everything this harness claims to own, against the vendor page that
    /// decided it.
    ///
    /// What this replaced only checked that the baseline parsed. The block it
    /// reads now is hand-authored beside the rest of the baseline, and this is
    /// what keeps that block from being decoration: a namespace no vendor
    /// document names, or a declared kind no owned surface routes, is red here.
    ///
    /// Both directions, because the defect it was written for ran both ways --
    /// `~/.cursor/rules` was owned and does not exist, `~/.pi/agent/prompts`
    /// exists and was not owned. Conformance caught neither: its
    /// `declared_native_route_is_compilable` case asks for **one** route, not
    /// every one.
    #[test]
    fn every_surface_this_harness_owns_is_one_the_vendor_documents() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../references")
            .join(format!("{TOOL}-baseline.json"));
        let baseline: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        let problems = harness_runtime::surfaces::disagreements(&HARNESS, &baseline);
        assert!(
            problems.is_empty(),
            "the declaration and {TOOL}-baseline.json disagree:
  {}",
            problems.join(
                "
  "
            )
        );
    }

    #[test]
    fn the_control_directory_and_state_file_are_provider_owned_not_product_owned() {
        assert!(CURSOR.control_directory.contains("setup-system"));
        assert!(CURSOR.state_file.starts_with("NDDEV-"));
        assert!(!CURSOR.native_namespaces.contains(&CURSOR.state_file));
    }
    /// A setup that writes a configuration file says where its format came from.
    ///
    /// The release before this one made the *surfaces* sourced: a path this
    /// provider owns cites the page that documents it. This is the same rule
    /// one level down, and it was written because two of the seven failed it.
    ///
    /// opencode's baseline set `"permission": "ask"` where the product
    /// documents an object of tool names, and antigravity's set
    /// `toolPermissions` where the product reads `toolPermission` with four
    /// values, none of them the one written. Both were valid JSON in the right
    /// file at the right path. Both installed, verified and restored cleanly.
    /// Neither changed anything about the product — a target that looks
    /// configured and is not, which is the failure this estate refuses one
    /// level up and had been shipping one level down.
    /// Two files in one setup that a case-insensitive filesystem would merge.
    ///
    /// macOS and Windows fold case, so such a pair is one file there and two on
    /// Linux -- the setup would install different content depending on the
    /// machine, and its catalogue digest would differ per platform. The bundle
    /// reader has refused this for an arriving bundle since 0.0.11; this is the
    /// same rule applied to what this repository authors.
    /// Every component entry point describes itself.
    ///
    /// A `SKILL.md` or an agent whose frontmatter lost its `description` still
    /// installs, verifies and restores cleanly -- and the product names it after
    /// its directory and gives the model nothing to choose on. Documents under
    /// `references/` and files under `commands/` are exempt, because the
    /// products measured do not read frontmatter from either.
    /// Supporting documents are reachable from an entry point.
    ///
    /// A `references/` folder whose skill has no `SKILL.md` is prose nothing
    /// routes to. A generator in this repository produced exactly that, and
    /// every other guard passed it: the files are documents, so `unsourced`
    /// exempts them, and there is no `SKILL.md`, so `undescribed` has nothing
    /// to check.
    /// Nothing shipped sends a reader to a file this setup does not carry.
    ///
    /// A routing table naming `references/surfaces.md` in a setup that ships no
    /// such file sends the reader nowhere -- and the reader is a model, which
    /// will not say so. The generator here did exactly that: it pointed every
    /// harness's agent at that path, and codex ships no skill at all.
    #[test]
    fn nothing_shipped_names_a_document_it_does_not_carry() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems = harness_runtime::catalog::dangling_references(&catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }

    #[test]
    fn every_reference_folder_has_an_entry_point() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems = harness_runtime::catalog::unreachable_references(&catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }

    /// Nothing inside a skill is a file no reader is sent to.
    ///
    /// Two findings in one hour were of exactly this shape and every guard in
    /// this estate was silent on both: an executable validator shipped into
    /// people's homes that nothing named, and eleven authoring pages written
    /// into four harnesses and routed to from none. The estate asked whether a
    /// *named* file exists and never whether an *existing* file is named.
    #[test]
    fn nothing_inside_a_skill_is_stranded() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let found = harness_runtime::catalog::stranded(
            &harness_runtime::Catalog::at(&root).list().unwrap(),
        );
        assert!(found.problems.is_empty(), "{}", found.problems.join("\n  "));
        // cursor carries 14 file(s) inside its skill. Stated so that a layout change emptying the skill fails here rather than passing a guard with nothing left to walk.
        assert_eq!(
            found.entry_points, 14,
            "the stranded-file guard walked {} files inside skills, not 14",
            found.entry_points
        );
    }

    #[test]
    fn every_component_entry_point_describes_itself() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let examined = harness_runtime::catalog::undescribed(&catalog.list().unwrap());
        assert!(
            examined.problems.is_empty(),
            "{}",
            examined.problems.join("\n  ")
        );
        // cursor ships 2 entry point(s) across its four postures. Stated so that a layout change removing them fails here rather than passing a guard with nothing left to check.
        assert_eq!(
            examined.entry_points, 2,
            "the description guard examined {} entry points, not 2",
            examined.entry_points
        );
    }

    #[test]
    fn no_two_files_in_a_setup_differ_only_in_case() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems = harness_runtime::catalog::colliding(&catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }

    #[test]
    fn a_setup_that_writes_configuration_says_where_its_format_came_from() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems = harness_runtime::catalog::unsourced(&catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }
    /// Three postures, on every one of the seven.
    ///
    /// `baseline` is a working floor, `minimal` is the product's own defaults,
    /// and `full-auto` asks nothing and sandboxes nothing. A caller who learns
    /// them on one product knows them on all seven, which is the whole reason
    /// the names are the estate's rather than each harness's.
    ///
    /// The second half of the check is the one worth having: two setups with
    /// the same bytes mean one of them is a posture in name only, and it would
    /// still read as offered in `list`.
    #[test]
    fn the_three_postures_are_offered_and_are_actually_different() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems = harness_runtime::catalog::asymmetric(&catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }
    /// Nothing this setup ships tells a reader to run something that is not here.
    ///
    /// A setup carries documents an agent reads and acts on -- a skill, a rule,
    /// a command file -- and nothing was checking them. One shipped
    /// `software-status --target <dir> --json` and `list --json` for six
    /// releases; the binary refuses both, and says so in those words.
    ///
    /// Two refusals: a name belonging to the frozen estate, and any line naming
    /// this provider followed by a verb `into_command` does not accept. English
    /// is not judged -- `install` in a sentence is a word, and only
    /// `<provider> install` is an instruction.
    #[test]
    fn nothing_this_harness_ships_names_a_command_it_refuses() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems =
            harness_runtime::catalog::misdirecting(HARNESS.provider_id, &catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }
}
