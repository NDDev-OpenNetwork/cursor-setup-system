//! The Cursor CLI setup system.
//!
//! This file is the harness's *facts*. Every command over them lives in
//! [`harness_runtime`], shared with every other setup system, so a change to
//! behaviour lands once and a change to Cursor CLI's surface lands here.
//!
//! The owner assigned this harness the program lifecycle as well. It is not
//! declared yet, for the same reason as Grok and Pi.

use std::process::ExitCode;

use harness_runtime::Harness;
use provider_v3::{ComponentKind, ProjectionKind};

/// Everything specific to Cursor CLI, verified against `cursor-baseline.json`.
pub const CURSOR: Harness = Harness {
    harness_id: "cursor",
    provider_id: "cursor-setup-system",
    version: env!("CARGO_PKG_VERSION"),
    product: "Cursor CLI",
    vendor: "Anysphere",
    documented_config_home: "~/.cursor",
    config_home_env: "CURSOR_CONFIG_DIR",
    control_directory: ".cursor-setup-system",
    state_file: "NDDEV-CURSOR-PROVIDER.json",
    profile_id: "cursor/native-and-plugins/1",
    // Everything outside this list is a sibling overlay preserved verbatim.
    native_namespaces: &[
        "AGENTS.md",
        "cli-config.json",
        "rules",
        "skills",
        "agents",
        "commands",
        "hooks",
        "plugins",
    ],
    // The product's own: credentials, session history and runtime caches. Never
    // read, never written, and never copied into a backup slot.
    never_touch: &["auth.json", "sessions"],
    permission_profiles: &["default"],
    component_kinds: &[
        ComponentKind::Instruction,
        ComponentKind::Skill,
        ComponentKind::Agent,
        ComponentKind::Command,
        ComponentKind::Hook,
        ComponentKind::Mcp,
        ComponentKind::Plugin,
        ComponentKind::Setting,
    ],
    projection_kinds: &[
        ProjectionKind::NativeFiles,
        ProjectionKind::Marketplace,
        ProjectionKind::Plugin,
    ],
    max_files: 8192,
    max_bytes: 64 * 1024 * 1024,
    kit_identity: include_str!("../../../provider-kit/v3/KIT-IDENTITY.json"),
};

fn main() -> ExitCode {
    harness_runtime::run(&CURSOR, std::env::args().skip(1).collect())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic)]

    use super::*;

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

    #[test]
    fn the_baseline_this_harness_cites_is_present_and_readable() {
        // The facts above are transcribed from it; a build whose baseline is
        // missing has no evidence for what it claims to own.
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../references/cursor-baseline.json");
        let value: serde_json::Value =
            serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
        assert!(value.is_object());
    }

    #[test]
    fn the_control_directory_and_state_file_are_provider_owned_not_product_owned() {
        assert!(CURSOR.control_directory.contains("setup-system"));
        assert!(CURSOR.state_file.starts_with("NDDEV-"));
        assert!(!CURSOR.native_namespaces.contains(&CURSOR.state_file));
    }
}
