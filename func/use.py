from func.config import Config


def switch_jdk(jdk: str, cfg: Config):
    """Switch JAVA_HOME environment variable."""
    java_home = cfg.jvm_root / "current"
    jdk_path = cfg.jdk_home / jdk
    # 软连接
    if jdk_path.exists():
        if java_home.exists():
            if java_home.is_dir():
                java_home.rmdir()
            else:
                java_home.unlink()
        java_home.symlink_to(jdk_path, True)
        cfg = cfg.change_jdk(jdk)
        cfg.save()
        return True
    else:
        return False
