use jvm::func::config::init_config;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 获取JVM根目录
    let jvm_root = PathBuf::from(".java-mocha");
    
    // 初始化配置
    let config = init_config(
        jvm_root,
        None,  // 使用默认值
        None,  // 使用默认值
        None,  // 使用默认值
        None,  // 使用默认值
    )?;
    
    println!("配置初始化成功: {:#?}", config);
    
    // 使用自定义路径初始化配置
    let config2 = init_config(
        PathBuf::from(".java-mocha-custom"),
        Some(PathBuf::from(".java-mocha-custom/jdk")),
        Some(PathBuf::from(".java-mocha-custom/default")),
        Some(PathBuf::from(".java-mocha-custom/cache")),
        Some("http://proxy.example.com:8080".to_string()),
    )?;
    
    println!("自定义配置初始化成功: {:#?}", config2);
    
    Ok(())
}