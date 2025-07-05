import platform
from pathlib import Path
from typing import Any, Dict, List, Optional

import typer
from typing_extensions import Annotated

from core import Foojay, JSONDataHandler
from core.type import Architecture, Distribution, OperatingSystem, SupportTerm, enum2val
from core.utils import load_json, mk_sure, save_json


def list_publisher(dir_path: Path) -> List[Dict[str, Any]]:
    """
    获取发行版的 name、build_of_openjdk、build_of_graalvm、official_uri 和 versions 信息，versions 只显示 major 级别
    """
    handler = JSONDataHandler.load_data(dir_path / "distributions.json")
    fields = [
        "name",
        "build_of_openjdk",
        "build_of_graalvm",
        "official_uri",
    ]
    publisher = handler.get_specific_fields(fields)

    return publisher
