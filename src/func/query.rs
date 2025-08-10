use crate::core::datatype::{DataFile, SupportTerm};
use crate::core::handler::DocumentHandler;
use serde_json::Value;
use std::error::Error;
use std::{collections::HashMap, path::Path};

pub fn query_info(data_dir: &Path, publisher: &str) -> Result<Vec<Value>, Box<dyn Error>> {
    let handler = DocumentHandler::load_data(&data_dir.join(DataFile::Packages.as_ref()))?;

    let fields = [
        "distribution",
        "major_version",
        "term_of_support",
        "latest_build_available",
        "distribution_version",
    ];

    let mut data = handler.get_specific_fields(&fields)?;

    // Filter data
    data = data.filter(|x| {
        x.as_object()
            .and_then(|obj| obj.get("distribution"))
            .and_then(|v| v.as_str())
            .map(|dist| dist == publisher)
            .unwrap_or(false)
            && x.as_object()
                .and_then(|obj| obj.get("latest_build_available"))
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
    })?;

    // Rename fields
    let mut name_map = HashMap::new();
    name_map.insert("distribution".to_string(), "publisher".to_string());
    name_map.insert("major_version".to_string(), "major".to_string());
    name_map.insert("term_of_support".to_string(), "term".to_string());
    name_map.insert("latest_build_available".to_string(), "latest".to_string());
    name_map.insert("distribution_version".to_string(), "version".to_string());

    data.rename(&name_map)?;

    Ok(data.document().as_array().unwrap().to_vec())
}

pub fn query_info_version(
    data_dir: &Path,
    publisher: &str,
    major_version: i32,
) -> Result<Vec<Value>, Box<dyn Error>> {
    let handler = DocumentHandler::load_data(&data_dir.join(DataFile::Packages.as_ref()))?;

    let fields = [
        "distribution",
        "major_version",
        "term_of_support",
        "latest_build_available",
        "distribution_version",
    ];

    let mut data = handler.get_specific_fields(&fields)?;

    // Filter data
    data = data.filter(|x| {
        x.as_object()
            .and_then(|obj| obj.get("distribution"))
            .and_then(|v| v.as_str())
            .map(|dist| dist == publisher)
            .unwrap_or(false)
            && x.as_object()
                .and_then(|obj| obj.get("major_version"))
                .and_then(|v| v.as_i64())
                .map(|ver| ver == major_version as i64)
                .unwrap_or(false)
    })?;

    // Rename fields
    let mut name_map = HashMap::new();
    name_map.insert("distribution".to_string(), "publisher".to_string());
    name_map.insert("major_version".to_string(), "major".to_string());
    name_map.insert("term_of_support".to_string(), "term".to_string());
    name_map.insert("latest_build_available".to_string(), "latest".to_string());
    name_map.insert("distribution_version".to_string(), "Version".to_string());

    data.rename(&name_map)?;
    // publisher ┃ major ┃ term ┃ latest ┃ Version
    let res = data.orderby(&["publisher", "major", "term", "latest", "Version"]).unwrap();
    Ok(res)
}

pub fn query_info_term(
    data_dir: &Path,
    publisher: &str,
    term_of_support: SupportTerm,
) -> Result<Vec<Value>, Box<dyn Error>> {
    let handler = DocumentHandler::load_data(&data_dir.join(DataFile::Packages.as_ref()))?;

    let fields = [
        "distribution",
        "major_version",
        "term_of_support",
        "latest_build_available",
        "distribution_version",
    ];

    let mut data = handler.get_specific_fields(&fields)?;

    // Filter data
    data = data.filter(|x| {
        x.as_object()
            .and_then(|obj| obj.get("distribution"))
            .and_then(|v| v.as_str())
            .map(|dist| dist == publisher)
            .unwrap_or(false)
            && x.as_object()
                .and_then(|obj| obj.get("term_of_support"))
                .and_then(|v| v.as_str())
                .map(|term| term == term_of_support.as_ref())
                .unwrap_or(false)
            && x.as_object()
                .and_then(|obj| obj.get("latest_build_available"))
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
    })?;

    // Rename fields
    let mut name_map = HashMap::new();
    name_map.insert("distribution".to_string(), "publisher".to_string());
    name_map.insert("major_version".to_string(), "major".to_string());
    name_map.insert("term_of_support".to_string(), "term".to_string());
    name_map.insert("latest_build_available".to_string(), "latest".to_string());
    name_map.insert("distribution_version".to_string(), "Version".to_string());

    data.rename(&name_map)?;
    let res = data.orderby(&["publisher", "major", "term", "latest", "Version"]).unwrap();

    Ok(res)
}
