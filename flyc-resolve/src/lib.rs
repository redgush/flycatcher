//! Tools for resolving module paths.
//!
//! The `lib` directory must be in the same directory as the executable.

use pathdiff::diff_paths;
use std::env::{current_dir, current_exe};
use std::path::{Path, PathBuf};

/// Returns the provided string relative to the current working directory.
pub fn get_debug_name(mut abs: String) -> PathBuf {
    if abs.starts_with("\\\\?\\") {
        abs = abs[4..].into();
    }

    diff_paths(abs, current_dir().unwrap()).unwrap()
}

/// Gets the absolute path to a Flycatcher module.  It will check the `src` path first because
/// it has higher priority, then it will check the modules path.
pub fn resolve_path(name: String, src: String) -> Option<PathBuf> {
    let mut tmp_name = name.clone();

    if !tmp_name.ends_with(".flyc") {
        tmp_name.push_str(".flyc");
    }

    let p = Path::new(&src).join(Path::new(&tmp_name));
    if p.exists() {
        return Some(p);
    } else {
        let mut exe = current_exe().unwrap();
        exe.pop();

        let mod_path = exe.join("lib");
        let folder = mod_path.join(&name);

        if folder.exists() {
            let module_file = folder.join(Path::new("mod.flyc"));
            if module_file.exists() {
                return Some(module_file);
            }
        }
    }

    None
}
