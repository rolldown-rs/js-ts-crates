// https://github.com/microsoft/TypeScript/blob/7976d9cef5be76372e610c28806fca351ab5d70f/src/compiler/tracing.ts#L332

use serde::Deserialize;
use std::ops::Deref;
use std::path::PathBuf;

pub type TypeId = u32;

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
pub struct TypesLocationSpan {
    pub line: u32,
    pub character: u32,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
pub struct TypesLocation {
    pub path: PathBuf,
    pub start: TypesLocationSpan,
    pub end: TypesLocationSpan,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[serde(default, rename_all = "camelCase")]
pub struct TypesEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_type_arguments: Option<Vec<TypeId>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_check_type: Option<TypeId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_extends_type: Option<TypeId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_true_type: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_false_type: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_type: Option<TypeId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub destructuring_pattern: Option<TypesLocation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub evolving_array_element_type: Option<TypeId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub evolving_array_final_type: Option<TypeId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_declaration: Option<TypesLocation>,

    pub flags: Vec<String>,

    pub id: TypeId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexed_access_index_type: Option<TypeId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexed_access_object_type: Option<TypeId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub instantiated_type: Option<TypeId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub intersection_types: Option<Vec<TypeId>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_tuple: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrinsic_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyof_type: Option<TypeId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursion_id: Option<TypeId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_location: Option<TypesLocation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_mapped_constraint_type: Option<TypeId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_mapped_mapped_type: Option<TypeId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_mapped_source_type: Option<TypeId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitution_base_type: Option<TypeId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_arguments: Option<Vec<TypeId>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub union_types: Option<Vec<TypeId>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct TypesJson(pub Vec<TypesEvent>);

impl Deref for TypesJson {
    type Target = Vec<TypesEvent>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
