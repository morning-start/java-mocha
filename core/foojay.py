from urllib.parse import urljoin

import requests

from .type import (
    Architecture,
    ArchiveType,
    Distribution,
    OperatingSystem,
    PackVersion,
    PkgType,
    SupportTerm,
    VersionType,
    enum2val,
)


class Foojay:
    """
    https://api.foojay.io/swagger-ui
    """

    base_url = "https://api.foojay.io/disco/v3.0/"
    distributions = urljoin(base_url, "distributions")
    versions = urljoin(base_url, "major_versions")
    ids = urljoin(base_url, "ids")
    packages = urljoin(base_url, "packages")

    def __init__(self, proxies: dict = None, timeout=10, verify=True):
        self.headers = {
            "Accept": "application/json",
        }
        self.timeout = timeout
        self.verify = verify
        self.proxies = proxies

    def get(
        self,
        url: str,
        params: dict = {},
    ) -> list:
        res = requests.get(
            url=url,
            headers=self.headers,
            proxies=self.proxies,
            params=params,
            timeout=self.timeout,
            verify=self.verify,
        )
        return res.json().get("result", [])

    def search_distributions(
        self,
        version: str = None,
        distro_name: Distribution = None,
        include_versions: bool = True,
        include_synonyms: bool = False,
    ):
        if version is not None:
            url = urljoin(self.distributions, f"versions/{version}")
        elif distro_name is not None:
            url = urljoin(self.distributions, enum2val(distro_name))
        else:
            url = self.distributions
        return self.get(
            url,
            params={
                "include_versions": include_versions,
                "include_synonyms": include_synonyms,
            },
        )

    def search_versions(
        self,
        version: int = None,
        version_definition: VersionType = None,
        include_versions: bool = False,
    ):
        if version is not None:
            url = urljoin(self.versions, f"versions/{version}/ga")
        elif version_definition is not None:
            url = urljoin(self.versions, f"{version_definition}")
        else:
            url = self.versions
        return self.get(url, params={"include_versions": include_versions})

    def search_packages(
        self,
        version: str = None,
        version_by_definition: PackVersion = None,
        jdk_version: int = None,
        distribution: list[Distribution] = None,
        architecture: Architecture = None,
        operating_system: OperatingSystem = None,
        archive_type: list[ArchiveType] = None,
        package_type: PkgType = "jdk",
        term_of_support: list[SupportTerm] = None,
        include_versions: bool = True,
        javafx_bundled: bool = None,
        free_to_use_in_production=True,
    ):
        """
        根据指定的条件搜索 Java 包。

        Parameters
        ----------
        version : str, optional
            指定版本号（例如 11.9.0.1, 1.8.0_262, 15, 16-ea）。
            支持版本范围查询（例如 15.0.1..<16）。
        version_by_definition : PackVersion, optional
            根据预定义规则选择版本（latest, latest_sts, latest_mts, latest_lts）。
        jdk_version : int, optional
            JDK 的主要版本号（例如 11），用于筛选基于不同 JDK 构建的 GraalVM。
        distribution : list[str], optional
            分发包名称列表（如 aoj, corretto, temurin 等）。
        architecture : list[str], optional
            架构类型列表（如 aarch64, amd64, x86 等）。
        archive_type : list[str], optional
            归档文件格式列表（如 zip, tar.gz, rpm 等）。
        package_type : str, optional
            包类型（jdk 或 jre），默认为 jdk。
        operating_system : list[str], optional
            支持的操作系统列表（如 linux, windows, macos 等）。
        term_of_support : list[SupportTerm], optional
            支持期限类型列表（sts, mts, lts）。
        include_versions : bool, optional
            是否包含所有可用版本，默认为 False。
        javafx_bundled : bool, optional
            如果为 True，则返回包含 JavaFX 的包，默认为 None
        free_to_use_in_production : bool, optional
            如果为 True，表示该包可免费用于生产环境，默认为 True。

        Returns
        -------
        list
            符合条件的 Java 包列表。
        """

        distribution = enum2val(distribution)
        architecture = enum2val(architecture)
        archive_type = enum2val(archive_type)
        operating_system = enum2val(operating_system)
        term_of_support = enum2val(term_of_support)

        params = {
            "version": version,
            "version_by_definition": version_by_definition,
            "jdk_version": jdk_version,
            "distribution": distribution,
            "architecture": architecture,
            "archive_type": archive_type,
            "package_type": package_type,
            "operating_system": operating_system,
            "release_status": ["ga"],
            "term_of_support": term_of_support,
            "include_versions": include_versions,
            "javafx_bundled": javafx_bundled,
            "free_to_use_in_production": free_to_use_in_production,
        }
        return self.get(self.packages, params)

    def query_ids(self, id: str, pack=False) -> dict:
        if pack:
            url = urljoin(self.packages, id)
        else:
            url = urljoin(self.ids, id)
        return self.get(url)[0]
