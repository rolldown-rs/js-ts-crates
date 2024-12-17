use clean_path::Clean;
use relative_path::RelativePathBuf;
use serde::Deserialize;
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "lowercase")]
pub enum PathKind {
    #[default]
    Path,
    Glob,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(from = "String", into = "String")]
pub struct CompilerPath {
    pub kind: PathKind,
    pub path: RelativePathBuf,
    pub expanded_path: PathBuf,
}

impl CompilerPath {
    pub fn expand(&mut self, source_dir: &Path, target_dir: &Path) {
        self.expanded_path = if let Ok(rel_path) = self.path.strip_prefix("${configDir}") {
            rel_path.to_logical_path(target_dir)
        } else {
            self.path.to_logical_path(source_dir)
        }
        .clean();
    }
}

impl From<&str> for CompilerPath {
    fn from(value: &str) -> Self {
        let kind = if value.contains('*') || value.contains('?') {
            PathKind::Glob
        } else {
            PathKind::Path
        };

        Self {
            kind,
            path: RelativePathBuf::from(value),
            expanded_path: PathBuf::new(),
        }
    }
}

impl From<String> for CompilerPath {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<CompilerPath> for String {
    fn from(value: CompilerPath) -> String {
        value.to_string()
    }
}

impl fmt::Display for CompilerPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path.as_str().replace('\\', "/"))
    }
}

impl Deref for CompilerPath {
    type Target = RelativePathBuf;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl DerefMut for CompilerPath {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.path
    }
}
