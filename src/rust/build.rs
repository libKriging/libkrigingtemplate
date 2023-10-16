/*
 * Copyright Nuant (c) 2022.
 */

use std::error::Error;
use vergen::EmitBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    EmitBuilder::builder()
        .build_timestamp()
        .cargo_debug()
        .cargo_target_triple()
        .cargo_features()
        .git_branch()
        .git_commit_date()
        .git_commit_timestamp()
        .git_describe(true, true, Some("v[0-9]*"))
        .git_sha(false)
        .rustc_semver()
        .rustc_channel()
        .emit()?;
    Ok(())
}
