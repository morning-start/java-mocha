use std::fs::{read_to_string, write};
use std::path::Path;
use crate::func::config::Config;
use crate::func::switch::switch_jdk;

/// 读取项目级 Java 版本配置文件
///
/// # Arguments
/// * `path` - 目录路径，将在该目录及其父目录中查找配置文件
///
/// # Returns
/// * `Option<String>` - 找到的 Java 版本，格式为 "publisher@version"
pub fn read_local_version(path: &Path) -> Option<String> {
    // 尝试在当前目录及其父目录中查找 .jvmrc 或 .java-version 文件
    let mut current_path = path;
    while current_path.parent().is_some() {
        // 检查 .jvmrc 文件
        let jvmrc_path = current_path.join(".jvmrc");
        if jvmrc_path.exists() {
            if let Ok(content) = read_to_string(&jvmrc_path) {
                let version = content.trim().to_string();
                if !version.is_empty() {
                    return Some(version);
                }
            }
        }
        
        // 检查 .java-version 文件
        let java_version_path = current_path.join(".java-version");
        if java_version_path.exists() {
            if let Ok(content) = read_to_string(&java_version_path) {
                let version = content.trim().to_string();
                if !version.is_empty() {
                    return Some(version);
                }
            }
        }
        
        // 移动到父目录
        current_path = current_path.parent().unwrap();
    }
    
    None
}

/// 写入项目级 Java 版本配置文件
///
/// # Arguments
/// * `path` - 目录路径，将在该目录中创建配置文件
/// * `version` - Java 版本，格式为 "publisher@version"
///
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - 写入结果
pub fn write_local_version(path: &Path, version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let jvmrc_path = path.join(".jvmrc");
    write(&jvmrc_path, version)?;
    Ok(())
}

/// 检测并应用项目级 Java 版本
///
/// # Arguments
/// * `cfg` - 配置信息
///
/// # Returns
/// * `Result<Option<String>, Box<dyn std::error::Error>>` - 应用的 Java 版本，如果没有找到则返回 None
pub fn detect_and_apply_local_version(cfg: &Config) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir()?;
    if let Some(local_version) = read_local_version(&current_dir) {
        // 尝试切换到项目级版本
        match switch_jdk(&local_version, cfg) {
            Ok(true) => Ok(Some(local_version)),
            _ => Ok(None),
        }
    } else {
        Ok(None)
    }
}

/// 设置项目级 Java 版本
///
/// # Arguments
/// * `version` - Java 版本，格式为 "publisher@version"
///
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - 设置结果
pub fn set_local_version(version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir()?;
    write_local_version(&current_dir, version)?;
    Ok(())
}
