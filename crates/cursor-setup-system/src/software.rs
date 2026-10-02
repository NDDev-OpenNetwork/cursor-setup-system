//! Cursor's own program, as measured rather than as described.
//!
//! Generated from the `software_artifacts` block of
//! `references/cursor-baseline.json`. Every member path below was read out
//! of the archive it names, not assumed: codex's carries the target triple and
//! so genuinely differs per platform.
//!
//! Where a `previous_software_artifacts` block is present, it is transcribed
//! too. It is not a second choice: the outgoing current pin is stored there on
//! a bump, so the pair is always two consecutive real releases and there is
//! still exactly one value to keep fresh.
//!
//! Do not edit. The test at the bottom re-reads that baseline and compares it
//! field by field, so an edit here fails rather than silently installing bytes
//! nobody measured.

use harness_runtime::{Artifact, Delivery, Previous, Shape, Software};

/// The artifacts agent is published as.
pub(crate) const ARTIFACTS: &[Artifact] = &[
    Artifact {
        platform: "linux/arm64",
        url: "https://downloads.cursor.com/lab/2026.10.01-e373342/linux/arm64/agent-cli-package.tar.gz",
        bytes: 180_560_415,
        sha256: "sha256:785c5f6bf2a60eb1121e27ed8c14f5ee07ed1b5b6692324f2d9a997238245eb5",
        shape: Shape::GzipTar,
        member: "dist-package/cursor-agent",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://downloads.cursor.com/lab/2026.10.01-e373342/linux/x64/agent-cli-package.tar.gz",
        bytes: 182_613_962,
        sha256: "sha256:a79726c6e644520e993970be4c45775a6889802b67abe461a677a53219ae28e8",
        shape: Shape::GzipTar,
        member: "dist-package/cursor-agent",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://downloads.cursor.com/lab/2026.10.01-e373342/darwin/arm64/agent-cli-package.tar.gz",
        bytes: 176_796_992,
        sha256: "sha256:629e51de43a0b7fb3b86f5ebc7e579f7df7df941b39f29e82945cde750145afc",
        shape: Shape::GzipTar,
        member: "dist-package/cursor-agent",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://downloads.cursor.com/lab/2026.10.01-e373342/darwin/x64/agent-cli-package.tar.gz",
        bytes: 184_196_702,
        sha256: "sha256:a80db6e15626e0aa7af60fab3a95c98a243e2c1513c0c9485344a553d7b7b00e",
        shape: Shape::GzipTar,
        member: "dist-package/cursor-agent",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://downloads.cursor.com/lab/2026.10.01-e373342/windows/arm64/agent-cli-package.zip",
        bytes: 75_076_180,
        sha256: "sha256:3341c84d1de73461667df152967dc3a10394fb9792115cac1918457fe85c44b3",
        shape: Shape::Zip,
        member: "dist-package/cursor-agent.cmd",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://downloads.cursor.com/lab/2026.10.01-e373342/windows/x64/agent-cli-package.zip",
        bytes: 77_156_248,
        sha256: "sha256:31896ce9a43f63c8ae7ff3c4bee868213d6d02d60fb49908bef6dc91d582f6ee",
        shape: Shape::Zip,
        member: "dist-package/cursor-agent.cmd",
    },
];

/// The artifacts 2026.09.28-64d2043 was published as, kept so
/// `software_update` has a version to move from and `rollback` a tree to
/// return to. Measured from bytes when it was the current pin.
pub(crate) const PREVIOUS_ARTIFACTS: &[Artifact] = &[
    Artifact {
        platform: "linux/arm64",
        url: "https://downloads.cursor.com/lab/2026.09.28-64d2043/linux/arm64/agent-cli-package.tar.gz",
        bytes: 180_564_349,
        sha256: "sha256:c737599b27d3d8d6743c72b487204e335f3a8ea2fdbaf18302ee207a646ffd8d",
        shape: Shape::GzipTar,
        member: "dist-package/cursor-agent",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://downloads.cursor.com/lab/2026.09.28-64d2043/linux/x64/agent-cli-package.tar.gz",
        bytes: 182_618_248,
        sha256: "sha256:6e4cd936a4866b8a77c50ff51a564460d715772fabc477a01aa0f0455d9559f0",
        shape: Shape::GzipTar,
        member: "dist-package/cursor-agent",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://downloads.cursor.com/lab/2026.09.28-64d2043/darwin/arm64/agent-cli-package.tar.gz",
        bytes: 176_795_016,
        sha256: "sha256:c0d7e9cd2e62438610b886d3439907dc1f98c2923b07b3a41416cc919aaf53c7",
        shape: Shape::GzipTar,
        member: "dist-package/cursor-agent",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://downloads.cursor.com/lab/2026.09.28-64d2043/darwin/x64/agent-cli-package.tar.gz",
        bytes: 184_187_482,
        sha256: "sha256:3efe0dff2f3d92a1e8139e33fef182801556b57ed50a6afd7bac19c4fad09549",
        shape: Shape::GzipTar,
        member: "dist-package/cursor-agent",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://downloads.cursor.com/lab/2026.09.28-64d2043/windows/arm64/agent-cli-package.zip",
        bytes: 75_088_705,
        sha256: "sha256:72cfb4e4365a2c4b63c0c100c64c91d661bb619e2c890d8a7a2ba333df044f82",
        shape: Shape::Zip,
        member: "dist-package/cursor-agent.cmd",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://downloads.cursor.com/lab/2026.09.28-64d2043/windows/x64/agent-cli-package.zip",
        bytes: 77_143_980,
        sha256: "sha256:2d7abd33929520e2169d7392360e71a7b137144077d1574818960fe0949ec52a",
        shape: Shape::Zip,
        member: "dist-package/cursor-agent.cmd",
    },
];

/// Cursor's program, and where its bytes come from.
pub(crate) const SOFTWARE: Software = Software {
    version: "2026.10.01-e373342",
    command: "agent",
    delivery: Delivery::Artifacts(ARTIFACTS),
    unsupported: &[],
    previous: Some(Previous {
        version: "2026.09.28-64d2043",
        artifacts: PREVIOUS_ARTIFACTS,
    }),
};

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic)]

    // Named rather than glob-imported: a product delivered by a package manager
    // has no `Artifact` in scope, and the test is the same text for all seven.
    use harness_runtime::{Delivery, Shape};

    use super::SOFTWARE;

    fn measured() -> serde_json::Value {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../references/cursor-baseline.json");
        serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
    }

    #[test]
    fn every_artifact_compiled_in_is_the_one_the_baseline_measured() {
        let block = &measured()["software_artifacts"];
        assert_eq!(block["version"], SOFTWARE.version);
        assert_eq!(block["command"], SOFTWARE.command);

        let Delivery::Artifacts(compiled) = SOFTWARE.delivery else {
            // A product delivered by a package manager has no artifacts, and
            // the baseline must agree that it has none.
            assert_eq!(block["shape"], "manager");
            assert!(block["platforms"].as_object().unwrap().is_empty());
            return;
        };
        let published = block["platforms"].as_object().unwrap();
        assert_eq!(
            compiled.len(),
            published.len(),
            "the table and the baseline disagree on how many platforms exist"
        );
        for artifact in compiled {
            let entry = &published[artifact.platform];
            assert_eq!(entry["url"], artifact.url, "{}", artifact.platform);
            assert_eq!(entry["bytes"], artifact.bytes, "{}", artifact.platform);
            assert_eq!(entry["sha256"], artifact.sha256, "{}", artifact.platform);
            let member = entry.get("member").and_then(serde_json::Value::as_str);
            assert_eq!(
                member.unwrap_or(""),
                artifact.member,
                "{} names a different member",
                artifact.platform
            );
            assert_eq!(
                artifact.shape == Shape::Raw,
                member.is_none(),
                "{} disagrees about whether the bytes are the program",
                artifact.platform
            );
        }
    }

    /// The second pin is the baseline's, or it is absent in both places.
    ///
    /// Asserted from either side rather than only where it exists: a harness
    /// that has never been bumped must compile in `None`, and a build that
    /// dropped the block while the baseline still carried it would otherwise
    /// pass by having nothing to compare.
    #[test]
    fn the_version_this_build_can_move_between_is_the_one_measured_before_it() {
        let baseline = measured();
        let recorded = baseline.get("previous_software_artifacts");
        let Some(earlier) = SOFTWARE.previous else {
            assert!(
                recorded.is_none(),
                "the baseline records a previous release and this build names none"
            );
            return;
        };
        let block = recorded.unwrap_or_else(|| {
            panic!("this build names a previous release the baseline does not record")
        });
        assert_eq!(block["version"], earlier.version);
        assert_ne!(
            earlier.version, SOFTWARE.version,
            "a second pin equal to the first is one version wearing two names"
        );
        let published = block["platforms"].as_object().unwrap();
        assert_eq!(
            earlier.artifacts.len(),
            published.len(),
            "the previous table and the baseline disagree on how many platforms exist"
        );
        for artifact in earlier.artifacts {
            let entry = &published[artifact.platform];
            assert_eq!(entry["url"], artifact.url, "{}", artifact.platform);
            assert_eq!(entry["bytes"], artifact.bytes, "{}", artifact.platform);
            assert_eq!(entry["sha256"], artifact.sha256, "{}", artifact.platform);
        }
    }

    #[test]
    fn a_platform_the_vendor_does_not_publish_is_listed_rather_than_missing() {
        let block = &measured()["software_artifacts"];
        let unpublished: Vec<&str> = block
            .get("unpublished")
            .and_then(serde_json::Value::as_array)
            .map(|entries| {
                entries
                    .iter()
                    .filter_map(serde_json::Value::as_str)
                    .collect()
            })
            .unwrap_or_default();
        assert_eq!(unpublished, SOFTWARE.unsupported);
    }

    #[test]
    fn no_release_calls_a_platform_both_published_and_unpublished() {
        let baseline = measured();
        for name in ["software_artifacts", "previous_software_artifacts"] {
            let Some(block) = baseline.get(name) else {
                continue;
            };
            let published = block["platforms"].as_object().unwrap();
            let unpublished = block
                .get("unpublished")
                .and_then(serde_json::Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(serde_json::Value::as_str);
            for platform in unpublished {
                assert!(
                    !published.contains_key(platform),
                    "{name}: {platform} is both published and unpublished"
                );
            }
        }
    }
}
