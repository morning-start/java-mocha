use jvm::core::foojay::FooJay;
use jvm::core::utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 发送 GET 请求
    let foojay = FooJay::new(None, None);

    // 2. 解析响应为 JSON
    let json = foojay
        .search_distributions(None, None, None, None)
        .await?;
    println!("响应 JSON 内容：\n{}", serde_json::to_string_pretty(&json)?);

    // 3. 保存到本地文件
    utils::save_json(&json, "response.json")?;

    println!("响应已保存到 response.json");

    Ok(())
}
