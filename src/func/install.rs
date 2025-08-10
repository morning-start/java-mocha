use crate::core::datatype::{DataFile, PackageInfo};
use crate::core::handler::DocumentHandler;
use crate::core::utils::{
    build_client, download_package, extract_tar_gz, extract_zip, move_and_clean_subfolder,
    sha256sum,
};
use crate::func::config::Config;
use serde_json::Value;
use std::path::{Path, PathBuf};

/// 查询包URL
///
/// # Arguments
/// * `jdk` - 发行版名称@版本号
/// * `data_dir` - 数据目录路径
///
/// # Returns
/// * `Option<(String, String)>` - 包URL和JDK版本，如果未找到则返回None
pub fn query_package_url(
    jdk: &str,
    data_dir: &Path,
) -> Result<Option<(String, String)>, Box<dyn std::error::Error>> {
    // 解析jdk参数为发行版名称和版本号
    let parts: Vec<&str> = jdk.split('@').collect();
    if parts.len() != 2 {
        return Err("Invalid jdk format. Expected: distribution@version".into());
    }
    let distribution = parts[0];
    let version = parts[1];

    // 加载包数据
    let file_path = &data_dir.join(DataFile::Packages.as_ref());
    let handler = DocumentHandler::load_data(file_path)?;

    // 定义需要的字段
    let fields = [
        "id",
        "distribution",
        "distribution_version",
        "major_version",
        "latest_build_available",
        "links",
    ];

    // 获取指定字段的数据
    let mut data = handler.get_specific_fields(&fields)?;

    // 根据发行版筛选数据
    data = data.query(
        Some("distribution"),
        Some(&serde_json::Value::String(distribution.to_string())),
    )?;

    // 根据版本号进行进一步筛选
    if version == "latest" {
        data = data.query(
            Some("latest_build_available"),
            Some(&serde_json::Value::Bool(true)),
        )?;
    } else if version == "lts" {
        data = data.query(
            Some("term_of_support"),
            Some(&serde_json::Value::String("lts".to_string())),
        )?;
        data = data.query(
            Some("latest_build_available"),
            Some(&serde_json::Value::Bool(true)),
        )?;
    } else if version.chars().all(char::is_numeric) {
        let major_version: i32 = version.parse()?;
        data = data.query(
            Some("major_version"),
            Some(&serde_json::Value::Number(major_version.into())),
        )?;
        data = data.query(
            Some("latest_build_available"),
            Some(&serde_json::Value::Bool(true)),
        )?;
    } else {
        data = data.query(
            Some("distribution_version"),
            Some(&serde_json::Value::String(version.to_string())),
        )?;
    }

    // 获取第一个匹配的元素
    let document = data.document();
    let array = document.as_array().ok_or("Document is not an array")?;

    if array.is_empty() {
        return Ok(None);
    }

    let ele = &array[0];
    let obj = ele.as_object().ok_or("Element is not an object")?;

    // 提取链接和版本信息
    let links = obj
        .get("links")
        .ok_or("Links field not found")?
        .as_object()
        .ok_or("Links is not an object")?;
    let link = links
        .get("pkg_info_uri")
        .ok_or("pkg_info_uri not found in links")?
        .as_str()
        .ok_or("pkg_info_uri is not a string")?;

    let publisher = obj
        .get("distribution")
        .ok_or("distribution field not found")?
        .as_str()
        .ok_or("distribution is not a string")?;

    let version = obj
        .get("distribution_version")
        .ok_or("distribution_version field not found")?
        .as_str()
        .ok_or("distribution_version is not a string")?;

    let jdk_version = format!("{}@{}", publisher, version);

    Ok(Some((link.to_string(), jdk_version)))
}

/// 发送请求查找安装包 url 和校验和
///
/// # Arguments
/// * `url` - 包信息URL
/// * `proxy` - 可选的代理设置
///
/// # Returns
/// * `Option<Value>` - 包含安装包url和校验和的值，如果请求失败则返回None
pub async fn get_package_info(
    url: &str,
    proxy: Option<&str>,
) -> Result<Value, Box<dyn std::error::Error>> {
    // 创建HTTP客户端
    let client = build_client(proxy)?;
    // 发送GET请求
    let response = client.get(url).send().await?;

    // 检查响应状态
    if !response.status().is_success() {
        return Err(format!("HTTP request failed with status: {}", response.status()).into());
    }

    // 解析JSON响应
    let json: Value = response.json().await?;

    // 返回结果中的第一个元素
    if let Some(result) = json.get("result").and_then(|r| r.as_array()) {
        if !result.is_empty() {
            Ok(result[0].clone())
        } else {
            Err("No result found in response".into())
        }
    } else {
        Err("Invalid response format: missing or invalid 'result' field".into())
    }
}

/// 获取校验和信息
///
/// # Arguments
/// * `info` - 包含校验和信息的serde_json::Value
/// * `proxy` - 可选的代理设置
///
/// # Returns
/// * `Result<(String, String), Box<dyn std::error::Error>>` - 包含校验和类型和校验和的元组
pub async fn get_checksum(
    info: &PackageInfo,
    proxy: Option<&str>,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let checksum_type = info.checksum_type.clone();
    let mut checksum = info.checksum.clone();
    let checksum_uri = info.checksum_uri.clone();

    // 如果 checksum 是空字符串，且 checksum_uri 不是空字符串
    if checksum.is_empty() && !checksum_uri.is_empty() {
        let client = build_client(proxy)?;
        let response = client.get(checksum_uri).send().await?;
        response.error_for_status_ref()?;
        checksum = response.text().await?.trim().to_string();
    }

    Ok((checksum_type, checksum))
}

/// 校验下载的文件是否与校验和匹配
///
/// # Arguments
/// * `download_file_path` - 下载文件的路径
/// * `checksum` - 校验和
/// * `checksum_type` - 校验和类型
///
/// # Returns
/// * `Result<bool, Box<dyn std::error::Error>>` - 如果校验和匹配则返回true，否则返回false
pub async fn check_pack(
    download_file_path: &Path,
    checksum: &str,
    checksum_type: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    if checksum_type == "sha256" {
        let checksum_ = sha256sum(download_file_path)?;
        Ok(checksum == checksum_)
    } else {
        Err("Unsupported checksum type".into())
    }
}

/// 解压缩JDK文件
///
/// # Arguments
/// * `package_path` - 压缩包文件路径
/// * `target_folder` - 目标文件夹路径
///
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - 解压缩结果
pub fn extract_jdk(
    package_path: &Path,
    target_folder: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // 确保目标文件夹存在
    std::fs::create_dir_all(target_folder)?;

    // 根据文件扩展名选择解压缩方法
    if package_path.extension().and_then(|ext| ext.to_str()) == Some("zip") {
        extract_zip(package_path, target_folder)
    } else {
        extract_tar_gz(package_path, target_folder)
    }
}

/// 下载安装包到 cache_dir 并校验和
///
/// # Arguments
/// * `info` - 包含下载 URI、文件名和预期校验和的信息
/// * `cache_home` - 缓存目录路径
/// * `proxy` - 可选的代理地址
/// * `force` - 是否强制重新下载
///
/// # Returns
/// 下载后的文件路径，如果下载或校验失败则返回 None
pub async fn download_cache(
    info: &PackageInfo,
    cache_home: &Path,
    proxy: Option<&str>,
    force: bool,
) -> Option<PathBuf> {
    let file_path = cache_home.join(&info.filename);

    // 如果不是强制下载且文件已存在，则直接返回文件路径
    if !force && file_path.exists() {
        return Some(file_path);
    }

    // 下载文件
    if download_package(&info.direct_download_uri, &file_path, proxy)
        .await
        .is_err()
    {
        return None;
    }

    // 校验和验证
    if !info.checksum.is_empty() && !info.checksum_type.is_empty() {
        match check_pack(&file_path, &info.checksum, &info.checksum_type).await {
            Ok(true) => Some(file_path),
            Ok(false) => {
                // 校验和不匹配，删除文件
                let _ = std::fs::remove_file(&file_path);
                None
            }
            Err(_) => {
                // 校验和验证出错，删除文件
                let _ = std::fs::remove_file(&file_path);
                None
            }
        }
    } else {
        // 没有校验和信息，直接返回文件路径
        Some(file_path)
    }
}

/// 完整的JDK安装流程
///
/// # Arguments
/// * `jdk` - JDK发行版和版本，格式为"发行版@版本"
/// * `cfg` - 配置信息
/// * `force` - 是否强制重新下载
/// * `skip_check` - 是否跳过校验和检查
///
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - 安装结果
pub async fn full_install_process(
    jdk: &str,
    cfg: &Config,
    force: bool,
    skip_check: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    // 查询包URL
    let (info_url, jdk_version) =
        query_package_url(jdk, &cfg.data_dir)?.ok_or("Package not found")?;

    // 获取包信息
    let info_value = get_package_info(&info_url, Some(&cfg.proxy)).await?;

    // 提取包信息到PackageInfo结构体
    let info = PackageInfo {
        filename: val2str(&info_value, "filename"),
        direct_download_uri: val2str(&info_value, "direct_download_uri"),
        download_site_uri: val2str(&info_value, "download_site_uri"),
        signature_uri: val2str(&info_value, "signature_uri"),
        checksum_uri: val2str(&info_value, "checksum_uri"),
        checksum: val2str(&info_value, "checksum"),
        checksum_type: val2str(&info_value, "checksum_type"),
    };

    // 获取校验和信息
    let (checksum_type, checksum) = get_checksum(&info, Some(&cfg.proxy)).await?;

    // 下载文件
    let package_path = download_cache(&info, &cfg.cache_home, Some(&cfg.proxy), force)
        .await
        .ok_or("Failed to download package")?;

    // 校验和检查
    if !skip_check {
        let flag = check_pack(&package_path, &checksum, &checksum_type).await?;
        if !flag {
            return Err("Checksum verification failed".into());
        }
    }

    // 解压缩JDK
    let jdk_target_path = cfg.jdk_home.join(&jdk_version);
    extract_jdk(&package_path, &jdk_target_path)?;

    // 移动并清理子文件夹
    move_and_clean_subfolder(&jdk_target_path)?;

    Ok(jdk_version)
}

fn val2str(val: &Value, str: &str) -> String {
    val.get(str)
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}
