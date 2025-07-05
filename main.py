import platform
from pathlib import Path
from typing import Optional

import typer
from typing_extensions import Annotated

from api import Distribution, Foojay, OperatingSystem
from api.type import Architecture, SupportTerm, enum2val
from func.config import set_config
from func.sync import sync_data
from utils.file_utils import load_json, mk_sure, save_json
from utils.json_handler import JSONDataHandler

app = typer.Typer(
    no_args_is_help=True,
    add_completion=False,
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
            envvar="JVM_ROOT",
            help="JVM root directory, default is the `.java-mocha` directory under the user's home directory.",
        ),
    ] = Path.home()
    / ".java-mocha",
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
):
    set_config(jvm_root, jdk_home, cache_home)


@app.command(
    help="Sync the distribution, package, and version data from Foojay to local JSON files."
)
def sync():
    data = mk_sure("data")
    sync_data(data)


@app.command(help="list infos for jdk, publisher, version")
def list(publisher: str = None, version: int = None):
    pass


@app.command(help="Query available JDKs")
def query(publisher: Annotated[str, typer.Option(help="JDK 发布者")] = None):

    pass


if __name__ == "__main__":
    # sync_data()
    app()
