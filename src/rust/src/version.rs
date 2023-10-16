pub fn short_version() -> String {
    let version = format!("v{}", env!("CARGO_PKG_VERSION"));
    let commit_description = env!("VERGEN_GIT_DESCRIBE");
    let component_version = format!("{}-{}", env!("CARGO_PKG_NAME"), version);
    let note = if component_version == commit_description {
        "release"
    } else {
        "development"
    };

    format!("{version} [{note}]")
}

pub fn full_version() -> String {
    let mut features = env!("VERGEN_CARGO_FEATURES");
    if features.is_empty() {
        features = "default"
    };

    display_formatted_pack(features).join("\n")
}

#[rustfmt::skip]
fn display_formatted_pack(features: &str,
) -> Vec<String> {
    vec![
        format!("{:->61}", ""),
        format!("Package semver:     {}", short_version()),
        format!("Build timestamp:    {}", env!("VERGEN_BUILD_TIMESTAMP")),
        format!("Commit timestamp:   {}", env!("VERGEN_GIT_COMMIT_TIMESTAMP")),
        format!("Commit branch:      {}", env!("VERGEN_GIT_BRANCH")),
        format!("Commit SHA:         {}", env!("VERGEN_GIT_SHA")),
        format!("Commit description: {}", env!("VERGEN_GIT_DESCRIBE")),
        format!("rustc version:      {} [{}]", env!("VERGEN_RUSTC_SEMVER"), env!("VERGEN_RUSTC_CHANNEL")),
        format!("Target:             {}", env!("VERGEN_CARGO_TARGET_TRIPLE")),
        format!("Profile:            {}", get_build_profile_name()),
        format!("Features:           {features}"),
    ]
}

// cf https://doc.rust-lang.org/cargo/reference/profiles.html#custom-profiles
// The output for each profile will be placed in a directory of the same name as the profile
// in the target directory.
// As in the example above, the output would go into the target/release-lto directory.

fn get_build_profile_name() -> String {
    // The profile name is always the 3rd last part of the path (with 1 based indexing).
    // e.g. /code/core/target/cli/build/my-build-info-9f91ba6f99d7a061/out
    let profile_name = env!("OUT_DIR")
        .split(std::path::MAIN_SEPARATOR)
        .nth_back(3)
        .unwrap_or_else(|| "unknown")
        .to_string();

    if env!("VERGEN_CARGO_DEBUG")
        .parse::<bool>()
        .unwrap_or_default()
    {
        format!("{profile_name} [with debug info]")
    } else {
        profile_name
    }
}

#[cfg(test)]
mod debug {
    use crate::version::full_version;

    #[test]
    fn display_version() {
        println!("{}", full_version())
    }
}
