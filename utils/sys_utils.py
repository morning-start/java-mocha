# 获取架构和系统
import platform


def get_sys_arch() -> str:
    return platform.machine().lower()


def get_sys_os() -> str:
    return platform.system().lower()
