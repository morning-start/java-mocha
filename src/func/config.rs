use std::collections::HashMap;
use std::path::PathBuf;
use std::env;
use std::fs;
use std::fmt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_json::Map;
use crate::core::utils::load_json;

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub jvm_root: PathBuf,
    pub jdk_home: PathBuf,
    pub java_home: PathBuf,
    pub cache_home: PathBuf,
    pub data_dir: PathBuf,
    #[serde(default)]
    pub jdk_version: String,
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = self.to_json();
        write!(f, "Config({})", json)
    }
}

impl Config {
    /// 加载 JVM 根目录路径
    pub fn load_jvm() -> PathBuf {
        // 获取用户主目录
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let mut jvm_root = home_dir.join(".java-mocha");
        
        // 检查环境变量 JVM_ROOT
        if let Ok(env_jvm_root) = env::var("JVM_ROOT") {
            jvm_root = PathBuf::from(env_jvm_root);
        }
        
        jvm_root
    }
    
    /// 从 JSON 数据创建 Config 实例
    pub fn from_json(json: &Value) -> Result<Self, Box<dyn std::error::Error>> {
        let jvm_root = PathBuf::from(json["jvm_root"].as_str().unwrap_or(""));
        let jdk_home = PathBuf::from(json["jdk_home"].as_str().unwrap_or(""));
        let java_home = PathBuf::from(json["java_home"].as_str().unwrap_or(""));
        let cache_home = PathBuf::from(json["cache_home"].as_str().unwrap_or(""));
        let data_dir = PathBuf::from(json["data_dir"].as_str().unwrap_or(""));
        let jdk_version = json["jdk_version"].as_str().unwrap_or("").to_string();
        
        Ok(Self {
            jvm_root,
            jdk_home,
            java_home,
            cache_home,
            data_dir,
            jdk_version,
        })
    }
    
    /// 从 JVM 根目录加载配置
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_file = Self::load_jvm().join("config.json");
        let cfg = load_json(config_file.to_str().unwrap())?;
        Self::from_json(&cfg)
    }
    
    /// 初始化路径目录
    pub fn init_path(&self) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(&self.jdk_home)?;
        fs::create_dir_all(&self.cache_home)?;
        fs::create_dir_all(&self.data_dir)?;
        Ok(())
    }
    
    /// 保存配置到 JVM 根目录
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_file = self.jvm_root.join("config.json");
        let json_value = serde_json::to_value(self)?;
        let mut file = fs::File::create(config_file)?;
        serde_json::to_writer_pretty(&mut file, &json_value)?;
        Ok(())
    }
    
    /// 更改 JDK 版本
    pub fn change_jdk(&self, jdk_version: &str) -> Self {
        let mut new_config = self.clone();
        new_config.jdk_version = jdk_version.to_string();
        new_config
    }
    
    /// 将配置转换为映射表
    pub fn to_dict(&self) -> Map<String, Value> {
        let mut map = Map::new();
        map.insert("jvm_root".to_string(), Value::String(self.jvm_root.to_string_lossy().to_string()));
        map.insert("jdk_home".to_string(), Value::String(self.jdk_home.to_string_lossy().to_string()));
        map.insert("java_home".to_string(), Value::String(self.java_home.to_string_lossy().to_string()));
        map.insert("cache_home".to_string(), Value::String(self.cache_home.to_string_lossy().to_string()));
        map.insert("data_dir".to_string(), Value::String(self.data_dir.to_string_lossy().to_string()));
        map.insert("jdk_version".to_string(), Value::String(self.jdk_version.clone()));
        map
    }
    
    /// 将配置转换为 JSON 字符串
    pub fn to_json(&self) -> String {
        let map = self.to_dict();
        serde_json::to_string(&map).unwrap_or_default()
    }
}

/// 配置 Java Mocha 的 JVM 根目录、JDK 目录和缓存目录。
/// 
/// # Parameters
/// 
/// * `jvm_root` - JVM 根目录，默认为用户主目录下的 `.java-mocha` 目录。
/// * `jdk_home` - JDK 目录，默认为 JVM 根目录下的 `jdk` 目录。
/// * `java_home` - JAVA_HOME 环境变量，默认为 JVM 根目录下的 `default` 目录。
/// * `cache_home` - 缓存目录，默认为 JVM 根目录下的 `cache` 目录。
pub fn init_config(
    jvm_root: PathBuf,
    jdk_home: Option<PathBuf>,
    java_home: Option<PathBuf>,
    cache_home: Option<PathBuf>,
) -> Result<Config, Box<dyn std::error::Error>> {
    // 默认值
    let mut cfg_dict = HashMap::new();
    cfg_dict.insert("jvm_root".to_string(), jvm_root.to_string_lossy().to_string());
    cfg_dict.insert("jdk_home".to_string(), jvm_root.join("jdk").to_string_lossy().to_string());
    cfg_dict.insert("java_home".to_string(), jvm_root.join("default").to_string_lossy().to_string());
    cfg_dict.insert("cache_home".to_string(), jvm_root.join("cache").to_string_lossy().to_string());
    cfg_dict.insert("data_dir".to_string(), jvm_root.join("data").to_string_lossy().to_string());
    cfg_dict.insert("jdk_version".to_string(), "".to_string());

    // 如果有config
    let config_file = jvm_root.join("config.json");
    if config_file.exists() {
        let existing_config = Config::load()?;
        let existing_dict = existing_config.to_dict();
        
        // 更新cfg_dict中的值
        for (key, value) in existing_dict {
            if let serde_json::Value::String(str_value) = value {
                cfg_dict.insert(key, str_value);
            }
        }
    }

    // 根据参数更新
    if let Some(jdk_home_val) = jdk_home {
        cfg_dict.insert("jdk_home".to_string(), jdk_home_val.to_string_lossy().to_string());
    }
    
    if let Some(java_home_val) = java_home {
        cfg_dict.insert("java_home".to_string(), java_home_val.to_string_lossy().to_string());
    }
    
    if let Some(cache_home_val) = cache_home {
        cfg_dict.insert("cache_home".to_string(), cache_home_val.to_string_lossy().to_string());
    }
    

    // 创建配置对象
    let cfg = Config {
        jvm_root: PathBuf::from(cfg_dict.get("jvm_root").unwrap().as_str()),
        jdk_home: PathBuf::from(cfg_dict.get("jdk_home").unwrap().as_str()),
        java_home: PathBuf::from(cfg_dict.get("java_home").unwrap().as_str()),
        cache_home: PathBuf::from(cfg_dict.get("cache_home").unwrap().as_str()),
        data_dir: PathBuf::from(cfg_dict.get("data_dir").unwrap().as_str()),
        jdk_version: cfg_dict.get("jdk_version").unwrap().clone(),
    };
    
    // 初始化路径
    cfg.init_path()?;
    
    // 保存配置
    cfg.save()?;
    
    Ok(cfg)
}