from core.utils import remove_directory
from func.config import Config


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
