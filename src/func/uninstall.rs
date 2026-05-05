use crate::core::converter::{CliInputConverter, InputConverter};
use std::fs::{remove_dir_all, remove_file};
use crate::func::config::Config;

pub fn uninstall_jdk(jdk: &str, cfg: &Config) -> bool {
    let jdk_version = match CliInputConverter::convert(jdk.to_string()) {
        Ok(parsed) => format!("{}@{}", parsed.distribution.as_ref(), format_version(&parsed.version_spec)),
        Err(_) => jdk.to_string()
    };
    
    let jdk_path = cfg.jdk_home.join(&jdk_version);
    perform_uninstall(&jdk_path)
}

fn perform_uninstall(jdk_path: &std::path::Path) -> bool {
    if jdk_path.exists() {
        let result = if jdk_path.is_dir() {
            remove_dir_all(jdk_path)
        } else {
            remove_file(jdk_path)
        };
        result.is_ok()
    } else {
        false
    }
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