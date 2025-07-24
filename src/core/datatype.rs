use strum_macros::{AsRefStr, Display};
// cSpell: disable
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
