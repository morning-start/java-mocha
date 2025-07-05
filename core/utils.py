import json
import platform
from pathlib import Path


def get_sys_arch() -> str:
    return platform.machine().lower()


def get_sys_os() -> str:
    return platform.system().lower()


def save_json(file_name, json_data):
    with open(file_name, "w", encoding="utf-8") as f:
        json.dump(json_data, f, indent=4)


def load_json(file_name) -> list:
    return json.load(open(file_name, encoding="utf-8"))


def mk_sure(path: str):
    Path(path).mkdir(parents=True, exist_ok=True)
    return Path(path)


def mk_symlink(src: str, dst: str, is_dir: bool = True):
    Path(dst).symlink_to(src, is_dir)
