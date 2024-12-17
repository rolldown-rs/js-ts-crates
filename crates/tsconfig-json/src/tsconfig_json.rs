use crate::compiler_options::CompilerOptions;
use crate::path_types::*;
use clean_path::Clean;
use rustc_hash::FxHashMap;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::{fs, io};

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
pub struct TsConfigJson {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extends: Option<ExtendsField>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub compiler_options: Option<CompilerOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<CompilerPath>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<CompilerPath>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<CompilerPath>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<Vec<ProjectReference>>,

    // For all other fields we don't want to explicitly support,
    // but consumers may want to access for some reason
    #[serde(flatten)]
    pub other_fields: FxHashMap<String, serde_json::Value>,
}

// https://www.typescriptlang.org/docs/handbook/release-notes/typescript-5-0.html#supporting-multiple-configuration-files-in-extends
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(untagged)]
pub enum ExtendsField {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, PartialEq)]
pub struct TsConfigExtendsChain {
    pub path: PathBuf,
    pub config: TsConfigJson,
}

impl TsConfigJson {
    pub fn expand(&mut self, source_dir: &Path, target_dir: &Path) {
        if let Some(options) = &mut self.compiler_options {
            options.expand(source_dir, target_dir);
        }

        if let Some(include) = &mut self.include {
            for path in include.iter_mut() {
                path.expand(source_dir, target_dir);
            }
        }

        if let Some(exclude) = &mut self.exclude {
            for path in exclude.iter_mut() {
                path.expand(source_dir, target_dir);
            }
        }

        if let Some(files) = &mut self.files {
            for path in files.iter_mut() {
                path.expand(source_dir, target_dir);
            }
        }

        if let Some(references) = &mut self.references {
            for reference in references.iter_mut() {
                reference.path.expand(source_dir, target_dir);
            }
        }
    }

    pub fn extend(&mut self, other: TsConfigJson) {
        if let Some(value) = other.compiler_options {
            self.compiler_options
                .get_or_insert(Default::default())
                .extend(value);
        }

        if let Some(value) = other.include {
            self.include = Some(value);
        }

        if let Some(value) = other.exclude {
            self.exclude = Some(value);
        }

        if let Some(value) = other.files {
            self.files = Some(value);
        }

        // These aren't extendable, so always overwrite with the
        // other value, even when `None`
        self.extends = other.extends;
        self.references = other.references;

        self.other_fields.extend(other.other_fields);
    }

    pub fn resolve_path_in_node_modules<N: AsRef<str>, D: AsRef<Path>>(
        package_file: N,
        starting_dir: D,
    ) -> Option<PathBuf> {
        let package_file = package_file.as_ref();
        let mut current_dir = Some(starting_dir.as_ref());

        while let Some(dir) = current_dir {
            let file_path = if package_file.ends_with(".json") {
                dir.join("node_modules").join(package_file)
            } else {
                dir.join("node_modules")
                    .join(package_file)
                    .join("tsconfig.json")
            };

            if file_path.exists() {
                return Some(file_path);
            }

            current_dir = dir.parent();
        }

        None
    }

    pub fn resolve_extends_chain<T: AsRef<Path>>(path: T) -> io::Result<Vec<TsConfigExtendsChain>> {
        let mut chain = vec![];

        resolve_extends_chain_deep(path.as_ref().to_owned(), &mut chain)?;

        // Reverse so that the base file is the 0-index,
        // and the files that overwrite it come next
        chain.reverse();

        Ok(chain)
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct ProjectReference {
    pub path: CompilerPath,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepend: Option<bool>,
}

fn resolve_extends_chain_deep(
    path: PathBuf,
    chain: &mut Vec<TsConfigExtendsChain>,
) -> io::Result<()> {
    let parent_dir = path.parent().unwrap();
    let config: TsConfigJson = serde_json::from_slice(&fs::read(&path)?)?;
    let mut inner_chain = vec![];

    if let Some(extends) = &config.extends {
        for extends_from in match extends {
            ExtendsField::Single(other) => vec![other],
            ExtendsField::Multiple(others) => others.iter().rev().collect(),
        } {
            // File path
            if extends_from.starts_with('.') {
                resolve_extends_chain_deep(
                    if extends_from.ends_with(".json") {
                        parent_dir.join(extends_from)
                    } else {
                        parent_dir.join(extends_from).join("tsconfig.json")
                    },
                    &mut inner_chain,
                )?;
            }
            // Node module
            else if let Some(package_path) =
                TsConfigJson::resolve_path_in_node_modules(extends_from, parent_dir)
            {
                resolve_extends_chain_deep(package_path, &mut inner_chain)?;
            }
        }
    }

    chain.push(TsConfigExtendsChain {
        path: path.clean(),
        config,
    });
    chain.extend(inner_chain);

    Ok(())
}
