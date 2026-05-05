use crate::core::converter::{CliInputConverter, InputConverter};
use crate::core::datatype::PackageInfo;
use crate::core::foojay::FooJay;
use crate::core::state::{OperationEvent, OperationResult, OperationState};
use crate::core::utils::{
    download_package, extract_tar_gz, extract_zip, move_and_clean_subfolder,
    sha256sum,
};
use crate::func::config::Config;
use serde_json::Value;
use std::path::PathBuf;

pub struct JdkInstaller {
    state: OperationState,
    config: Config,
    force: bool,
    skip_check: bool,
}

impl JdkInstaller {
    pub fn new(config: Config, force: bool, skip_check: bool) -> Self {
        Self {
            state: OperationState::Initial,
            config,
            force,
            skip_check,
        }
    }

    pub async fn process(&mut self, event: OperationEvent) -> OperationResult<()> {
        self.state = match (&self.state, event) {
            (OperationState::Initial, OperationEvent::Validate(request)) => {
                OperationState::Validated(request)
            }

            (OperationState::Validated(request), OperationEvent::Resolve) => {
                let package_info = self.resolve_package(request).await?;
                OperationState::Resolved(package_info)
            }

            (OperationState::Resolved(info), OperationEvent::Download) => {
                let path = self.download_package(info).await?;
                OperationState::Downloaded(path)
            }

            (OperationState::Downloaded(path), OperationEvent::Verify) => {
                self.verify_package(path.clone()).await?;
                OperationState::Verified(path.clone())
            }

            (OperationState::Verified(path), OperationEvent::Install) => {
                let version = self.install_package(path.clone()).await?;
                OperationState::Installed(version)
            }

            (_, OperationEvent::Fail(err)) => OperationState::Failed(err),

            _ => return Err("Invalid state transition".to_string()),
        };

        Ok(())
    }

    pub fn get_state(&self) -> &OperationState {
        &self.state
    }

    pub async fn run_from_input(&mut self, input: &str) -> OperationResult<String> {
        let request = CliInputConverter::convert(input.to_string())?;
        
        self.process(OperationEvent::Validate(request)).await?;
        self.process(OperationEvent::Resolve).await?;
        self.process(OperationEvent::Download).await?;
        self.process(OperationEvent::Verify).await?;
        self.process(OperationEvent::Install).await?;

        if let OperationState::Installed(version) = &self.state {
            Ok(version.clone())
        } else {
            Err("Installation failed".to_string())
        }
    }

    async fn resolve_package(&self, request: &crate::core::state::JdkRequest) -> OperationResult<PackageInfo> {
        let foojay = FooJay::new(None, None);
        let result = foojay.search_packages(
            None, None, None,
            Some(vec![request.distribution]),
            Some(request.target_arch),
            Some(request.target_os),
            None, Some(request.pkg_type),
            None, None, None, None
        ).await.map_err(|e| format!("Failed to search packages: {}", e))?;

        parse_package_info(&result, Some(&self.config.proxy)).await
    }

    async fn download_package(&self, info: &PackageInfo) -> OperationResult<PathBuf> {
        let file_path = self.config.cache_home.join(&info.filename);

        if !self.force && file_path.exists() {
            return Ok(file_path);
        }

        download_package(&info.direct_download_uri, &file_path, Some(&self.config.proxy))
            .await
            .map_err(|e| format!("Failed to download package: {}", e))?;

        Ok(file_path)
    }

    async fn verify_package(&self, path: PathBuf) -> OperationResult<()> {
        if self.skip_check {
            return Ok(());
        }

        if let OperationState::Resolved(info) = &self.state {
            if !info.checksum.is_empty() && !info.checksum_type.is_empty() {
                if info.checksum_type == "sha256" {
                    let calculated = sha256sum(&path)
                        .map_err(|e| format!("Failed to calculate checksum: {}", e))?;
                    if calculated != info.checksum {
                        let _ = std::fs::remove_file(&path);
                        return Err("Checksum verification failed".to_string());
                    }
                } else {
                    return Err(format!("Unsupported checksum type: {}", info.checksum_type));
                }
            }
        }

        Ok(())
    }

    async fn install_package(&self, path: PathBuf) -> OperationResult<String> {
        if let OperationState::Resolved(info) = &self.state {
            let jdk_version = format!("{}@{}", info.filename.split('-').next().unwrap_or("unknown"), 
                                     info.filename.split('-').nth(1).unwrap_or("unknown").split('.').next().unwrap_or("unknown"));
            
            let jdk_target_path = self.config.jdk_home.join(&jdk_version);
            std::fs::create_dir_all(&jdk_target_path)
                .map_err(|e| format!("Failed to create directory: {}", e))?;

            if path.extension().and_then(|ext| ext.to_str()) == Some("zip") {
                extract_zip(&path, &jdk_target_path)
                    .map_err(|e| format!("Failed to extract zip: {}", e))?;
            } else {
                extract_tar_gz(&path, &jdk_target_path)
                    .map_err(|e| format!("Failed to extract tar.gz: {}", e))?;
            }

            move_and_clean_subfolder(&jdk_target_path)
                .map_err(|e| format!("Failed to clean up directory: {}", e))?;
            Ok(jdk_version)
        } else {
            Err("Cannot install without resolved package info".to_string())
        }
    }
}

async fn parse_package_info(result: &Value, proxy: Option<&str>) -> OperationResult<PackageInfo> {
    if let Some(array) = result.as_array() {
        if let Some(first) = array.first() {
            let obj = first.as_object().ok_or("Result is not an object".to_string())?;
            
            let links = obj.get("links").and_then(|l| l.as_object())
                .ok_or("Missing links field".to_string())?;
            let pkg_info_uri = links.get("pkg_info_uri").and_then(|u| u.as_str())
                .ok_or("Missing pkg_info_uri".to_string())?;
            
            let client = crate::core::utils::build_client(proxy)
                .map_err(|e| format!("Failed to build client: {}", e))?;
            let response = client.get(pkg_info_uri)
                .send()
                .await
                .map_err(|e| format!("Failed to send request: {}", e))?
                .json::<Value>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;

            if let Some(result) = response.get("result").and_then(|r| r.as_array()) {
                if let Some(info) = result.first() {
                    return Ok(PackageInfo {
                        filename: val2str(info, "filename"),
                        direct_download_uri: val2str(info, "direct_download_uri"),
                        download_site_uri: val2str(info, "download_site_uri"),
                        signature_uri: val2str(info, "signature_uri"),
                        checksum_uri: val2str(info, "checksum_uri"),
                        checksum: val2str(info, "checksum"),
                        checksum_type: val2str(info, "checksum_type"),
                    });
                }
            }
        }
    }
    
    Err("Failed to parse package info".to_string())
}

fn val2str(val: &Value, key: &str) -> String {
    val.get(key).and_then(|v| v.as_str()).unwrap_or("").to_string()
}