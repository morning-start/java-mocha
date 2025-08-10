use super::sync::SUPPORTED_PUBLISHER;
use crate::core::{datatype::DataFile, handler::DocumentHandler};
use serde_json::{Number, Value};
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

/// 列出本地 JDK 安装路径
///
/// 该函数扫描指定目录下的所有子目录，
/// 将每个子目录的路径作为字符串添加到返回的向量中。
///
/// # 参数
///
/// * `jdk_home` - 需要扫描的 JDK 主目录路径
///
/// # 返回值
///
/// * 包含所有本地 JDK 安装路径的字符串向量
pub fn list_local_jdk(jdk_home: &Path) -> Vec<String> {
    let mut jdk_list = Vec::new();
    if jdk_home.exists() {
        for entry in jdk_home.read_dir().unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_dir() {
                jdk_list.push(
                    entry
                        .path()
                        .as_path()
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .to_string(),
                );
            }
        }
    }

    jdk_list
}

/// 获取发行版的 name、build_of_openjdk、build_of_graalvm、official_uri 和 versions 信息，versions 只显示 major 级别
/// 列出受支持的发行商信息
///
/// 该函数从数据目录加载发行商数据，并提取特定字段，
/// 然后进行一系列处理，包括构建类型组装、字段重命名、排序和筛选，
/// 最终返回处理后的 JSON 数据。
pub fn list_publisher(data_dir: &Path) -> Vec<Value> {
    // 构建发行商数据文件的路径
    let file_path = data_dir.join(DataFile::Distributions.as_ref());
    // 加载发行商数据文件到 DocumentHandler
    let handler = DocumentHandler::load_data(file_path.as_path()).unwrap();
    // 定义需要提取的字段列表
    let fields = [
        "api_parameter",
        "build_of_openjdk",
        "build_of_graalvm",
        "official_uri",
    ];
    // 提取指定字段的数据
    let mut publisher = handler.get_specific_fields(&fields).unwrap();

    // 应用构建类型组装函数，转换 build_of_openjdk 和 build_of_graalvm 字段
    publisher.map(assemble_build).unwrap();

    // 创建字段重命名映射表，将 api_parameter 重命名为 name
    let mut name_map = HashMap::new();
    name_map.insert("api_parameter".to_string(), "name".to_string());
    // 执行字段重命名
    publisher.rename(&name_map).unwrap();

    // 按 name, build, official_uri 字段顺序对数据进行排序
    publisher
        .orderby(&["name", "build", "official_uri"])
        .unwrap();
    // 获取受支持的发行商名称列表
    let supported_publisher: Vec<&str> = SUPPORTED_PUBLISHER.iter().map(|x| x.as_ref()).collect();

    // 筛选出受支持的发行商数据
    let filtered_publisher = publisher
        .filter(|x| {
            x.as_object()
                .and_then(|obj| obj.get("name"))
                .and_then(|name| name.as_str())
                .map(|name| supported_publisher.contains(&name))
                .unwrap_or(false)
        })
        .unwrap();

    // 返回处理后的 JSON 数据
    filtered_publisher.document().as_array().unwrap().to_vec()
}

/// 列出版本信息
///
/// 该函数从数据目录加载发行商数据，并提取特定字段，
/// 最终返回处理后的 JSON 数据。
///
/// # 参数
///
/// * `data_dir` - 数据目录路径
///
/// # 返回值
///
/// * 处理后的版本 JSON 数据
pub fn list_version(data_dir: &Path) -> Vec<Value> {
    // 构建发行商数据文件的路径
    let file_path = data_dir.join(DataFile::Distributions.as_ref());
    // 加载发行商数据文件到 DocumentHandler
    let handler = DocumentHandler::load_data(file_path.as_path()).unwrap();
    // 定义需要提取的字段列表
    let fields = ["name", "build", "official_uri"];
    // 提取指定字段的数据
    let version = handler.get_specific_fields(&fields).unwrap();
    version.document().as_array().unwrap().to_vec()
}

/// 列出发布版本信息
///
/// 该函数从数据目录加载发布版本数据，并提取特定字段，
/// 然后进行字段重命名和分组操作，
/// 最终返回处理后的 JSON 数据。
///
/// # 参数
///
/// * `data_dir` - 数据目录路径
///
/// # 返回值
///
/// * 处理后的发布版本 JSON 数据
pub fn list_publish_version(data_dir: &Path) -> Vec<Value> {
    let file_path = data_dir.join(DataFile::Packages.as_ref());
    // 加载数据文件到 DocumentHandler
    let handler = DocumentHandler::load_data(file_path.as_path()).unwrap();
    // 定义需要提取的字段列表
    let fields = ["distribution", "major_version"];
    // 提取指定字段的数据
    let mut data = handler.get_specific_fields(&fields).unwrap();
    // 重命名字段，将 distribution 重命名为 publisher
    data.rename(&HashMap::from([(
        "distribution".to_string(),
        "publisher".to_string(),
    )]))
    .unwrap();
    // f=  lambda arr: list(set(arr)),
    // 定义一个聚合函数，用于去除重复值
    // 该函数接收一个 JSON 值的数组，将其中的字符串值去重后返回新的字符串数组
    let agg_map = |arr: Vec<Value>| -> Vec<Value> {
        // 使用 HashSet 去重，然后转换为 Vec
        arr.into_iter()
            .map(|item| item.to_string())
            .collect::<HashSet<_>>()
            .into_iter()
            .map(|v| Value::String(v.to_string()))
            .collect()
    };
    // 按 publisher 字段分组数据
    let mut data = data.group_by("publisher", Some(agg_map)).unwrap();
    data.apply("major_version", |arr: Value| -> Value {
        let mut arr = arr.as_array().unwrap().clone();
        arr.sort_by(|a, b| b.as_str().unwrap().cmp(a.as_str().unwrap()));
        Value::Array(arr)
    })
    .unwrap();

    // 按 publisher 字段排序数据
    let new_data = data.orderby(&["publisher", "major_version"]).unwrap();

    // major_version 转为 list[int]
    let res = new_data
        .into_iter()
        .map(|x| {
            let mut obj = x.as_object().unwrap().clone();
            let major_version = obj.remove("major_version").unwrap();
            // major_version 由 Array<String> 转为 Array<int>
            let mut major_version = major_version
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap().parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            // 倒序
            major_version.sort_by(|a, b| b.cmp(a));

            // 转换为 JSON 数组: vec<int> -> Value::Array<Number>
            let major_version = Value::Array(
                major_version
                    .iter()
                    .map(|v| Value::Number(Number::from(*v)))
                    .collect(),
            );
            obj.insert("major_version".to_string(), major_version);

            Value::Object(obj)
        })
        .collect::<Vec<Value>>();
    res
}

/// 组装构建类型信息
///
/// 该函数根据 build_of_openjdk 和 build_of_graalvm 两个布尔字段的值，
/// 确定并设置新的 build 字段，然后移除原有的两个布尔字段。
///
/// # 参数
///
/// * `item` - 包含构建信息的 JSON 值
///
/// # 返回值
///
/// * 处理后的 JSON 值，包含新的 build 字段
fn assemble_build(mut item: Value) -> Value {
    // 获取 build_of_openjdk 和 build_of_graalvm 字段的值
    let build_of_openjdk = item
        .get("build_of_openjdk")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let build_of_graalvm = item
        .get("build_of_graalvm")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    // 根据字段值确定 build 类型
    let build_type = if build_of_openjdk && build_of_graalvm {
        "OpenJDK + GraalVM"
    } else if build_of_openjdk {
        "OpenJDK"
    } else if build_of_graalvm {
        "GraalVM"
    } else {
        "Unknown"
    };

    // 如果 item 是一个对象，则修改其字段
    if let Some(obj) = item.as_object_mut() {
        // 移除原有的布尔字段
        obj.remove("build_of_openjdk");
        obj.remove("build_of_graalvm");
        // 插入新的 build 字段
        obj.insert("build".to_string(), Value::String(build_type.to_string()));
    }

    item
}
