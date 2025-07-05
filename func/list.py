import json
import platform
from pathlib import Path
from typing import Any, Dict, List, Optional

import typer
from typing_extensions import Annotated

from core import Foojay, JSONDataHandler
from core.handler import JSONType
from core.type import Architecture, Distribution, OperatingSystem, SupportTerm, enum2val
from core.utils import load_json, mk_sure, save_json


def list_local_jdk(jdk_home: Path):
    # 展示本地jdk
    if not jdk_home.exists():
        return []
    local_jdks = []
    for item in jdk_home.iterdir():
        if item.is_dir():
            local_jdks.append(item.name)
    return local_jdks


def list_publisher(jvm: Path):
    """
    获取发行版的 name、build_of_openjdk、build_of_graalvm、official_uri 和 versions 信息，versions 只显示 major 级别
    """
    data_dir = jvm / "data"
    handler = JSONDataHandler.load_data(data_dir / "distributions.json")
    fields = [
        "name",
        "build_of_openjdk",
        "build_of_graalvm",
        "official_uri",
    ]
    publisher = handler.get_specific_fields(fields).document
    # 将  "build_of_openjdk",build_of_graalvm", 合并为  key为build，值为 openjdk或graalvm
    new_publisher: JSONType = []
    for item in publisher:
        build_type = "openjdk" if item["build_of_openjdk"] else "graalvm"
        new_publisher.append(
            {
                "name": item["name"],
                "build": build_type,
                "official_uri": item["official_uri"],
            }
        )
    return new_publisher
