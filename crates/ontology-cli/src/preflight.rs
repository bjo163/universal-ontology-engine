use std::fs;
use std::path::Path;

const MAX_SOURCE_BYTES: u64 = 2 * 1024 * 1024;
const MAX_SOURCE_FILES: usize = 100_000;
const HARD_MAX_DEPTH: usize = 256;

const IGNORED: &[&str] = &[
    ".git",
    ".next",
    ".turbo",
    "node_modules",
    "target",
    "dist",
    "build",
    "coverage",
    "__pycache__",
    ".venv",
    "venv",
    ".idea",
    ".vscode",
    "vendor",
];

const SOURCE_EXTENSIONS: &[&str] = &[
    "rs", "ts", "tsx", "js", "jsx", "go", "py", "java", "kt", "rb", "php", "cs", "cpp", "h", "hpp",
];

pub fn validate_workspace(
    workspace: &Path,
    requested_max_depth: Option<usize>,
) -> Result<(), String> {
    let metadata = fs::symlink_metadata(workspace).map_err(|error| {
        format!(
            "workspace preflight failed for `{}`: {error}",
            workspace.display()
        )
    })?;
    if metadata.file_type().is_symlink() {
        return Err(format!(
            "workspace symlink is not allowed by the MVP safety contract: `{}`",
            workspace.display()
        ));
    }
    if !metadata.is_dir() {
        return Err(format!(
            "workspace is not a directory: `{}`",
            workspace.display()
        ));
    }

    let max_depth = requested_max_depth
        .unwrap_or(HARD_MAX_DEPTH)
        .min(HARD_MAX_DEPTH);
    let mut source_files = 0usize;
    walk(workspace, 0, max_depth, &mut source_files)
}

fn walk(
    path: &Path,
    depth: usize,
    max_depth: usize,
    source_files: &mut usize,
) -> Result<(), String> {
    if depth > max_depth {
        return Ok(());
    }

    let entries = fs::read_dir(path).map_err(|error| {
        format!(
            "workspace preflight cannot read `{}`: {error}",
            path.display()
        )
    })?;

    for entry in entries {
        let entry = entry
            .map_err(|error| format!("workspace preflight directory entry failed: {error}"))?;
        let child = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if IGNORED.contains(&name.as_ref()) {
            continue;
        }

        let file_type = entry.file_type().map_err(|error| {
            format!(
                "workspace preflight cannot inspect `{}`: {error}",
                child.display()
            )
        })?;
        if file_type.is_symlink() {
            return Err(format!(
                "symlink evidence is not followed by the MVP safety contract: `{}`",
                child.display()
            ));
        }
        if file_type.is_dir() {
            walk(&child, depth + 1, max_depth, source_files)?;
            continue;
        }
        if !file_type.is_file() || !is_source_file(&child) {
            continue;
        }

        *source_files += 1;
        if *source_files > MAX_SOURCE_FILES {
            return Err(format!(
                "workspace exceeds MVP source-file safety limit ({MAX_SOURCE_FILES})"
            ));
        }

        let bytes = entry
            .metadata()
            .map_err(|error| {
                format!(
                    "workspace preflight cannot stat `{}`: {error}",
                    child.display()
                )
            })?
            .len();
        if bytes > MAX_SOURCE_BYTES {
            return Err(format!(
                "source exceeds MVP per-file safety limit ({} bytes > {} bytes): `{}`",
                bytes,
                MAX_SOURCE_BYTES,
                child.display()
            ));
        }
    }

    Ok(())
}

fn is_source_file(path: &Path) -> bool {
    path.extension()
        .and_then(|value| value.to_str())
        .is_some_and(|extension| SOURCE_EXTENSIONS.contains(&extension))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_root(label: &str) -> std::path::PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("ontology-preflight-{label}-{stamp}"))
    }

    #[test]
    fn accepts_small_read_only_source_tree() {
        let root = temp_root("small");
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(root.join("src/lib.rs"), "pub fn demo() {}\n").unwrap();
        validate_workspace(&root, None).unwrap();
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn rejects_oversized_source_before_parsing() {
        let root = temp_root("large");
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(
            root.join("src/large.rs"),
            vec![b'x'; (MAX_SOURCE_BYTES as usize) + 1],
        )
        .unwrap();
        let error = validate_workspace(&root, None).unwrap_err();
        assert!(error.contains("per-file safety limit"));
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn rejects_symlink_before_discovery_can_follow_it() {
        use std::os::unix::fs::symlink;

        let root = temp_root("symlink");
        let outside = temp_root("outside");
        fs::create_dir_all(root.join("src")).unwrap();
        fs::create_dir_all(&outside).unwrap();
        fs::write(outside.join("outside.rs"), "pub fn outside() {}\n").unwrap();
        symlink(&outside, root.join("src/escape")).unwrap();

        let error = validate_workspace(&root, None).unwrap_err();
        assert!(error.contains("symlink evidence is not followed"));

        fs::remove_dir_all(root).unwrap();
        fs::remove_dir_all(outside).unwrap();
    }
}
