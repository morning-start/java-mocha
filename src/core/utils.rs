use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn save_json(json: &Value, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(file_path)?;
    file.write_all(serde_json::to_string_pretty(json)?.as_bytes())?;
    Ok(())
}

pub fn load_json(file_path: &Path) -> Result<Value, Box<dyn std::error::Error>> {
    let json = serde_json::from_str(&std::fs::read_to_string(file_path)?)?;
    Ok(json)
}

#[derive(Debug, Clone, Default)]
pub struct UrlParams {
    inner: HashMap<String, String>,
}

impl UrlParams {
    // 构造函数
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: &str, value: &dyn ToString) -> Option<String> {
        self.inner.insert(key.to_string(), value.to_string())
    }
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.inner.remove(key)
    }
    pub fn set(&mut self, key: &str, value: &dyn ToString) -> Option<String> {
        self.inner.insert(key.to_string(), value.to_string())
    }
    pub fn get(&self, key: &str) -> Option<&String> {
        self.inner.get(key)
    }
    pub fn add_iterable<I, V: ToString>(&mut self, iterable: I, key: &str)
    where
        I: IntoIterator<Item = V>,
    {
        iterable.into_iter().for_each(|v| {
            self.inner.insert(key.to_string(), v.to_string());
        });
    }
    pub fn add_optional<T: ToString>(&mut self, key: &str, value: Option<T>) {
        if value.is_some() {
            self.inner
                .insert(key.to_string(), value.unwrap().to_string());
        }
    }
}
impl UrlParams {
    // 从HashMap初始化
    pub fn from_hashmap(map: HashMap<String, String>) -> Self {
        Self { inner: map }
    }

    // 转换回HashMap
    pub fn into_hashmap(self) -> HashMap<String, String> {
        self.inner
    }

    // 批量插入
    pub fn extend(&mut self, other: impl IntoIterator<Item = (String, String)>) {
        self.inner.extend(other);
    }
}
