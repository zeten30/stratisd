// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Utilities to support Stratis.

use std::path::{Path, PathBuf};
use std::process::Command;

use uuid::Uuid;

use stratis::{ErrorEnum, StratisError, StratisResult};


const PATH_PREFIX: &str = "/usr/sbin";

/// Common function to call a command line utility, returning an Result with an error message which
/// also includes stdout & stderr if it fails.
fn execute_cmd(cmd: &mut Command, error_msg: &str) -> StratisResult<()> {
    let result = cmd.output()?;
    if result.status.success() {
        Ok(())
    } else {
        let std_out_txt = String::from_utf8_lossy(&result.stdout);
        let std_err_txt = String::from_utf8_lossy(&result.stderr);
        let err_msg = format!("{} stdout: {} stderr: {}",
                              error_msg,
                              std_out_txt,
                              std_err_txt);
        Err(StratisError::Engine(ErrorEnum::Error, err_msg))
    }
}

/// Create a filesystem on devnode.
pub fn create_fs(devnode: &Path, uuid: Uuid) -> StratisResult<()> {
    execute_cmd(Command::new([PATH_PREFIX, "mkfs.xfs"].iter().collect::<PathBuf>())
                    .arg("-f")
                    .arg("-q")
                    .arg(&devnode)
                    .arg("-m")
                    .arg(format!("uuid={}", uuid)),
                &format!("Failed to create new filesystem at {:?}", devnode))
}

/// Use the xfs_growfs command to expand a filesystem mounted at the given
/// mount point.
pub fn xfs_growfs(mount_point: &Path) -> StratisResult<()> {
    execute_cmd(Command::new([PATH_PREFIX, "xfs_growfs"].iter().collect::<PathBuf>())
                    .arg(mount_point)
                    .arg("-d"),
                &format!("Failed to expand filesystem {:?}", mount_point))
}

/// Set a new UUID for filesystem on the devnode.
pub fn set_uuid(devnode: &Path, uuid: Uuid) -> StratisResult<()> {
    execute_cmd(Command::new([PATH_PREFIX, "xfs_admin"].iter().collect::<PathBuf>())
                    .arg("-U")
                    .arg(format!("{}", uuid))
                    .arg(&devnode),
                &format!("Failed to set UUID for filesystem {:?}", devnode))
}

/// Call thin_check on a thinpool
pub fn thin_check(devnode: &Path) -> StratisResult<()> {
    execute_cmd(Command::new([PATH_PREFIX, "thin_check"].iter().collect::<PathBuf>())
                    .arg("-q")
                    .arg(devnode),
                &format!("thin_check for thin pool meta device {:?} failed", devnode))
}

/// Call thin_repair on a thinpool
pub fn thin_repair(meta_dev: &Path, new_meta_dev: &Path) -> StratisResult<()> {
    execute_cmd(Command::new([PATH_PREFIX, "thin_repair"].iter().collect::<PathBuf>())
                    .arg("-i")
                    .arg(meta_dev)
                    .arg("-o")
                    .arg(new_meta_dev),
                &format!("thin_repair of thin pool meta device {:?} failed", meta_dev))
}
