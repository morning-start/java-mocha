use crate::core::datatype::{
    Architecture, ArchiveType, Distribution, OperatingSystem, PackVersion, PkgType, SupportTerm,
    VersionType,
};
use crate::core::utils::UrlParams;
use reqwest::Client;
use reqwest::header::HeaderMap;
use serde_json::Value;
use std::time::Duration;

pub struct FooJay {
    pub distributions: String,
    pub versions: String,
    pub ids: String,
    pub packages: String,
    pub client: Client,
}
impl FooJay {
    pub fn new(timeout: Option<Duration>, verify: Option<bool>) -> Self {
        let timeout = timeout.unwrap_or(Duration::from_secs(10));
        let verify = verify.unwrap_or(true);

        let base_url = "https://api.foojay.io/disco/v3.0".to_string();
        let mut headers: HeaderMap = HeaderMap::new();
        headers.insert("Accept", "application/json".parse().unwrap());

        let client = Client::builder()
            // 设置超时时间（连接超时和请求超时）
            .timeout(timeout)
            // 禁用 SSL 验证（仅在测试环境使用，生产环境不推荐）
            .danger_accept_invalid_certs(!verify)
            // 设置默认请求头
            .default_headers(headers)
            .build()
            .expect("Failed to build client");

        Self {
            distributions: format!("{}/distributions", base_url),
            versions: format!("{}/major_versions", base_url),
            ids: format!("{}/ids", base_url),
            packages: format!("{}/packages", base_url),
            client,
        }
    }
}

impl FooJay {
    /// 发送 GET 请求并返回 JSON 响应中的 "result" 字段
    async fn get(&self, url: &str, params: Option<&UrlParams>) -> Result<Value, reqwest::Error>
    {
        let client = &self.client;
        let mut request = client.get(url);

        // 添加查询参数
        if let Some(params) = params {
            request = request.query(&params.params);
        }

        // 发送 HTTP 请求并等待响应
        // `await` 表示异步等待请求完成
        // `?` 操作符用于错误传播（若请求失败则返回错误）
        let response = request.send().await?;

        // 将响应体解析为 JSON 格式
        // `serde_json::Value` 是一个可以表示任何 JSON 数据的通用类型
        // 再次使用 `await?` 处理异步解析和错误传播
        let json: Value = response.json().await?;

        // 从 JSON 中提取 "result" 字段
        let result = json["result"].clone();
        Ok(result)
    }
    pub async fn search_distributions(
        &self,
        version: Option<String>,
        distro_name: Option<Distribution>,
        include_versions: Option<bool>,
        include_synonyms: Option<bool>,
    ) -> Result<Value, reqwest::Error> {
        // 默认值
        let mut params = UrlParams::new();
        let include_versions = include_versions.unwrap_or(false);
        let include_synonyms = include_synonyms.unwrap_or(false);
        params.add("include_versions", &include_versions);
        params.add("include_synonyms", &include_synonyms);

        let mut url = self.distributions.clone();

        if version.is_some() {
            url = format!(
                "{}/{}/{}",
                self.distributions,
                "versions",
                version.as_ref().unwrap()
            );
        } else if distro_name.is_some() {
            url = format!(
                "{}/{}",
                self.distributions,
                distro_name.as_ref().unwrap().as_ref()
            );
        }

        // 返回值
        self.get(&url, Some(&params)).await
    }
    pub async fn search_versions(
        &self,
        version: Option<i8>,
        version_definition: Option<VersionType>,
        include_versions: Option<bool>,
    ) -> Result<Value, reqwest::Error> {
        let include_versions = include_versions.unwrap_or(false);
        let mut url = self.versions.clone();
        if version.is_some() {
            url = format!(
                "{}/{}/{}/{}",
                &self.versions,
                "versions",
                version.as_ref().unwrap(),
                "ga"
            );
        }
        else if version_definition.is_some() {
            url = format!(
                "{}/{}",
                &self.versions,
                version_definition.as_ref().unwrap().as_ref()
            );
        }
        let mut params = UrlParams::new();
        params.add("include_versions", &include_versions);
        self.get(&url, Some(&params)).await
    }
    pub async fn search_packages(
        &self,
        version: Option<String>,
        version_by_definition: Option<PackVersion>,
        jdk_version: Option<i8>,
        distribution: Option<Vec<Distribution>>,
        architecture: Option<Architecture>,
        operating_system: Option<OperatingSystem>,
        archive_type: Option<Vec<ArchiveType>>,
        package_type: Option<PkgType>,
        term_of_support: Option<SupportTerm>,
        include_versions: Option<bool>,
        javafx_bundled: Option<bool>,
        free_to_use_in_production: Option<bool>,
    ) -> Result<Value, reqwest::Error> {
        let mut params = UrlParams::new();
        // single value
        let include_versions = include_versions.unwrap_or(true);
        let free_to_use_in_production = free_to_use_in_production.unwrap_or(true);
        let javafx_bundled = javafx_bundled.unwrap_or(false);
        params.add("include_versions", &include_versions);
        params.add("free_to_use_in_production", &free_to_use_in_production);
        params.add("javafx_bundled", &javafx_bundled);

        // iterable
        let distribution = distribution.unwrap_or(vec![]);
        let architecture = architecture.unwrap_or(Architecture::get_local_arch());
        let archive_type = archive_type.unwrap_or(vec![ArchiveType::Zip]);
        let operating_system = operating_system.unwrap_or(OperatingSystem::get_local_os());
        params.add_iterable(distribution, "distribution");
        params.add_iterable(architecture.aliases(), "architecture");
        params.add_iterable(archive_type, "archive_type");
        params.add_iterable(operating_system.aliases(), "operating_system");

        // not sure
        params.add_optional("package_type", package_type);
        params.add_optional("term_of_support", term_of_support);
        params.add_optional("version", version);
        params.add_optional("version_by_definition", version_by_definition);
        params.add_optional("jdk_version", jdk_version);

        self.get(&self.packages, Some(&params)).await
    }
}
