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
    local_jdks: list[str] = []
    for item in jdk_home.iterdir():
        if item.is_dir():
            local_jdks.append(item.name)
    return local_jdks


def list_publisher(data_dir: Path):
    """
    获取发行版的 name、build_of_openjdk、build_of_graalvm、official_uri 和 versions 信息，versions 只显示 major 级别
    """
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


def list_version(data_dir: Path) -> JSONType:
    """
    获取版本的 name、version、release_date、support_term、download_uri、download_size、download_hash 信息
    """
    handler = JSONDataHandler.load_data(data_dir / "versions.json")
    fields = [
        "major_version",
        "term_of_support",
        "maintained",
    ]
    version = handler.get_specific_fields(fields).document
    # # 对每个进行改名
    # for item in version:
    #     item["version"] = item.pop("major_version")
    #     item["support_term"] = item.pop("term_of_support")
    return version
