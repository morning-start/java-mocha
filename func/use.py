import os

from func.config import Config


def switch_jdk(jdk: str, cfg: Config):
    """Switch JAVA_HOME environment variable."""
    java_home = cfg.jvm_root / "current"
    jdk_path = cfg.jdk_home / jdk
    # 软连接
    if jdk_path.exists():
        if java_home.exists():
            os.remove(java_home)
        os.symlink(jdk_path, java_home)
        return True
    else:
        return False
