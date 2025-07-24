use reqwest::Client;
use reqwest::header::HeaderMap;
use std::collections::HashMap;
use std::time::Duration;

use crate::core::datatype::Distribution;

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
            versions: format!("{}/versions", base_url),
            ids: format!("{}/ids", base_url),
            packages: format!("{}/packages", base_url),
            client,
        }
    }
}

impl FooJay {
    /// 发送 GET 请求并返回 JSON 响应中的 "result" 字段
    async fn get(
        &self,
        url: &str,
        params: Option<&HashMap<String, String>>,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let client = &self.client;
        let mut request = client.get(url);

        // 添加查询参数
        if let Some(params) = params {
            request = request.query(params);
        }

        // 发送 HTTP 请求并等待响应
        // `await` 表示异步等待请求完成
        // `?` 操作符用于错误传播（若请求失败则返回错误）
        let response = request.send().await?;

        // 将响应体解析为 JSON 格式
        // `serde_json::Value` 是一个可以表示任何 JSON 数据的通用类型
        // 再次使用 `await?` 处理异步解析和错误传播
        let json: serde_json::Value = response.json().await?;

        Ok(json)
    }
    pub async fn search_distributions(
        &self,
        version: Option<String>,
        distro_name: Option<Distribution>,
        include_versions: Option<bool>,
        include_synonyms: Option<bool>,
    ) -> Result<serde_json::Value, reqwest::Error> {
        // 默认值
        let include_versions = include_versions.unwrap_or(false);
        let include_synonyms = include_synonyms.unwrap_or(false);

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
        let mut params = HashMap::new();
        params.insert("include_versions".to_string(), include_versions.to_string());
        params.insert("include_synonyms".to_string(), include_synonyms.to_string());

        // 返回值
        self.get(&url, Some(&params)).await
    }
}
