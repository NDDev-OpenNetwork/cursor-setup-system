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
        url: "https://downloads.cursor.com/lab/2026.09.10-fd3934a/linux/arm64/agent-cli-package.tar.gz",
        bytes: 177_655_979,
        sha256: "sha256:e0494438b01c37bc34848491d1f3478ef469494c56caf020de11796d146db64a",
        shape: Shape::GzipTar,
        member: "dist-package/cursor-agent",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://downloads.cursor.com/lab/2026.09.10-fd3934a/linux/x64/agent-cli-package.tar.gz",
        bytes: 179_673_253,
        sha256: "sha256:27997c8391ad853a5a732b1845db8ef82a8ba6afb0f7829cc739464f8966e96e",
        shape: Shape::GzipTar,
        member: "dist-package/cursor-agent",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://downloads.cursor.com/lab/2026.09.10-fd3934a/darwin/arm64/agent-cli-package.tar.gz",
        bytes: 174_178_929,
        sha256: "sha256:aec0b01ae056de48a02fe315fbf0580eb91377752d993307499988cbe0285423",
        shape: Shape::GzipTar,
        member: "dist-package/cursor-agent",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://downloads.cursor.com/lab/2026.09.10-fd3934a/darwin/x64/agent-cli-package.tar.gz",
        bytes: 181_409_095,
        sha256: "sha256:964cc72e88125c6b48ecaaebef68bf7cb752eb7b9d010a5535cf9f8e677dcf83",
        shape: Shape::GzipTar,
        member: "dist-package/cursor-agent",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://downloads.cursor.com/lab/2026.09.10-fd3934a/windows/arm64/agent-cli-package.zip",
        bytes: 72_069_603,
        sha256: "sha256:acb34343a1915085fd882d392b4d3c35ecf54b15467df6a4cf34c62e065412e8",
        shape: Shape::Zip,
        member: "dist-package/cursor-agent.cmd",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://downloads.cursor.com/lab/2026.09.10-fd3934a/windows/x64/agent-cli-package.zip",
        bytes: 74_169_932,
        sha256: "sha256:cdf0b9b7c2f8892d4c2d2ccbf38226c831a0d56a043e2132d90f1aefac90b846",
        shape: Shape::Zip,
        member: "dist-package/cursor-agent.cmd",
    },
];

/// The artifacts 2026.09.02-c22c1a3 was published as, kept so
/// `software_update` has a version to move from and `rollback` a tree to
/// return to. Measured from bytes when it was the current pin.
pub(crate) const PREVIOUS_ARTIFACTS: &[Artifact] = &[
    Artifact {
        platform: "linux/arm64",
        url: "https://downloads.cursor.com/lab/2026.09.02-c22c1a3/linux/arm64/agent-cli-package.tar.gz",
        bytes: 177_647_834,
        sha256: "sha256:fb7bc635be6172ebcf68f907fd9217e3614da51916455c6d7fdb66690997884c",
        shape: Shape::GzipTar,
        member: "dist-package/cursor-agent",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://downloads.cursor.com/lab/2026.09.02-c22c1a3/linux/x64/agent-cli-package.tar.gz",
        bytes: 179_684_142,
        sha256: "sha256:b73b59854762535c0fc20d7ccc51c3b5a356a851491088d60a362be48750f53c",
        shape: Shape::GzipTar,
        member: "dist-package/cursor-agent",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://downloads.cursor.com/lab/2026.09.02-c22c1a3/darwin/arm64/agent-cli-package.tar.gz",
        bytes: 174_181_041,
        sha256: "sha256:3d814861be3225fc8c38be320fb22e344d8f711a24279f1f9119e7b313ea51e7",
        shape: Shape::GzipTar,
        member: "dist-package/cursor-agent",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://downloads.cursor.com/lab/2026.09.02-c22c1a3/darwin/x64/agent-cli-package.tar.gz",
        bytes: 181_413_402,
        sha256: "sha256:59e8afd9b4f5eba44682018b9428c4a94e230d090871eae19e703540aad6ef6a",
        shape: Shape::GzipTar,
        member: "dist-package/cursor-agent",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://downloads.cursor.com/lab/2026.09.02-c22c1a3/windows/arm64/agent-cli-package.zip",
        bytes: 72_044_862,
        sha256: "sha256:a503b66a18c175ead4bad2c1dd7abf96161fd03a4fccb7d1372d4a74df1d0b07",
        shape: Shape::Zip,
        member: "dist-package/cursor-agent.cmd",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://downloads.cursor.com/lab/2026.09.02-c22c1a3/windows/x64/agent-cli-package.zip",
        bytes: 74_166_530,
        sha256: "sha256:2a560a7629828fa00c1d6a67eef5ea9bd3e99609abd45cb02ef2e743e83b9d47",
        shape: Shape::Zip,
        member: "dist-package/cursor-agent.cmd",
    },
];

/// Cursor's program, and where its bytes come from.
pub(crate) const SOFTWARE: Software = Software {
    version: "2026.09.10-fd3934a",
    command: "agent",
    delivery: Delivery::Artifacts(ARTIFACTS),
    unsupported: &[],
    previous: Some(Previous {
        version: "2026.09.02-c22c1a3",
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
