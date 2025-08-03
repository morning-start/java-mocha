use std::str::FromStr;

use strum_macros::{AsRefStr, Display};
// cSpell: disable

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, AsRefStr)]
pub enum DataFile {
    #[strum(serialize = "distributions.json")]
    Distributions,
    #[strum(serialize = "versions.json")]
    Versions,
    #[strum(serialize = "packages.json")]
    Packages,
}

// ANCHOR Distribution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, AsRefStr)]
pub enum Distribution {
    #[strum(serialize = "aoj")]
    AOJ,

    #[strum(serialize = "aoj_openj9")]
    AojOpenj9,

    #[strum(serialize = "bisheng")]
    BISHENG,

    #[strum(serialize = "corretto")]
    CORRETTO,

    #[strum(serialize = "dragonwell")]
    DRAGONWELL,

    #[strum(serialize = "graalvm_ce8")]
    GraalvmCe8,

    #[strum(serialize = "graalvm_ce11")]
    GraalvmCe11,

    #[strum(serialize = "graalvm_ce16")]
    GraalvmCe16,

    #[strum(serialize = "graalvm_ce17")]
    GraalvmCe17,

    #[strum(serialize = "graalvm_ce19")]
    GraalvmCe19,

    #[strum(serialize = "graalvm_ce20")]
    GraalvmCe20,

    #[strum(serialize = "graalvm_community")]
    GraalvmCommunity,

    #[strum(serialize = "graalvm")]
    GRAALVM,

    #[strum(serialize = "jetbrains")]
    JETBRAINS,

    #[strum(serialize = "kona")]
    KONA,

    #[strum(serialize = "liberica")]
    LIBERICA,

    #[strum(serialize = "liberica_native")]
    LibericaNative,

    #[strum(serialize = "mandrel")]
    MANDREL,

    #[strum(serialize = "microsoft")]
    MICROSOFT,

    #[strum(serialize = "ojdk_build")]
    OjdkBuild,

    #[strum(serialize = "openlogic")]
    OPENLOGIC,

    #[strum(serialize = "oracle_open_jdk")]
    OracleOpenJdk,

    #[strum(serialize = "oracle")]
    ORACLE,

    #[strum(serialize = "redhat")]
    REDHAT,

    #[strum(serialize = "sap_machine")]
    SapMachine,

    #[strum(serialize = "semeru")]
    SEMERU,

    #[strum(serialize = "semeru_certified")]
    SemeruCertified,

    #[strum(serialize = "temurin")]
    TEMURIN,

    #[strum(serialize = "trava")]
    TRAVA,

    #[strum(serialize = "zulu_prime")]
    ZuluPrime,

    #[strum(serialize = "zulu")]
    ZULU,
}

// ANCHOR VersionType
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, AsRefStr)]
pub enum VersionType {
    #[strum(serialize = "latest_ea")]
    LatestEA,
    #[strum(serialize = "latest_ga")]
    LatestGA,
    #[strum(serialize = "latest_sts")]
    LatestSTS,
    #[strum(serialize = "latest_mts")]
    LatestMTS,
    #[strum(serialize = "latest_lts")]
    LatestLTS,
    #[strum(serialize = "useful")]
    Useful,
}

// ANCHOR PackVersion
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, AsRefStr)]
pub enum PackVersion {
    #[strum(serialize = "latest")]
    Latest,
    #[strum(serialize = "latest_sts")]
    LatestSTS,
    #[strum(serialize = "latest_mts")]
    LatestMTS,
    #[strum(serialize = "latest_lts")]
    LatestLTS,
}

// ANCHOR SupportTerm
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, AsRefStr)]
pub enum SupportTerm {
    #[strum(serialize = "sts")]
    STS,
    #[strum(serialize = "mts")]
    MTS,
    #[strum(serialize = "lts")]
    LTS,
}

// ANCHOR Architecture
#[derive(Debug, Clone, Copy, PartialEq, Eq, AsRefStr)]
pub enum Architecture {
    Arm32,
    Arm64,
    Amd64,
    I386,
    Mips,
    Ppc,
    Ppc64,
    Riscv64,
    S390,
    S390x,
    Sparc,
    Sparcv9,
}

impl Architecture {
    /// 获取当前架构的所有别名
    pub fn aliases(&self) -> &'static [&'static str] {
        match self {
            Self::Arm32 => &["aarch32", "arm32", "arm"],
            Self::Arm64 => &["aarch64", "arm64"],
            Self::Amd64 => &["amd64", "x64", "x86-64"],
            Self::I386 => &["i386", "x86", "x86-32", "i486", "i586", "i686"],
            Self::Mips => &["mips"],
            Self::Ppc => &["ppc"],
            Self::Ppc64 => &["ppc64", "ppc64le", "ppc64el"],
            Self::Riscv64 => &["riscv64"],
            Self::S390 => &["s390"],
            Self::S390x => &["s390x"],
            Self::Sparc => &["sparc"],
            Self::Sparcv9 => &["sparcv9"],
        }
    }

    /// 获取所有架构枚举值
    pub fn all() -> &'static [Self] {
        &[
            Self::Arm32,
            Self::Arm64,
            Self::Amd64,
            Self::I386,
            Self::Mips,
            Self::Ppc,
            Self::Ppc64,
            Self::Riscv64,
            Self::S390,
            Self::S390x,
            Self::Sparc,
            Self::Sparcv9,
        ]
    }

    // 获取本机架构
    pub fn get_local_arch() -> Self {
        let arch = std::env::consts::ARCH;
        let arch = arch.to_lowercase().replace('_', "-");
        let arch = Self::from_str(&arch).unwrap();
        arch
    }
}

impl FromStr for Architecture {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        // 遍历所有架构及其别名进行匹配
        for arch in Self::all() {
            if arch.aliases().contains(&s.as_str()) {
                return Ok(*arch);
            }
        }
        Err(format!("Unknown architecture: {}", s))
    }
}

// ANCHOR OperatingSystem
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, AsRefStr)]
pub enum OperatingSystem {
    AIX,
    LINUX,
    MACOS,
    QNX,
    SOLARIS,
    WINDOWS,
}

impl OperatingSystem {
    pub fn aliases(&self) -> &'static [&'static str] {
        match self {
            Self::AIX => &["aix"],
            Self::LINUX => &["linux", "alpine_linux", "linux_musl"],
            Self::MACOS => &["macos"],
            Self::QNX => &["qnx"],
            Self::SOLARIS => &["solaris"],
            Self::WINDOWS => &["windows"],
        }
    }

    pub fn all() -> &'static [Self] {
        &[
            Self::AIX,
            Self::LINUX,
            Self::MACOS,
            Self::QNX,
            Self::SOLARIS,
            Self::WINDOWS,
        ]
    }
    pub fn get_local_os() -> Self {
        let sys_os = std::env::consts::OS;
        let sys_os = sys_os.to_lowercase();
        let os = Self::from_str(&sys_os).unwrap();
        os
    }
}
impl FromStr for OperatingSystem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        // 遍历所有操作系统及其别名进行匹配
        for os in Self::all() {
            if os.aliases().contains(&s.as_str()) {
                return Ok(*os);
            }
        }
        Err(format!("Unknown operating system: {}", s))
    }
}

// ANCHOR ArchiveType
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, AsRefStr)]
pub enum ArchiveType {
    #[strum(serialize = "apk")]
    Apk,
    #[strum(serialize = "cab")]
    Cab,
    #[strum(serialize = "deb")]
    Deb,
    #[strum(serialize = "dmg")]
    Dmg,
    #[strum(serialize = "exe")]
    Exe,
    #[strum(serialize = "msi")]
    Msi,
    #[strum(serialize = "pkg")]
    Pkg,
    #[strum(serialize = "rpm")]
    Rpm,
    #[strum(serialize = "tar")]
    Tar,
    #[strum(serialize = "tar.gz")]
    TarGz,
    #[strum(serialize = "tgz")]
    Tgz,
    #[strum(serialize = "zip")]
    Zip,
}

// ANCHOR PkgType
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, AsRefStr)]
pub enum PkgType {
    #[strum(serialize = "jdk")]
    Jdk,
    #[strum(serialize = "jre")]
    Jre,
}
