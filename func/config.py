import os
from pathlib import Path
from typing import NamedTuple

from core.utils import load_json, save_json


class Config(NamedTuple):
    jvm_root: Path
    jdk_home: Path
    java_home: Path
    cache_home: Path
    data_dir: Path
    proxy: str = ""
    jdk_version: str = ""

    @staticmethod
    def load_jvm():
        jvm_root = Path.home() / ".java-mocha"
        jvm_root = Path(os.getenv("JVM_ROOT", jvm_root))
        return jvm_root

    def __repr__(self):
        info = self.to_json()
        return f"Config({info})"

    def __str__(self):
        return self.__repr__()

    def to_dict(self) -> dict[str, str | Path]:
        return self._asdict()

    def to_json(self):
        return {k: str(v) for k, v in self.to_dict().items()}

    def init_path(self):
        self.jdk_home.mkdir(parents=True, exist_ok=True)
        self.cache_home.mkdir(parents=True, exist_ok=True)
        self.data_dir.mkdir(parents=True, exist_ok=True)

    @classmethod
    def from_json(cls, json: dict[str, str]):
        """
        从 JSON 字符串加载配置
        """
        jvm_root = Path(json["jvm_root"])
        jdk_home = Path(json["jdk_home"])
        java_home = Path(json["java_home"])
        cache_home = Path(json["cache_home"])
        data_dir = Path(json["data_dir"])
        proxy = json["proxy"]
        jdk_version = json["jdk_version"]
        return cls(
            jvm_root, jdk_home, java_home, cache_home, data_dir, proxy, jdk_version
        )

    @classmethod
    def load(cls):
        """
        从 JVM 根目录加载配置
        """
        config_file = cls.load_jvm() / "config.json"
        cfg = load_json(config_file)
        return cls.from_json(cfg)

    def save(self):
        """
        保存配置到 JVM 根目录
        """
        save_json(self.jvm_root / "config.json", self.to_json())

    def change_jdk(self, jdk_version: str):
        cfg_dict = self.to_dict()
        cfg_dict["jdk_version"] = jdk_version
        return Config.from_json(cfg_dict)


def init_config(
    jvm_root: Path,
    jdk_home: Path = None,
    java_home: Path = None,
    cache_home: Path = None,
    proxy: str = None,
):
    """
    配置 Java Mocha 的 JVM 根目录、JDK 目录和缓存目录。
    Parameters:
    ----------
    jvm_root: Path
        JVM 根目录，默认为用户主目录下的 `.java-mocha` 目录。
    jdk_home: Path
        JDK 目录，默认为 JVM 根目录下的 `jdk` 目录。
    java_home: Path
        JAVA_HOME 环境变量，默认为 JVM 根目录下的 `default` 目录。
    cache_home: Path
        缓存目录，默认为 JVM 根目录下的 `cache` 目录。
    """
    # 默认值
    cfg_dict = {
        "jvm_root": jvm_root,
        "jdk_home": jvm_root / "jdk",
        "java_home": jvm_root / "default",
        "cache_home": jvm_root / "cache",
        "data_dir": jvm_root / "data",
        "proxy": "",
        "jdk_version": "",
    }

    # 如果有config
    if (jvm_root / "config.json").exists():
        cfg_dict.update(Config.load().to_dict())

    # 根据参数更新
    cfg_dict["jdk_home"] = jdk_home if jdk_home else cfg_dict["jdk_home"]
    cfg_dict["java_home"] = java_home if java_home else cfg_dict["java_home"]
    cfg_dict["cache_home"] = cache_home if cache_home else cfg_dict["cache_home"]
    cfg_dict["proxy"] = proxy if proxy else cfg_dict["proxy"]

    cfg = Config.from_json(cfg_dict)
    cfg.init_path()
    cfg.save()
