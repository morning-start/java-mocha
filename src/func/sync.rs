use super::config::Config;

use crate::core::datatype::{Architecture, ArchiveType, DataFile, Distribution, OperatingSystem, PkgType};
use crate::core::foojay::FooJay;
use crate::core::utils::save_json;
use indicatif::{ProgressBar, ProgressStyle};


pub const SUPPORTED_PUBLISHER: [Distribution; 2] = [Distribution::ORACLE, Distribution::OracleOpenJdk];
pub const SUPPORTED_ARCHIVE_TYPE: [ArchiveType; 2] = [ArchiveType::TarGz, ArchiveType::Zip];

pub async fn sync_data(cfg: &Config) {
    let foojay = FooJay::new(None, Some(false));
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner());
    pb.enable_steady_tick(std::time::Duration::from_millis(80));
    pb.set_message("Searching distributions...");
    let distributions = foojay
        .search_distributions(None, None, None, None)
        .await
        .unwrap();
    
    pb.set_message("Searching versions...");
    let versions = foojay.search_versions(None, None, None).await.unwrap();
    
    pb.set_message("Searching packages...");
    let packages = foojay
        .search_packages(
            None,
            None,
            None,
            Some(SUPPORTED_PUBLISHER.to_vec()),
            Some(Architecture::get_local_arch()),
            Some(OperatingSystem::get_local_os()),
            Some(SUPPORTED_ARCHIVE_TYPE.to_vec()),
            Some(PkgType::Jdk),
            None,
            None,
            Some(false),
            None,
        )
        .await
        .unwrap();
    save_json(&packages, &cfg.data_dir.join(DataFile::Packages.as_ref())).unwrap();
    save_json(&distributions, &cfg.data_dir.join(DataFile::Distributions.as_ref())).unwrap();
    save_json(&versions, &cfg.data_dir.join(DataFile::Versions.as_ref())).unwrap();
    pb.finish_with_message("Finished");
}
