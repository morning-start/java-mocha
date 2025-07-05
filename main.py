import os
from pathlib import Path
from typing import Optional

import typer
from rich.console import Console
from typing_extensions import Annotated

from core import Foojay, JSONDataHandler
from core.handler import show_table
from core.type import Architecture, Distribution, OperatingSystem, SupportTerm, enum2val
from core.utils import load_json, mk_sure, save_json
from func.config import init_config, set_config
from func.list import list_publisher
from func.sync import sync_data

console = Console()
app = typer.Typer(
    no_args_is_help=True,
    add_completion=False,
    short_help="Java Mocha is a Java version management tool developed based on the Foojay API.",
    help="Java Mocha is a Java version management tool developed based on the Foojay API. It can be used for version management via the command-line interface or integrated through the API.",
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
    init_config(jvm_root, jdk_home, cache_home)
    set_config(jvm_root, "proxy", proxy)


@app.command(
    help="Sync the distribution, package, and version data from Foojay to local JSON files."
)
def sync():
    jvm_root = load_jvm()
    sync_data(jvm_root)


@app.command(help="list infos for jdk, publisher, version")
def list(
    publisher: Annotated[bool, typer.Option(help="Publisher name")] = False,
    version: Annotated[bool, typer.Option(help="Version flag")] = False,
):
    jvm_root = load_jvm()
    # 都为 False 时，列出本地jdk信息 publisher@version
    if not publisher and not version:
        pass
    # version 为 True 时，列出所有major发行版
    elif version:
        pass
    # publisher 为 True 时，列出所有发行商
    elif publisher:
        publisher_data = list_publisher(jvm_root)
        table = show_table(publisher_data)
        console.print(table)

    else:
        # typer处理错误，输入不符合规范
        typer.secho("input is not supported", err=True)


@app.command(help="Query available JDKs")
def query(publisher: Annotated[str, typer.Option(help="JDK 发布者")] = None):

    pass


if __name__ == "__main__":
    pass
    app()
