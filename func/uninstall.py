from pathlib import Path

from func.config import Config


def remove_directory(path: Path):
    """递归删除非空目录"""
    for item in path.iterdir():
        if item.is_dir():
            remove_directory(item)
        else:
            item.unlink()
    path.rmdir()


def uninstall_jdk(jdk: str, cfg: Config):
    """Uninstall JDK."""
    jdk_path = cfg.jdk_home / jdk
    if jdk_path.exists():
        if jdk_path.is_dir():
            remove_directory(jdk_path)
        else:
            jdk_path.unlink()
        return True
    return False
