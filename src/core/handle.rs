use super::utils::load_json; // 引用项目中已有的JSON加载函数
use serde_json::Value;
use std::{option::Option, path::Path};

/// JSON文档处理器，提供加载、查询和字段重命名功能
#[derive(Debug, Clone)]
pub struct DocumentHandler {
    document: Value,
}

impl DocumentHandler {
    /// 创建新的文档处理器实例
    pub fn new(document: Value) -> Self {
        Self { document }
    }

    /// 从JSON文件加载数据并创建处理器实例
    pub fn load_data(file_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let document = load_json(file_path)?;
        Ok(Self::new(document))
    }

    /// 获取当前文档的不可变引用
    pub fn document(&self) -> &Value {
        &self.document
    }

    /// 获取当前文档的可变引用
    pub fn document_mut(&mut self) -> &mut Value {
        &mut self.document
    }
}

impl DocumentHandler {
    /// 根据键值对查询数据，返回新的处理器实例
    pub fn query(
        &self,
        key: Option<&str>,
        value: Option<&Value>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // 如果未提供键或值，返回当前实例的克隆
        if key.is_none() || value.is_none() {
            return Ok(self.clone());
        }
        let key = key.unwrap();
        let value = value.unwrap();

        // 确保文档是数组类型
        let array = self.document.as_array().ok_or("Document is not an array")?;

        // 筛选符合条件的元素
        let filtered: Vec<Value> = array
            .iter()
            .filter(|item| item.as_object().and_then(|obj| obj.get(key)) == Some(value))
            .cloned()
            .collect();

        Ok(Self::new(Value::Array(filtered)))
    }

    /// 根据名称映射重命名字段
    pub fn rename(
        &mut self,
        name_map: &std::collections::HashMap<String, String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 确保文档是数组类型
        let array = self
            .document
            .as_array_mut()
            .ok_or("Document is not an array")?;

        // 遍历每个对象并重命名字段
        for item in array {
            let obj = item
                .as_object_mut()
                .ok_or("Array element is not an object")?;
            for (old_name, new_name) in name_map {
                if let Some(value) = obj.remove(old_name) {
                    obj.insert(new_name.clone(), value);
                }
            }
        }

        Ok(())
    }
    /// 根据指定字段顺序重排文档中的对象字段
    pub fn orderby(&mut self, levels: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
        let array = self
            .document
            .as_array_mut()
            .ok_or("Document is not an array")?;
        let mut new_doc = Vec::with_capacity(array.len());

        for item in array.drain(..) {
            let mut new_item = serde_json::Map::new();
            if let Some(obj) = item.as_object() {
                for &level in levels {
                    if let Some(value) = obj.get(level) {
                        new_item.insert(level.to_string(), value.clone());
                    }
                }
            }
            new_doc.push(Value::Object(new_item));
        }

        self.document = Value::Array(new_doc);
        Ok(())
    }

    /// 根据指定键对文档进行排序
    pub fn sort(&mut self, key: &str, reverse: bool) -> Result<(), Box<dyn std::error::Error>> {
        let array = self
            .document
            .as_array_mut()
            .ok_or("Document is not an array")?;

        array.sort_by(|a, b| {
            let a_val = a.as_object().and_then(|obj| obj.get(key));
            let b_val = b.as_object().and_then(|obj| obj.get(key));
            // TODO 处理不同类型的字段
            match (a_val, b_val) {
                (Some(a), Some(b)) => a.as_str().cmp(&b.as_str()),
                (Some(_), None) => std::cmp::Ordering::Greater,
                (None, Some(_)) => std::cmp::Ordering::Less,
                (None, None) => std::cmp::Ordering::Equal,
            }
        });

        if reverse {
            array.reverse();
        }

        Ok(())
    }

    /// 对文档中的每个元素应用转换函数
    pub fn map<F>(&mut self, f: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Fn(Value) -> Value,
    {
        let array = self
            .document
            .as_array_mut()
            .ok_or("Document is not an array")?;

        for item in array {
            *item = f(item.take());
        }

        Ok(())
    }

    /// 对文档中指定键的值应用转换函数
    pub fn apply<F>(&mut self, key: &str, f: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Fn(Value) -> Value,
    {
        let array = self
            .document
            .as_array_mut()
            .ok_or("Document is not an array")?;

        for item in array {
            if let Some(obj) = item.as_object_mut() {
                if let Some(value) = obj.remove(key) {
                    obj.insert(key.to_string(), f(value));
                }
            }
        }

        Ok(())
    }

    /// 根据条件筛选文档元素，返回新的处理器实例
    pub fn filter<F>(&self, condition: F) -> Result<Self, Box<dyn std::error::Error>>
    where
        F: Fn(&Value) -> bool,
    {
        let array = self.document.as_array().ok_or("Document is not an array")?;
        let filtered: Vec<Value> = array
            .iter()
            .filter(|&item| condition(item))
            .cloned()
            .collect();

        Ok(Self::new(Value::Array(filtered)))
    }

    /// 获取文档中指定字段的信息，返回新的处理器实例
    pub fn get_specific_fields(&self, fields: &[&str]) -> Result<Self, Box<dyn std::error::Error>> {
        let array = self.document.as_array().ok_or("Document is not an array")?;
        let mut new_doc = Vec::with_capacity(array.len());

        for item in array {
            let obj = item.as_object().ok_or("Array element is not an object")?;
            let mut new_item = serde_json::Map::new();

            for &field in fields {
                if let Some(value) = obj.get(field) {
                    new_item.insert(field.to_string(), value.clone());
                }
            }

            new_doc.push(Value::Object(new_item));
        }

        Ok(Self::new(Value::Array(new_doc)))
    }

    /// 按指定键分组文档元素，支持聚合函数处理
    pub fn group_by<F>(
        &self,
        key: &str,
        agg_map: Option<F>,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        F: Fn(&str, Vec<Value>) -> Value,
    {
        let array = self.document.as_array().ok_or("Document is not an array")?;
        let mut grouped_data = std::collections::HashMap::new();

        // 分组数据
        for item in array {
            let obj = item.as_object().ok_or("Array element is not an object")?;
            let key_value = obj
                .get(key)
                .cloned()
                .ok_or(format!("Key '{}' not found in item", key))?;

            let entry = grouped_data.entry(key_value.clone()).or_insert_with(|| {
                let mut group = serde_json::Map::new();
                group.insert(key.to_string(), key_value);
                group
            });

            // 收集其他字段
            for (k, v) in obj {
                if k != key {
                    let values = entry
                        .entry(k.clone())
                        .or_insert_with(|| Value::Array(Vec::new()));
                    if let Value::Array(arr) = values {
                        arr.push(v.clone());
                    }
                }
            }
        }

        // 应用聚合函数
        if let Some(agg_func) = agg_map {
            for group in grouped_data.values_mut() {
                let keys: Vec<String> = group.keys().cloned().collect();
                for k in keys {
                    if k != key {
                        if let Value::Array(arr) = group.remove(&k).unwrap() {
                            let aggregated = agg_func(&k, arr);
                            group.insert(k, aggregated);
                        }
                    }
                }
            }
        }

        // 转换为结果数组
        let result: Vec<Value> = grouped_data
            .into_iter()
            .map(|(_, group)| Value::Object(group))
            .collect();

        Ok(Self::new(Value::Array(result)))
    }
}
