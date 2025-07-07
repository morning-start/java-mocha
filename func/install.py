from pathlib import Path

import requests
from rich.progress import Progress, SpinnerColumn, TextColumn

from core.handler import DocumentHandler
from core.style import download_package
from core.type import DataFile
from core.utils import extract_tar_gz, extract_zip, move_and_clean_subfolder, sha256sum
from func.config import Config


def query_package_url(jdk: str, data_dir: Path):
    """
    查询 package url
    Args:
        jdk: 发行版名称@版本号
    Returns:
        package url，如果未找到则返回 None
    """
    p, v = jdk.split("@")
    handler = DocumentHandler.load_data(data_dir / DataFile.PACKAGES.value)
    fields = [
        "id",
        "distribution",
        "distribution_version",
        "major_version",
        "latest_build_available",
        "links",
    ]
    data = handler.get_specific_fields(fields)
    data = handler.query("distribution", value=p)
    if v == "latest":
        data = data.query("latest_build_available", True)
    elif v.isdigit():
        data = data.query("major_version", int(v))
        data = data.query("latest_build_available", True)
    else:
        data = data.query("distribution_version", v)
    ele = data.document[0]
    link: str = ele.get("links").get("pkg_info_uri")
    publisher: str = ele.get("distribution")
    version: str = ele.get("distribution_version")
    jdk_version = f"{publisher}@{version}"
    return link, jdk_version


def get_package_info(url: str, proxy: str = None) -> dict:
    """
    发送请求查找安装包 url 和校验和
    Args:
        uri: 包 uri
    Returns:
        包含安装包 url 和校验和的字典，如果请求失败则返回 None
    """
    # 此处需要替换为实际的 API 地址
    if proxy:
        proxies = {"https": proxy}
    else:
        proxies = None

    response = requests.get(url, proxies=proxies)
    response.raise_for_status()
    return response.json().get("result")[0]


def get_checksum(info: dict, proxy: str = None):
    """
    获取校验和信息
    Args:
        info: 包含校验和信息的字典
    Returns:
        包含校验和类型和校验和的字典
    """
    if proxy:
        proxies = {"https": proxy}
    else:
        proxies = None
    checksum_type: str = info.get("checksum_type")
    checksum: str = info.get("checksum")
    checksum_uri: str = info.get("checksum_uri")
    if checksum_uri:
        response = requests.get(checksum_uri, proxies=proxies)
        response.raise_for_status()
        checksum = response.text.strip()
    return checksum_type, checksum


def download_cache(
    info: dict, cache_home: Path, proxy: str = None, force: bool = False
):
    """
    下载安装包到 cache_dir 并校验和
    Args:
        url: 安装包下载地址
        cache_dir: 缓存目录路径
        expected_checksum: 预期的校验和
    Returns:
        下载后的文件路径，如果下载或校验失败则返回 None
    """
    if proxy:
        proxies = {"https": proxy}
    else:
        proxies = None
    uri: str = info.get("direct_download_uri")
    file_name: str = info.get("filename")
    file_path = cache_home / file_name
    if not file_path.exists() or force:
        download_package(uri, file_path, proxies)
    return file_path


def check_pack(file_path: Path, checksum: str, checksum_type: str):
    if checksum_type == "sha256":
        checksum_ = sha256sum(file_path)
    else:
        raise ValueError(f"Unsupported checksum type: {checksum_type}")

    return checksum_ == checksum


def install_jdk(file_path: Path, jdk_dir: Path):
    jdk_dir.mkdir(parents=True, exist_ok=True)
    if file_path.suffix == ".zip":
        extract_zip(file_path, jdk_dir)
    elif file_path.suffix == ".tar.gz":
        extract_tar_gz(file_path, jdk_dir)
    else:
        raise ValueError(f"Unsupported file type: {file_path.suffix}")


def full_install_process(jdk: str, cfg: Config, force: bool = False):
    info_url, jdk = query_package_url(jdk, cfg.data_dir)
    with Progress(
        SpinnerColumn(),
        TextColumn("[progress.description]{task.description}"),
        transient=True,
    ) as progress:
        progress.add_task(description="Query package info...")
        info = get_package_info(info_url, cfg.proxy)
        progress.add_task(description="Get checksum...")
        checksum_type, checksum = get_checksum(info, cfg.proxy)
    package_path = download_cache(info, cfg.cache_home, cfg.proxy, force)
    flag = check_pack(package_path, checksum, checksum_type)
    if not flag:
        raise ValueError("Checksum verification failed")
    install_jdk(package_path, cfg.jdk_home / jdk)
    move_and_clean_subfolder(cfg.jdk_home / jdk)
