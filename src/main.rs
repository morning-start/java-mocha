use jvm::func::config;
use jvm::func::sync;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = config::Config::load()?;
    println!("{}", cfg.data_dir.display());
    sync::sync_data(&cfg).await;
    println!("数据已同步");

    Ok(())
}
