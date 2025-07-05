import os
from pathlib import Path
from typing import Optional

import typer
from typing_extensions import Annotated

from core import log
from core.handler import show_table
from func.config import Config, init_config
from func.list import list_local_jdk, list_publish_version, list_publisher, list_version
from func.sync import sync_data

app = typer.Typer(
    no_args_is_help=True,
    add_completion=False,
    short_help="Java Mocha is a Java version management tool developed based on the Foojay API.",
    help="""Java Mocha is a Java version management tool developed based on the Foojay API.
It can be used for version management via the command-line interface or integrated through the API.""",
    epilog="""Before using, 1. please first initialize the configuration with `jvm config`,
2. then sync the data with `jvm sync`. 3. Use `--help` to view specific command usage.""",
)


def load_jvm() -> Path:
    jvm_root = Path.home() / ".java-mocha"
    if "JVM_ROOT" in os.environ:
        jvm_root = Path(os.environ["JVM_ROOT"])
    return jvm_root


@app.command(help="Configure the JDK directory, and cache directory for Java Mocha.")
def config(
    jdk_home: Annotated[
        Optional[Path],
        typer.Option(
            envvar="JDK_HOME",
            help="JDK directory, default is the `jdk` directory under the JVM root directory.",
        ),
    ] = None,
    cache_home: Annotated[
        Optional[Path],
        typer.Option(
            help="Cache directory, default is the `cache` directory under the JVM root directory."
        ),
    ] = None,
    #  set proxy https:
    proxy: Annotated[
        Optional[str],
        typer.Option(
            help="Proxy server, default is the `http_proxy` environment variable."
        ),
    ] = None,
):
    jvm_root = load_jvm()
    init_config(jvm_root, jdk_home, cache_home, proxy)
    log.info("Config saved successfully.")


@app.command(help="Sync the Foojay JDK data to local JSON files.")
def sync():
    jvm_root = load_jvm()
    cfg = Config.load(jvm_root)
    proxies = None
    if cfg.proxy:
        log.info(f"Syncing data from Foojay with proxy {cfg.proxy}")
        proxies = {"https": cfg.proxy, "http": cfg.proxy}
    sync_data(jvm_root, proxies)


@app.command(help="list infos for local jdk, all publisher, all version")
def list(
    publisher: Annotated[
        bool, typer.Option(..., "--publisher", "-p", help="Publisher name")
    ] = False,
    version: Annotated[
        bool, typer.Option(..., "--version", "-v", help="Version flag")
    ] = False,
):
    jvm_root = load_jvm()
    cfg = Config.load(jvm_root)
    # 都为 False 时，列出本地jdk信息 publisher@version
    if not publisher and not version:
        jdks = list_local_jdk(cfg.jdk_home)
        jdks = ["a", "b", "c"]
        if not jdks:
            log.warning("No JDK found, please install first.")
        else:
            # TODO  格式化输出，正在使用，排序
            log.info(*jdks, sep="\n")
    # version 为 True 时，列出所有major发行版
    elif publisher and version:
        data = list_publish_version(cfg.data_dir)
        table = show_table(data)
        log.info(table)

    elif version:
        version_data = list_version(cfg.data_dir)
        table = show_table(version_data)
        log.info(table)
    # publisher 为 True 时，列出所有发行商
    elif publisher:
        publisher_data = list_publisher(cfg.data_dir)
        table = show_table(publisher_data)
        log.info(table)


@app.command(help="Query available JDKs")
def query(
    publisher: Annotated[
        str, typer.Option(..., "--publisher", "-p", help="The publisher name")
    ],
    version: Annotated[
        bool,
        typer.Option(..., "--version", "-v", help="Detailed version information"),
    ] = False,
):

    pass


@app.command(help="Install JDKs")
def install(
    jdk: Annotated[
        str,
        typer.Option(
            ...,
            "--jdk",
            "-j",
            help="The JDK version format as publisher@version e.g. oracle@11",
        ),
    ],
    force: Annotated[
        bool,
        typer.Option(..., "--force", "-f", help="Force install"),
    ] = False,
):
    pass


@app.command(
    help="Switch JAVA_HOME environment variable.",
    epilog="Before using, please view local jdk version via `jvm list` command. ",
)
def use(
    jdk: Annotated[
        str,
        typer.Option(
            ...,
            "--jdk",
            "-j",
            help="The JDK version format as publisher@version e.g. oracle@11",
        ),
    ],
):
    pass


if __name__ == "__main__":
    pass
    app()
