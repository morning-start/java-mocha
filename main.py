from pathlib import Path
from typing import Optional

import typer
from typing_extensions import Annotated

from core import log
from core.style import show_table, show_tree
from core.type import SupportTerm
from func.config import Config, check_java_home, init_config, load_jvm
from func.install import full_install_process
from func.list import list_local_jdk, list_publish_version, list_publisher, list_version
from func.query import query_info, query_info_term, query_info_version
from func.sync import sync_data
from func.uninstall import uninstall_jdk
from func.use import switch_jdk

app = typer.Typer(
    no_args_is_help=True,
    add_completion=False,
    short_help="Java Mocha is a Java version management tool developed based on the Foojay API.",
    help="""Java Mocha is a Java version management tool developed based on the Foojay API.\n
    It can be used for version management via the command-line interface or integrated through the API.""",
    epilog="""Before using, \n
    1. please first initialize the configuration with `jvm config`,\n
    2. then sync the data with `jvm sync`. \n
    3. Use `--help` to view specific command usage.""",
)


@app.command(
    help="Configure the JDK directory, and cache directory for Java Mocha.",
)
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
    java_home = init_config(jvm_root, jdk_home, cache_home, proxy)
    log.info("Config saved successfully.")
    if not check_java_home(jvm_root):
        log.warning(f"Please set JAVA_HOME to {java_home} manually.")


@app.command(help="Sync the Foojay JDK data to local JSON files.")
def sync():
    jvm_root = load_jvm()
    cfg = Config.load(jvm_root)
    proxies = None
    if cfg.proxy:
        log.info(f"Syncing data from Foojay with proxy {cfg.proxy}")
        proxies = {"https": cfg.proxy, "http": cfg.proxy}
    sync_data(cfg.data_dir, proxies)


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
        if not jdks:
            log.warning("No JDK found, please install first.")
        else:
            txt = "Installed jdk (* marks in use)"
            tree = show_tree(jdks, cfg.jdk_version, txt)
            log.info(tree)
    # version 为 True 时，列出所有major发行版
    elif publisher and version:
        data = list_publish_version(cfg.data_dir)
        title = "Each publisher available major versions"
        table = show_table(data, title)
        log.info(table)

    elif version:
        version_data = list_version(cfg.data_dir)
        title = "major versions infos"
        table = show_table(version_data, title)
        log.info(table)
    # publisher 为 True 时，列出所有发行商
    elif publisher:
        publisher_data = list_publisher(cfg.data_dir)
        title = "publishers infos"
        table = show_table(publisher_data, title)
        log.info(table)


@app.command(help="Query available JDKs")
def query(
    publisher: Annotated[
        str, typer.Option(..., "--publisher", "-p", help="The publisher name")
    ],
    major_version: Annotated[
        int,
        typer.Option(
            ..., "--major_version", "-v", help="Detailed major version information"
        ),
    ] = None,
    term_of_support: Annotated[
        SupportTerm,
        typer.Option(..., "--term_of_support", "-t", help="Term of support"),
    ] = None,
):
    jvm_root = load_jvm()
    cfg = Config.load(jvm_root)
    if term_of_support and major_version:
        log.error("term_of_support and major_version can not be used at the same time")
    elif major_version:
        data = query_info_version(cfg.data_dir, publisher, major_version)
        txt = f"JDKs for {publisher} version {major_version}"
        table = show_table(data, txt)
        log.info(table)
    elif term_of_support:
        data = query_info_term(cfg.data_dir, publisher, term_of_support)
        txt = f"Latest JDKs for {publisher} on {term_of_support}"
        table = show_table(data, txt)
        log.info(table)
    else:
        data = query_info(cfg.data_dir, publisher)
        txt = f"Latest JDKs for publisher {publisher}"
        table = show_table(data, txt)
        log.info(table)


@app.command(help="Install JDKs")
def install(
    jdk: Annotated[
        str,
        typer.Option(
            ...,
            "--publisher@version",
            "-jdk",
            help="""The JDK version format as publisher@version\n
            e.g. oracle@23, oracle@23.0.2, oracle@latest""",
        ),
    ],
    force: Annotated[
        bool,
        typer.Option(..., "--force", "-f", help="Force install"),
    ] = False,
):
    jvm_root = load_jvm()
    cfg = Config.load(jvm_root)
    try:
        full_install_process(jdk, cfg, force)
    except Exception as e:
        log.error(f"Install failed: {e}")


@app.command(
    help="Switch JAVA_HOME environment variable.",
    epilog="""Before using, \n
    1. please view local jdk version via `jvm list` command.\n
    2. set JAVA_HOME = JVM_ROOT/current """,
)
def use(
    jdk: Annotated[
        str,
        typer.Option(
            ...,
            "--publisher@version",
            "-jdk",
            help="The JDK version format as publisher@version e.g. oracle@11",
        ),
    ],
):
    jvm_root = load_jvm()
    cfg = Config.load(jvm_root)
    res = switch_jdk(jdk, cfg)
    if res:
        log.info(f"JDK switched to {jdk}.")
    else:
        log.error(f"JDK {jdk} not found.")


@app.command(
    help="Uninstall JDKs",
    epilog="Before using, please view local jdk version via `jvm list` command. ",
)
def uninstall(
    jdk: Annotated[
        str,
        typer.Option(
            ...,
            "--publisher@version",
            "-jdk",
            help="The JDK version format as publisher@version e.g. oracle@11",
        ),
    ],
):
    jvm_root = load_jvm()
    cfg = Config.load(jvm_root)
    res = uninstall_jdk(jdk, cfg)
    if res:
        log.info(f"JDK {jdk} uninstalled successfully.")
    else:
        log.error(f"JDK {jdk} not found.")


if __name__ == "__main__":
    pass
    app()
