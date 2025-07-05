from enum import Enum
from typing import Literal

from .utils import get_sys_arch, get_sys_os


def enum2val(enum_list: list[Enum] | Enum) -> list[str] | str:
    if isinstance(enum_list, list):
        return [enum.value for enum in enum_list]
    if isinstance(enum_list, Enum):
        return enum_list.value

    return None


VersionType = Literal[
    "latest_ea", "latest_ga", "latest_sts", "latest_mts", "latest_lts", "useful"
]

PackVersion = Literal["latest", "latest_sts", "latest_mts", "latest_lts"]
PkgType = Literal["jdk", "jre"]


class SupportTerm(Enum):
    STS = "sts"  # Short Term Stable
    MTS = "mts"  # Mid Term Stable
    LTS = "lts"  # Long Term Stable


class OperatingSystem(Enum):
    AIX = ["aix"]
    LINUX = ["linux", "alpine_linux", "linux_musl"]
    MACOS = ["macos"]
    QNX = ["qnx"]
    SOLARIS = ["solaris"]
    WINDOWS = ["windows"]

    @classmethod
    def get_local_os(cls) -> "OperatingSystem":
        sys_os = get_sys_os()
        # 建立别名到枚举的映射
        alias_map = {}
        for arch_enum in cls:
            for alias in arch_enum.value:
                alias_map[alias] = arch_enum
        canonical = alias_map.get(sys_os)
        if canonical is None:
            raise ValueError(f"Unsupported System: {sys_os}")
        return canonical


class Architecture(Enum):
    ARM32 = ["aarch32", "arm32", "arm"]
    ARM64 = ["aarch64", "arm64"]
    AMD64 = ["amd64", "x64", "x86-64"]
    I386 = ["i386", "x86", "x86-32", "i486", "i586", "i686"]
    MIPS = ["mips"]
    PPC = ["ppc"]
    PPC64 = ["ppc64", "ppc64le", "ppc64el"]
    RISCV64 = ["riscv64"]
    S390 = ["s390"]
    S390X = ["s390x"]
    SPARC = ["sparc"]
    SPARCV9 = ["sparcv9"]

    @classmethod
    def get_local_architecture(cls) -> "Architecture":
        """根据当前系统架构返回对应的 Architecture 枚举值"""
        sys_arch = get_sys_arch()

        # 建立别名到枚举的映射
        alias_map = {}
        for arch_enum in cls:
            for alias in arch_enum.value:
                alias_map[alias] = arch_enum

        canonical = alias_map.get(sys_arch)
        if canonical is None:
            raise ValueError(f"Unsupported architecture: {sys_arch}")
        return canonical


class ArchiveType(Enum):
    APK = "apk"
    CAB = "cab"
    DEB = "deb"
    DMG = "dmg"
    EXE = "exe"
    MSI = "msi"
    PKG = "pkg"
    RPM = "rpm"
    TAR = "tar"
    TAR_GZ = "tar.gz"
    TGZ = "tgz"
    ZIP = "zip"


class Distribution(Enum):
    AOJ = "aoj"
    AOJ_OPENJ9 = "aoj_openj9"
    BISHENG = "bisheng"
    CORRETTO = "corretto"
    DRAGONWELL = "dragonwell"
    GRAALVM_CE8 = "graalvm_ce8"
    GRAALVM_CE11 = "graalvm_ce11"
    GRAALVM_CE16 = "graalvm_ce16"
    GRAALVM_CE17 = "graalvm_ce17"
    GRAALVM_CE19 = "graalvm_ce19"
    GRAALVM_CE20 = "graalvm_ce20"
    GRAALVM_COMMUNITY = "graalvm_community"
    GRAALVM = "graalvm"
    JETBRAINS = "jetbrains"
    KONA = "kona"
    LIBERICA = "liberica"
    LIBERICA_NATIVE = "liberica_native"
    MANDREL = "mandrel"
    MICROSOFT = "microsoft"
    OJDK_BUILD = "ojdk_build"
    OPENLOGIC = "openlogic"
    ORACLE_OPEN_JDK = "oracle_open_jdk"
    ORACLE = "oracle"
    REDHAT = "redhat"
    SAP_MACHINE = "sap_machine"
    SEMERU = "semeru"
    SEMERU_CERTIFIED = "semeru_certified"
    TEMURIN = "temurin"
    TRAVA = "trava"
    ZULU_PRIME = "zulu_prime"
    ZULU = "zulu"
