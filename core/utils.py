import hashlib
import json
import platform
import tarfile
import zipfile
from pathlib import Path


def extract_zip(zip_file: Path, target_folder: Path):
    """
    解压缩zip文件到指定文件夹

    :param zip_file: zip文件的路径
    :param target_folder: 目标文件夹的路径
    """
    with zipfile.ZipFile(zip_file, "r") as zip_ref:
        zip_ref.extractall(target_folder)


def extract_tar_gz(tar_gz_file: Path, target_folder: Path):
    """
    解压缩tar.gz文件到指定文件夹

    :param tar_gz_file: tar.gz文件的路径
    :param target_folder: 目标文件夹的路径
    """
    with tarfile.open(tar_gz_file, "r:gz") as tar_ref:
        tar_ref.extractall(target_folder)


def sha256sum(file_path: Path) -> str:
    sha256 = hashlib.sha256()
    with open(file_path, "rb") as f:
        for chunk in iter(lambda: f.read(4096), b""):
            sha256.update(chunk)
    return sha256.hexdigest()


def get_sys_arch() -> str:
    return platform.machine().lower()


def get_sys_os() -> str:
    return platform.system().lower()


def save_json(file_name, json_data):
    with open(file_name, "w", encoding="utf-8") as f:
        json.dump(json_data, f, indent=4)


def load_json(file_name) -> list | dict:
    return json.load(open(file_name, encoding="utf-8"))


def mk_sure(path: str):
    Path(path).mkdir(parents=True, exist_ok=True)
    return Path(path)


def mk_symlink(src: str, dst: str, is_dir: bool = True):
    Path(dst).symlink_to(src, is_dir)
