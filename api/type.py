from enum import Enum
from typing import Literal


def enum2val(enum_list: list[Enum] | Enum):
    if isinstance(enum_list, list[Enum]):
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
    AIX = "aix"
    ALPINE_LINUX = "alpine_linux"
    LINUX = "linux"
    LINUX_MUSL = "linux_musl"
    MACOS = "macos"
    QNX = "qnx"
    SOLARIS = "solaris"
    WINDOWS = "windows"


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


class Architecture(Enum):
    AARCH32 = "aarch32"
    AARCH64 = "aarch64"
    AMD64 = "amd64"
    ARM = "arm"
    ARM32 = "arm32"
    ARM64 = "arm64"
    MIPS = "mips"
    PPC = "ppc"
    PPC64EL = "ppc64el"
    PPC64LE = "ppc64le"
    PPC64 = "ppc64"
    RISCV64 = "riscv64"
    S390 = "s390"
    S390X = "s390x"
    SPARC = "sparc"
    SPARCV9 = "sparcv9"
    X64 = "x64"
    X86_64 = "x86-64"
    X86 = "x86"
    I386 = "i386"
    I486 = "i486"
    I586 = "i586"
    I686 = "i686"
    X86_32 = "x86-32"


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
