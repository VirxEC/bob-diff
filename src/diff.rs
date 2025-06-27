use anyhow::anyhow;
use bob_lib::bobdiff;
use std::{fs, path::Path};

pub fn command_diff(old: &Path, new: &Path) -> anyhow::Result<Vec<u8>> {
    if !fs::exists(old)? {
        return Err(anyhow!("Directory doesn't exist: {old:?}"));
    }

    if !fs::exists(new)? {
        return Err(anyhow!("Directory doesn't exist: {new:?}"));
    }

    let diff = bobdiff::DirDiff::new(old, new);
    Ok(diff.ser())
}

pub fn command_diff_apply(dir: &Path, buf: &[u8]) -> anyhow::Result<()> {
    if !fs::exists(dir)? {
        return Err(anyhow!("Directory doesn't exist"));
    }

    let diff = bobdiff::DirDiff::deser(buf)?;
    diff.apply_to(dir, true)?;

    Ok(())
}
