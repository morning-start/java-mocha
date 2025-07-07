from pathlib import Path

from core.handler import DocumentHandler, ItemType, JSONType
from core.type import DataFile


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
    handler = DocumentHandler.load_data(data_dir / DataFile.PUBLISHERS.value)
    fields = [
        "api_parameter",
        "build_of_openjdk",
        "build_of_graalvm",
        "official_uri",
    ]
    publisher = handler.get_specific_fields(fields)

    def _assemble_build(item: ItemType):
        open_flag = item.pop("build_of_openjdk")
        lvm_flag = item.pop("build_of_graalvm")
        build_type = ""
        if open_flag and lvm_flag:
            build_type = "openjdk+graalvm"
        elif open_flag:
            build_type = "openjdk"
        elif lvm_flag:
            build_type = "graalvm"
        else:
            build_type = "unknown"
        item["build"] = build_type
        return item

    publisher.map(_assemble_build)
    publisher.rename({"api_parameter": "name"})
    publisher.orderby(["name", "build", "official_uri"])
    return publisher.document


def list_version(data_dir: Path) -> JSONType:
    """
    获取版本的 name、version、release_date、support_term、download_uri、download_size、download_hash 信息
    """
    handler = DocumentHandler.load_data(data_dir / DataFile.VERSIONS.value)
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


def list_publish_version(data_dir: Path):
    handler = DocumentHandler.load_data(data_dir / DataFile.PACKAGES.value)
    fields = [
        "distribution",
        "major_version",
    ]
    data = handler.get_specific_fields(fields)
    data.rename({"distribution": "publisher"})
    data = data.group_by(
        "publisher",
        lambda arr: list(set[int](arr)),
    )
    data.apply("major_version", lambda x: sorted(x, reverse=True))
    return data.document
