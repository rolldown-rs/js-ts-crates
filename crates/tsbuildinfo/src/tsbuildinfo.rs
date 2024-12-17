// Based on `IncrementalBuildInfoBase` type.
// https://github.com/microsoft/TypeScript/blob/main/src/compiler/builder.ts#L1129
// And the `getBuildInfo` function.
// https://github.com/microsoft/TypeScript/blob/main/src/compiler/builder.ts#L1234

use rustc_hash::FxHashMap;
use serde::Deserialize;
use std::path::PathBuf;

pub type FileId = u16;
pub type FileIdPair = (u16, u16);

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(default, rename_all = "camelCase")]
pub struct TsBuildInfoProgram {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_file_set: Option<Vec<FileId>>,

    pub check_pending: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub emit_diagnostics_per_file: Option<Vec<DiagnosticField>>,

    pub errors: bool,

    pub exported_modules_map: Vec<FileIdPair>,

    pub file_ids_list: Vec<Vec<FileId>>,

    pub file_infos: Vec<FileInfoField>,

    pub file_names: Vec<PathBuf>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_changed_dts_file: Option<PathBuf>,

    // The shape of these options is *not* the same as the
    // struct in the `typescript_tsconfig_json` crate!
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_root: Option<Vec<FileIdPair>>,

    pub root: Vec<RootField>,

    pub referenced_map: Vec<FileIdPair>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_diagnostics_per_file: Option<Vec<DiagnosticField>>,

    #[serde(flatten)]
    pub other_fields: FxHashMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(default, rename_all = "camelCase")]
pub struct TsBuildInfo {
    pub program: TsBuildInfoProgram,
    pub version: String,
}

impl TsBuildInfo {
    pub fn get_file_from_id(&self, id: FileId) -> PathBuf {
        self.program
            .file_names
            .get((id - 1) as usize)
            .unwrap()
            .to_path_buf()
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(untagged)]
pub enum DiagnosticField {
    File(FileId),
    FileDiagnostic(FileId, serde_json::Value),
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(untagged, rename_all_fields = "camelCase")]
pub enum FileInfoField {
    Hash(String),

    Info {
        #[serde(default)]
        affects_global_scope: bool,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        signature: Option<FileInfoSignatureField>,

        version: String,
    },
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(untagged)]
pub enum FileInfoSignatureField {
    Bool(bool),
    String(String),
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(untagged)]
pub enum RootField {
    Pair(FileId, FileId),
    Single(FileId),
}
