from pathlib import Path

from core.utils import save_json


def set_config(jvm_root: Path, jdk_home: Path, cache_home: Path):
    """
    配置 Java Mocha 的 JVM 根目录、JDK 目录和缓存目录。
    Parameters:
    ----------
    jvm_root: Path
        JVM 根目录，默认为用户主目录下的 `.java-mocha` 目录。
    jdk_home: Path
        JDK 目录，默认为 JVM 根目录下的 `jdk` 目录。
    cache_home: Path
        缓存目录，默认为 JVM 根目录下的 `cache` 目录。
    """
    # Initialize the configuration file
    config_file = jvm_root / ".java-mocha" / "config.json"
    jdk_home = jvm_root / "jdk" if jdk_home is None else jdk_home
    cache_home = jvm_root / "cache" if cache_home is None else cache_home
    cfg = {
        "jvm_root": str(jvm_root),
        "jdk_home": str(jdk_home),
        "cache_home": str(cache_home),
    }
    jvm_root.mkdir(parents=True, exist_ok=True)
    jdk_home.mkdir(parents=True, exist_ok=True)
    cache_home.mkdir(parents=True, exist_ok=True)

    save_json(config_file, cfg)
