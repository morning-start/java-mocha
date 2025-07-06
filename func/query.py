from pathlib import Path
from typing import Callable

from core import JSONDataHandler
from core.handler import refine_versions
from core.type import DataFile


def query_publisher_major(data_dir: Path, publisher: str):
    handler = JSONDataHandler.load_data(data_dir / DataFile.PUBLISHERS.value)
    fields = [
        "api_parameter",
        "versions",
    ]
    data = handler.get_specific_fields(fields).filter(
        lambda x: publisher == x["api_parameter"]
    )
    data.apply("versions", refine_versions)
    data.rename({"api_parameter": "name", "versions": "major_version"})
    return data.document


def query_publisher_versions(data_dir: Path, publisher: str, version: int):
    handler = JSONDataHandler.load_data(data_dir / DataFile.PUBLISHERS.value)
    fields = [
        "api_parameter",
        "versions",
    ]
    data = handler.get_specific_fields(fields).filter(
        lambda x: publisher == x["api_parameter"]
    )
    version_filter: Callable[[str], bool] = lambda v: v.startswith(str(version))
    data.apply("versions", lambda x: list(filter(version_filter, x)))
    return data.document
