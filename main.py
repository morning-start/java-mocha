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
    short_help="Java Mocha is a Java version management tool developed based on the Foojay API.",
    help="Java Mocha is a Java version management tool developed based on the Foojay API. It can be used for version management via the command-line interface or integrated through the API.",
)


@app.command(
    help="Configure the JVM root directory, JDK directory, and cache directory for Java Mocha."
)
def config(
    jvm_root: Annotated[
        Path,
        typer.Argument(
            help="JVM root directory, default is the `.java-mocha` directory under the user's home directory."
        ),
    ] = Path.home()
    / ".java-mocha",
    jdk_home: Annotated[
        Optional[Path],
        typer.Option(
            help="JDK directory, default is the `jdk` directory under the JVM root directory."
        ),
    ] = None,
    cache_home: Annotated[
        Optional[Path],
        typer.Option(
            help="Cache directory, default is the `cache` directory under the JVM root directory."
        ),
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
    # Initialize the configuration file
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


@app.command(
    help="Sync the distribution, package, and version data from Foojay to local JSON files."
)
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


@app.command(help="List all available versions")
def list_remote():
    pass


if __name__ == "__main__":
    # sync_data()
    app()
