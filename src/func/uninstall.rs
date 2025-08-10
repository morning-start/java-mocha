use std::fs::{remove_dir_all, remove_file};
use crate::func::config::Config;

/// 卸载JDK
///
/// # Arguments
/// * `jdk` - 要卸载的JDK版本
/// * `cfg` - 配置信息
///
/// # Returns
/// * `bool` - 如果成功卸载则返回true，否则返回false
pub fn uninstall_jdk(jdk: &str, cfg: &Config) -> bool {
    let jdk_path = cfg.jdk_home.join(jdk);
    
    if jdk_path.exists() {
        let result = if jdk_path.is_dir() {
            remove_dir_all(&jdk_path)
        } else {
            remove_file(&jdk_path)
        };
        
        result.is_ok()
    } else {
        false
    }
}