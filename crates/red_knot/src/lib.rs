use std::fmt::Formatter;
use std::ops::Deref;

use rustc_hash::FxHashSet;

pub(crate) use db::{Db, Jar};
use ruff_db::file_system::{FileSystemPath, FileSystemPathBuf};
use ruff_db::vfs::VfsFile;

mod db;
mod lint;
mod program;
mod typeshed_versions;
mod watch;

#[derive(Debug, Clone)]
pub struct Workspace {
    /// TODO this should be a resolved path. We should probably use a newtype wrapper that guarantees that
    /// PATH is a UTF-8 path and is normalized.
    root: FileSystemPathBuf,
    /// The files that are open in the workspace.
    ///
    /// * Editor: The files that are actively being edited in the editor (the user has a tab open with the file).
    /// * CLI: The resolved files passed as arguments to the CLI.
    open_files: FxHashSet<VfsFile>,
}

impl Workspace {
    pub fn new(root: FileSystemPathBuf) -> Self {
        Self {
            root,
            open_files: FxHashSet::default(),
        }
    }

    pub fn root(&self) -> &FileSystemPath {
        self.root.as_path()
    }

    // TODO having the content in workspace feels wrong.
    pub fn open_file(&mut self, file_id: VfsFile) {
        self.open_files.insert(file_id);
    }

    pub fn close_file(&mut self, file_id: VfsFile) {
        self.open_files.remove(&file_id);
    }

    // TODO introduce an `OpenFile` type instead of using an anonymous tuple.
    pub fn open_files(&self) -> impl Iterator<Item = VfsFile> + '_ {
        self.open_files.iter().copied()
    }

    pub fn is_file_open(&self, file_id: VfsFile) -> bool {
        self.open_files.contains(&file_id)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Name(smol_str::SmolStr);

impl Name {
    #[inline]
    pub fn new(name: &str) -> Self {
        Self(smol_str::SmolStr::new(name))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Deref for Name {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<T> From<T> for Name
where
    T: Into<smol_str::SmolStr>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
