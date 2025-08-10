use crate::core::utils::link;

use super::config::Config;
use std::fs;

/// Switch JAVA_HOME environment variable.
/// 
/// This function switches the JAVA_HOME environment variable to point to a specific JDK version.
/// It creates a symbolic link from the `java_home` directory to the specified JDK directory.
/// If a symbolic link or directory already exists at `java_home`, it will be removed before creating the new link.
/// 
/// # Arguments
/// 
/// * `jdk` - The JDK version to switch to.
/// * `cfg` - The configuration object containing paths.
/// 
/// # Returns
/// 
/// * `Ok(true)` if the switch was successful.
/// * `Ok(false)` if the specified JDK path does not exist.
/// * `Err` if any file system operation fails.
/// 
/// # Examples
/// 
/// ```
/// use jvm::func::config::Config;
/// use jvm::func::switch::switch_jdk;
/// 
/// let cfg = Config::load().expect("Failed to load config");
/// let result = switch_jdk("oracle@24.0.1", &cfg);
/// assert!(result.is_ok());
/// ```
pub fn switch_jdk(jdk: &str, cfg: &Config) -> Result<bool, Box<dyn std::error::Error>> {
    let java_home = &cfg.java_home;
    let jdk_path = cfg.jdk_home.join(jdk);
    
    // Check if the JDK path exists
    if !jdk_path.exists() {
        return Ok(false);
    }
    
    // 清理现有的 java_home 链接或目录
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
            e
        })?;
    }
    
    // 创建符号链接（自动处理权限问题）
    if let Err(e) = link(&jdk_path, java_home) {
        eprintln!("Create link failed: {}", e);
        return Err(e);
    }
    
    // Update the configuration and save it
    let new_cfg = cfg.change_jdk(jdk);
    new_cfg.save()?;
    
    Ok(true)
}