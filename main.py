import platform
from pathlib import Path
from typing import Optional

import typer
from typing_extensions import Annotated

from api import Distribution, Foojay, OperatingSystem
from api.type import Architecture, SupportTerm, enum2val
from utils.file_utils import load_json, mk_sure, save_json
from utils.json_handler import JSONDataHandler

app = typer.Typer(
    no_args_is_help=True,
    short_help="Java Mocha 是一个 Java 版本管理工具，基于 Foojay 提供的 API 进行开发。",
    help="Java Mocha 是一个 Java 版本管理工具，基于 Foojay 提供的 API 进行开发。可以通过命令行工具进行版本管理，也可以通过 API 进行集成。",
)


@app.command(help="配置 Java Mocha 的 JVM 根目录、JDK 目录和缓存目录。")
def config(
    jvm_root: Annotated[
        Path,
        typer.Argument(help="JVM 根目录，默认为用户主目录下的 `.java-mocha` 目录。"),
    ] = Path.home()
    / ".java-mocha",
    jdk_home: Annotated[
        Optional[Path],
        typer.Option(help="JDK 目录，默认为 JVM 根目录下的 `jdk` 目录。"),
    ] = None,
    cache_home: Annotated[
        Optional[Path],
        typer.Option(help="缓存目录，默认为 JVM 根目录下的 `cache` 目录。"),
    ] = None,
):
    """
    配置 Java Mocha 的 JVM 根目录、JDK 目录和缓存目录。

    Parameters:
    ----------
    jvm_root: Path
        JVM 根目录，默认为用户主目录下的 `.java-mocha` 目录。
    jdk_home: Path
        JDK 目录，默认为 JVM 根目录下的 `jdk` 目录。
    cache_home: Path
        缓存目录，默认为 JVM 根目录下的 `cache` 目录。

    """
    # 初始化配置文件
    config_file = Path.home() / ".java-mocha" / "config.json"
    if config_file.exists():
        cfg = load_json(config_file)
        jvm_root = Path(cfg["jvm_root"])
        jdk_home = Path(cfg["jdk_home"])
        cache_home = Path(cfg["cache_home"])
    else:
        jdk_home = jvm_root / "jdk" if jdk_home is None else jdk_home
        cache_home = jvm_root / "cache" if cache_home is None else cache_home
        cfg = {
            "jvm_root": str(jvm_root),
            "jdk_home": str(jdk_home),
            "cache_home": str(cache_home),
        }
    jvm_root.mkdir(parents=True, exist_ok=True)
    jdk_home.mkdir(parents=True, exist_ok=True)
    cache_home.mkdir(parents=True, exist_ok=True)

    save_json(config_file, cfg)


@app.command(help="同步 Foojay 的发行版、软件包和版本数据到本地 JSON 文件。")
def sync():
    data = mk_sure("data")
    foojay = Foojay()
    distributions = foojay.search_distributions()
    save_json(data / "distributions.json", distributions)

    #  javafx_bundled=True,
    packages = foojay.search_packages(
        operating_system=OperatingSystem.get_local_os(),
        architecture=Architecture.get_local_architecture(),
        javafx_bundled=False,
        package_type="jdk",
    )
    save_json(data / "packages.json", packages)

    versions = foojay.search_versions()
    save_json(data / "versions.json", versions)


if __name__ == "__main__":
    # sync_data()
    app()
