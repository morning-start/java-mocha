use jvm::func::config;
use jvm::func::sync;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 发送 GET 请求
    let cfg = config::Config::load()?;
    println!("{}", cfg.data_dir.display());
    sync::sync_data(&cfg).await;
    println!("数据已同步");

    Ok(())
}
