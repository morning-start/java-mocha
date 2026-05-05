use crate::core::installer::JdkInstaller;
use crate::core::state::OperationResult;
use crate::func::config::Config;

pub async fn full_install_process(
    jdk: &str,
    cfg: &Config,
    force: bool,
    skip_check: bool,
) -> OperationResult<String> {
    let mut installer = JdkInstaller::new(cfg.clone(), force, skip_check);
    installer.run_from_input(jdk).await
}