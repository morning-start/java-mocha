use crate::core::datatype::{Architecture, Distribution, OperatingSystem, PkgType, SupportTerm};
use crate::core::state::{JdkRequest, VersionSpec};
use crate::core::utils::UrlParams;
use std::str::FromStr;

pub trait InputConverter<T> {
    fn convert(input: T) -> Result<JdkRequest, Box<dyn std::error::Error + Send + Sync>>;
}

pub struct CliInputConverter;

impl InputConverter<String> for CliInputConverter {
    fn convert(input: String) -> Result<JdkRequest, Box<dyn std::error::Error + Send + Sync>> {
        let parts: Vec<&str> = input.split('@').collect();
        if parts.len() != 2 {
            return Err("Invalid jdk format. Expected: distribution@version".into());
        }

        let distribution = Distribution::from_str(parts[0])?;
        let version_spec = parse_version_spec(parts[1])?;

        Ok(JdkRequest {
            distribution,
            version_spec,
            target_os: OperatingSystem::get_local_os(),
            target_arch: Architecture::get_local_arch(),
            pkg_type: PkgType::Jdk,
        })
    }
}

pub struct CustomInputConverter {
    pub target_os: OperatingSystem,
    pub target_arch: Architecture,
    pub pkg_type: PkgType,
}

impl CustomInputConverter {
    pub fn convert(self, input: String) -> Result<JdkRequest, Box<dyn std::error::Error + Send + Sync>> {
        let parts: Vec<&str> = input.split('@').collect();
        if parts.len() != 2 {
            return Err("Invalid jdk format. Expected: distribution@version".into());
        }

        let distribution = Distribution::from_str(parts[0])?;
        let version_spec = parse_version_spec(parts[1])?;

        Ok(JdkRequest {
            distribution,
            version_spec,
            target_os: self.target_os,
            target_arch: self.target_arch,
            pkg_type: self.pkg_type,
        })
    }
}

fn parse_version_spec(version: &str) -> Result<VersionSpec, Box<dyn std::error::Error + Send + Sync>> {
    match version.to_lowercase().as_str() {
        "latest" => Ok(VersionSpec::Latest),
        "lts" => Ok(VersionSpec::LTS),
        "sts" => Ok(VersionSpec::STS),
        "mts" => Ok(VersionSpec::MTS),
        "ea" => Ok(VersionSpec::EA),
        v if v.chars().all(char::is_numeric) => Ok(VersionSpec::Major(v.parse()?)),
        v => Ok(VersionSpec::Exact(v.to_string())),
    }
}

pub struct VersionSpecConverter;

impl VersionSpecConverter {
    pub fn to_query_params(spec: &VersionSpec) -> UrlParams {
        let mut params = UrlParams::new();

        match spec {
            VersionSpec::Latest => params.add("version_by_definition", &"latest_ga"),
            VersionSpec::LTS => params.add("version_by_definition", &"latest_lts"),
            VersionSpec::STS => params.add("version_by_definition", &"latest_sts"),
            VersionSpec::MTS => params.add("version_by_definition", &"latest_mts"),
            VersionSpec::EA => params.add("version_by_definition", &"latest_ea"),
            VersionSpec::Major(major) => params.add("jdk_version", major),
            VersionSpec::Exact(version) => params.add("version", version),
        }

        params
    }

    pub fn to_support_term(spec: &VersionSpec) -> Option<SupportTerm> {
        match spec {
            VersionSpec::LTS => Some(SupportTerm::LTS),
            VersionSpec::STS => Some(SupportTerm::STS),
            VersionSpec::MTS => Some(SupportTerm::MTS),
            _ => None,
        }
    }
}

pub struct JdkRequestConverter;

impl JdkRequestConverter {
    pub fn to_search_params(request: &JdkRequest) -> UrlParams {
        let mut params = VersionSpecConverter::to_query_params(&request.version_spec);

        params.add("distribution", &request.distribution.as_ref());
        params.add_iterable(request.target_os.aliases(), "operating_system");
        params.add_iterable(request.target_arch.aliases(), "architecture");
        params.add("package_type", &request.pkg_type.as_ref());

        if let Some(term) = VersionSpecConverter::to_support_term(&request.version_spec) {
            params.add("term_of_support", &term.as_ref());
        }

        params
    }
}