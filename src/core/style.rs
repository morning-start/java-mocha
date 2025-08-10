use serde_json::Value;
use tabled::{
    builder::Builder,
    settings::{Alignment, Format, object, style::Style},
};

pub fn show_table(data: &Vec<Value>, title: &str) -> String {
    // 确保 data 是一个数组 
    let array =data;

    if array.is_empty() {
        return "Empty data".to_string();
    }

    // 获取所有可能的列名（合并所有对象的键）
    let mut columns = std::collections::BTreeSet::new();
    for item in array {
        if let Value::Object(map) = item {
            for key in map.keys() {
                columns.insert(key.clone());
            }
        }
    }

    let mut builder = Builder::new();

    // 添加表头
    builder.push_record(columns.iter().map(|s| s.as_str()));

    // 添加每一行数据
    for item in array {
        let mut row = Vec::new();
        for col in &columns {
            let cell = match item.get(col) {
                Some(val) => val.to_string(),
                None => "".to_string(),
            };
            row.push(cell);
        }
        builder.push_record(row);
    }

    let mut table = builder.build();

    // 设置样式：带边框线
    // ✅ 正确方式：在第 1 行（表头后）添加 === 分隔线
    // ✅ 使用现代风格（自动在表头后加 === 分隔线）
    table.with(Style::modern());

    table.to_string()
}
pub fn show_tree(versions: &[String], current: &str, title: &str) -> String {
    // let mut tree = Tree::new();
    println!("{:?}", versions);
    "".to_string()
}
