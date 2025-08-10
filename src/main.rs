use std::path::Path;

use jvm::core;
use jvm::func;
use jvm::func::install;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = func::config::Config::load()?;
    // println!("{}", cfg.data_dir.display());
    // sync::sync_data(&cfg).await;
    // println!("数据已同步");
    // let java_list = func::switch::switch_jdk("oracle@21.0.7", &cfg);
    // println!("{:?}", java_list);
    let (checksum_type, checksum) = install::query_package_url("oracle@21.0.7", &cfg.data_dir)?
        .ok_or("Expected Some value, but got None")?;
    println!("{:?}", checksum_type);
    println!("{:?}", checksum);
    // jdk-21.0.7_windows-x64_bin.zip
    // let filename = "jdk-21.0.7_windows-x64_bin.zip";
    // let file_path = cfg.cache_home.join(filename);
    // let checksum = core::utils::sha256sum(file_path)?;
    // println!("{:?}", checksum);
    Ok(())
}
