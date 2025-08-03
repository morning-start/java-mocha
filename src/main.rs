use jvm::core::datatype::Architecture;
use jvm::core::datatype::DataFile;
use jvm::core::datatype::OperatingSystem;
use jvm::func::config;
use jvm::func::list;
use jvm::func::sync;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = config::Config::load()?;
    println!("{}", cfg.data_dir.display());
    sync::sync_data(&cfg).await;
    // println!("数据已同步");
    // let java_list = list::list_publish_version(&cfg.data_dir);
    // println!("{:?}", java_list);
    Ok(())
}
