use crate::core::datatype::{Architecture, Distribution, OperatingSystem, PackageInfo, PkgType};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JdkRequest {
    pub distribution: Distribution,
    pub version_spec: VersionSpec,
    pub target_os: OperatingSystem,
    pub target_arch: Architecture,
    pub pkg_type: PkgType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionSpec {
    Latest,
    LTS,
    STS,
    MTS,
    Major(i8),
    Exact(String),
    EA,
}

impl Default for VersionSpec {
    fn default() -> Self {
        VersionSpec::Latest
    }
}

#[derive(Debug)]
pub enum OperationState {
    Initial,
    Validated(JdkRequest),
    Resolved(PackageInfo),
    Downloaded(PathBuf),
    Verified(PathBuf),
    Installed(String),
    Failed(String),
}

#[derive(Debug)]
pub enum OperationEvent {
    Validate(JdkRequest),
    Resolve,
    Download,
    Verify,
    Install,
    Fail(String),
}

pub type OperationResult<T> = Result<T, String>;