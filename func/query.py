from pathlib import Path
from typing import Callable

from core import JSONDataHandler
from core.type import DataFile, SupportTerm


def query_info(data_dir: Path, publisher: str):
    handler = JSONDataHandler.load_data(data_dir / DataFile.PACKAGES.value)
    fields = [
        "distribution",
        "major_version",
        "term_of_support",
        "latest_build_available",
        "distribution_version",
    ]
    data = handler.get_specific_fields(fields)
    data = data.filter(
        lambda x: x["distribution"] == publisher and x["latest_build_available"] == True
    )
    data.rename(
        {
            "distribution": "publisher",
            "major_version": "major",
            "term_of_support": "term",
            "latest_build_available": "latest",
            "distribution_version": "version",
        }
    )
    return data.document


def query_info_version(data_dir: Path, publisher: str, major_version: int):
    handler = JSONDataHandler.load_data(data_dir / DataFile.PACKAGES.value)
    fields = [
        "distribution",
        "major_version",
        "term_of_support",
        "latest_build_available",
        "distribution_version",
    ]
    data = handler.get_specific_fields(fields)
    data = data.filter(
        lambda x: x["distribution"] == publisher and x["major_version"] == major_version
    )
    data.rename(
        {
            "distribution": "publisher",
            "major_version": "major",
            "term_of_support": "term",
            "latest_build_available": "latest",
            "distribution_version": "Version",
        }
    )
    return data.document


def query_info_term(data_dir: Path, publisher: str, term_of_support: SupportTerm):
    handler = JSONDataHandler.load_data(data_dir / DataFile.PACKAGES.value)
    fields = [
        "distribution",
        "major_version",
        "term_of_support",
        "latest_build_available",
        "distribution_version",
    ]
    data = handler.get_specific_fields(fields)
    data = data.filter(
        lambda x: x["distribution"] == publisher
        and x["term_of_support"] == term_of_support.value
        and x["latest_build_available"] == True
    )
    data.rename(
        {
            "distribution": "publisher",
            "major_version": "major",
            "term_of_support": "term",
            "latest_build_available": "latest",
            "distribution_version": "Version",
        }
    )
    return data.document
