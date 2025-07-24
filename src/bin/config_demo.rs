use jvm::func::config::Config;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // 加载 JVM 根目录
    let jvm_root = Config::load_jvm();
    println!("JVM 根目录: {:?}", jvm_root);

    // 创建配置实例
    let config = Config {
        jvm_root: jvm_root.clone(),
        jdk_home: jvm_root.join("jdk"),
        java_home: jvm_root.join("java"),
        cache_home: jvm_root.join("cache"),
        data_dir: jvm_root.join("data"),
        proxy: "".to_string(),
        jdk_version: "oracle@21.0.7".to_string(),
    };

    // 初始化路径目录
    config.init_path()?;
    println!("路径目录初始化完成");

    // 保存配置
    config.save()?;
    println!("配置已保存");

    // 显示配置的调试信息
    println!("{:#?}", config);

    // 转换为字典
    let dict = config.to_dict();
    println!("配置字典: {:?}", dict);

    // 转换为 JSON
    let json = config.to_json();
    println!("配置 JSON: {}", json);

    // 加载配置
    let loaded_config = Config::load()?;
    println!("加载的配置: {:#?}", loaded_config);

    // 更改 JDK 版本
    let updated_config = loaded_config.change_jdk("oracle@24.0.1");
    updated_config.save()?;
    println!("更新后的配置: {:#?}", updated_config);

    Ok(())
}