use jvm::core;
use  jvm::func;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = func::config::Config::load()?;
    // println!("{}", cfg.data_dir.display());
    // sync::sync_data(&cfg).await;
    // println!("数据已同步");
    let java_list = func::switch::switch_jdk("oracle@24.0.1", &cfg);
    println!("{:?}", java_list);
    Ok(())
}
