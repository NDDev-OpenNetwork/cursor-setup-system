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
        url: "https://downloads.cursor.com/lab/2026.09.23-86fc751/linux/arm64/agent-cli-package.tar.gz",
        bytes: 180_809_100,
        sha256: "sha256:38d1482c945172926e780206fce8cc51c67dd4f96162bcec680903aa70d80417",
        shape: Shape::GzipTar,
        member: "dist-package/cursor-agent",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://downloads.cursor.com/lab/2026.09.23-86fc751/linux/x64/agent-cli-package.tar.gz",
        bytes: 182_867_455,
        sha256: "sha256:740dd9d6eb5aec36ca90eaedf9fd5e2c489cd674d69b5147b3c2670f02d9776d",
        shape: Shape::GzipTar,
        member: "dist-package/cursor-agent",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://downloads.cursor.com/lab/2026.09.23-86fc751/darwin/arm64/agent-cli-package.tar.gz",
        bytes: 177_065_953,
        sha256: "sha256:fa3fe13d5589c586ff132a24c16eea96fb8efde88afdefddcd11d80fa199f3a5",
        shape: Shape::GzipTar,
        member: "dist-package/cursor-agent",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://downloads.cursor.com/lab/2026.09.23-86fc751/darwin/x64/agent-cli-package.tar.gz",
        bytes: 184_447_094,
        sha256: "sha256:809335cd4a92f7c11a20f605213585b6136c7bfe6722069facf94e988d3ed0e7",
        shape: Shape::GzipTar,
        member: "dist-package/cursor-agent",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://downloads.cursor.com/lab/2026.09.23-86fc751/windows/arm64/agent-cli-package.zip",
        bytes: 75_119_584,
        sha256: "sha256:9044b00326ffdee83b6c809f2ba5d6545dd9bc85d5d26695ba1db529f7c3cee5",
        shape: Shape::Zip,
        member: "dist-package/cursor-agent.cmd",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://downloads.cursor.com/lab/2026.09.23-86fc751/windows/x64/agent-cli-package.zip",
        bytes: 77_219_487,
        sha256: "sha256:9c7232600ca77e7a6327dc9a0036277bd57b66923c6b65fc7abe9d957eb728e2",
        shape: Shape::Zip,
        member: "dist-package/cursor-agent.cmd",
    },
];

/// The artifacts 2026.09.18-9a7762b was published as, kept so
/// `software_update` has a version to move from and `rollback` a tree to
/// return to. Measured from bytes when it was the current pin.
pub(crate) const PREVIOUS_ARTIFACTS: &[Artifact] = &[
    Artifact {
        platform: "linux/arm64",
        url: "https://downloads.cursor.com/lab/2026.09.18-9a7762b/linux/arm64/agent-cli-package.tar.gz",
        bytes: 180_513_813,
        sha256: "sha256:210d58f850f4616e4f265ff7006c558d5a2a008fce3e2bf8a0105c25ae3456a0",
        shape: Shape::GzipTar,
        member: "dist-package/cursor-agent",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://downloads.cursor.com/lab/2026.09.18-9a7762b/linux/x64/agent-cli-package.tar.gz",
        bytes: 182_574_768,
        sha256: "sha256:b1308f5a2fc05458b9d8966752986bb23a971bbcc67c842c1df94c4b8132bad9",
        shape: Shape::GzipTar,
        member: "dist-package/cursor-agent",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://downloads.cursor.com/lab/2026.09.18-9a7762b/darwin/arm64/agent-cli-package.tar.gz",
        bytes: 176_832_653,
        sha256: "sha256:4e67b9ac80cc4a56e0a91b3b437894e0ba489ef7ec37f120d8084d2bfd02095d",
        shape: Shape::GzipTar,
        member: "dist-package/cursor-agent",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://downloads.cursor.com/lab/2026.09.18-9a7762b/darwin/x64/agent-cli-package.tar.gz",
        bytes: 184_225_658,
        sha256: "sha256:f4298af7114a57ce317ddc13a49c273e1113b027e3e6b7a0769b8ef3565e6897",
        shape: Shape::GzipTar,
        member: "dist-package/cursor-agent",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://downloads.cursor.com/lab/2026.09.18-9a7762b/windows/arm64/agent-cli-package.zip",
        bytes: 74_962_496,
        sha256: "sha256:69090ef4cf44dc20b93dd9a6f99b0604b20217b22268d97e6a47ca546d7a0d4b",
        shape: Shape::Zip,
        member: "dist-package/cursor-agent.cmd",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://downloads.cursor.com/lab/2026.09.18-9a7762b/windows/x64/agent-cli-package.zip",
        bytes: 77_054_222,
        sha256: "sha256:9c1fbcda9f0a39667689a6a3146b549ffec465caf47723e0a7bb345142d5bde4",
        shape: Shape::Zip,
        member: "dist-package/cursor-agent.cmd",
    },
];

/// Cursor's program, and where its bytes come from.
pub(crate) const SOFTWARE: Software = Software {
    version: "2026.09.23-86fc751",
    command: "agent",
    delivery: Delivery::Artifacts(ARTIFACTS),
    unsupported: &[],
    previous: Some(Previous {
        version: "2026.09.18-9a7762b",
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
