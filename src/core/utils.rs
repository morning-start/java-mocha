use serde_json::Value;
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


// 创建系统链接，适配多个系统
pub fn link(link_name: &Path, target: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if link_name.exists() {
        std::fs::remove_file(link_name)?;
    }
    #[cfg(unix)]
    std::os::unix::fs::symlink(target, link_name)?;
    #[cfg(windows)]
    std::os::windows::fs::symlink_dir(target, link_name)?;
    Ok(())
}


#[derive(Debug, Default, Clone)]
pub struct UrlParams {
    pub params: Vec<(String, String)>,
}

impl UrlParams {
    /// 创建一个新的空的 UrlParams 实例
    pub fn new() -> Self {
        Self { params: Vec::new() }
    }

    /// 添加一个键值对到查询参数中
    /// 允许同一个键出现多次
    pub fn add(&mut self, key: &str, value: &dyn ToString) {
        let value_str = value.to_string();
        self.params.push((key.to_string(), value_str));
    }

    /// 添加一个可迭代集合的所有元素作为值，使用指定的键
    /// 集合中的每个元素都会被转换为字符串并作为独立的键值对添加
    pub fn add_iterable<I, V: ToString>(&mut self, iterable: I, key: &str)
    where
        I: IntoIterator<Item = V>,
    {
        for item in iterable {
            self.add(key, &item);
        }
    }

    /// 有条件地添加一个值到查询参数中
    /// 只有当提供的 `Option<T>` 是 `Some(value)` 时才会添加
    pub fn add_optional<T: ToString>(&mut self, key: &str, value: Option<T>) {
        if let Some(val) = value {
            self.add(key, &val);
        }
    }
    /// 返回参数的数量
    pub fn len(&self) -> usize {
        self.params.len()
    }

    /// 检查是否没有任何参数
    pub fn is_empty(&self) -> bool {
        self.params.is_empty()
    }

    /// 清空所有参数
    pub fn clear(&mut self) {
        self.params.clear();
    }
}
