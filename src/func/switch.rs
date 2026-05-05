use crate::core::converter::{CliInputConverter, InputConverter};
use crate::core::utils::link;
use crate::func::config::Config;
use std::fs;

pub fn switch_jdk(jdk: &str, cfg: &Config) -> Result<bool, String> {
    let parsed = CliInputConverter::convert(jdk.to_string())?;
    let jdk_version = format!("{}@{}", parsed.distribution.as_ref(), format_version(&parsed.version_spec));
    
    let java_home = &cfg.java_home;
    let jdk_path = cfg.jdk_home.join(&jdk_version);

    if !jdk_path.exists() {
        return Ok(false);
    }

    if java_home.exists() {
        let result = if java_home.is_dir() {
            fs::remove_dir_all(java_home)
        } else {
            fs::remove_file(java_home)
        };

        result.map_err(|e| {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                eprintln!("Remove existing path failed: {}。Please check permissions.", e);
            }
            format!("Failed to remove existing path: {}", e)
        })?;
    }

    if let Err(e) = link(&jdk_path, java_home) {
        eprintln!("Create link failed: {}", e);
        return Err(format!("Failed to create link: {}", e));
    }

    let new_cfg = cfg.change_jdk(&jdk_version);
    new_cfg.save().map_err(|e| format!("Failed to save config: {}", e))?;

    Ok(true)
}

fn format_version(spec: &crate::core::state::VersionSpec) -> String {
    match spec {
        crate::core::state::VersionSpec::Latest => "latest".to_string(),
        crate::core::state::VersionSpec::LTS => "lts".to_string(),
        crate::core::state::VersionSpec::STS => "sts".to_string(),
        crate::core::state::VersionSpec::MTS => "mts".to_string(),
        crate::core::state::VersionSpec::Major(m) => m.to_string(),
        crate::core::state::VersionSpec::Exact(s) => s.clone(),
        crate::core::state::VersionSpec::EA => "ea".to_string(),
    }
}