use std::{fs::read_dir, path::Path};
use globset::{Glob, GlobMatcher};
use super::result::{DirectoryEntry, FindError};

type FReturn = Result<Vec<DirectoryEntry>, FindError>;

/// This function lists all the paths from a base path
/// and returns a `DirectoryEntry` enum whether the path
/// could be read or not, if the `ignore_error` argument
/// is true you will not find any `DirectoryEntry::Error`
/// in the returned Vec.
#[allow(unused)]
pub fn find(base: &Path, ignore_error: bool) -> FReturn {
    find_recursive(base, &None, ignore_error)
}

/// This function lists all the paths from a base path
/// that match a glob, and filters the output using the
/// globset spec, if the `ignore_error` argument is true
/// you will not found any `DirectoryEntry::Error`
/// in the returned Vec.
#[allow(unused)]
pub fn find_glob(base: &Path, glob: &impl ToString, ignore_error: bool) -> FReturn {
    find_recursive(
        base,
        &Some(Glob::new(&glob.to_string())?.compile_matcher()),
        ignore_error
    )
}

pub fn find_recursive(base: &Path, glob: &Option<GlobMatcher>, ignore_error: bool) -> FReturn {
    if !base.is_dir() {
        return Err(
            FindError::NotADirectory(
                format!("{}", base.to_string_lossy())
            )
        );
    }

    Ok(
        read_dir(base)?
            .map(|entry| entry
                .map_or_else(
                    |err| vec![DirectoryEntry::Error(err.to_string())],
                    |entry| {
                        let path = entry.path();
                        let entry_result = vec![DirectoryEntry::Ok(path.to_owned())];
                        if path.is_dir() && !path.is_symlink() {
                            find_recursive(&path, glob, ignore_error)
                                .map_or_else(
                                    |err| if ignore_error {
                                        Vec::new()
                                    } else {
                                        vec![DirectoryEntry::Error(err.to_string())]
                                    },
                                    |sub_entries| {
                                        let mut all_entries = entry_result;
                                        all_entries.extend(sub_entries);
                                        all_entries
                                    },
                                )
                        } else {
                            entry_result
                        }
                    }
                )
            )
            .flatten()
            .filter(|path| match glob {
                Some(glob) => match path {
                    DirectoryEntry::Ok(entry) => glob.is_match(
                        &entry
                            .to_string_lossy()
                            .to_string()
                    ),
                    DirectoryEntry::Error(_) => true
                },
                None => false
            })
            .collect()
    )
}
