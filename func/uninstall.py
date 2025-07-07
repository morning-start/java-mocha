import shutil

from func.config import Config


def uninstall_jdk(jdk: str, cfg: Config):
    """Uninstall JDK."""
    jdk_path = cfg.jdk_home / jdk
    if jdk_path.exists():
        shutil.rmtree(jdk_path)
        return True
    return False
