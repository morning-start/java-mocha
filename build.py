# 将 dist/*打包为一个压缩包文件，根据系统类型，后缀名不同
import platform
import subprocess
import tarfile
import zipfile
from pathlib import Path


def compress_files(files: list[Path], output: Path):
    """
    压缩文件为一个压缩包文件
    :param files: 要压缩的文件列表
    :param output: 压缩包文件路径
    """
    if platform.system() == "Windows":
        with zipfile.ZipFile(output, "w", zipfile.ZIP_DEFLATED) as zipf:
            for file in files:
                zipf.write(file, file.name)
    else:
        with tarfile.open(output, "w:gz") as tar:
            for file in files:
                tar.add(file, file.name)


APP_NAME = "jvm"
ICON = "logo.ico"
VERSION = "1.0.0"
# 构建 pyinstaller 命令
cmd = [
    "pyinstaller",
    "--onefile",
    # f"--icon={ICON}",
    f"--name={APP_NAME}",
    "main.py",  # 假设主程序文件名为 main.py，可按需修改
]

try:
    # 执行 pyinstaller 命令
    subprocess.run(cmd, check=True)
    print("应用构建成功！")
    # 压缩 dist 目录下的文件为一个压缩包文件
    dist_dir = Path("dist")
    export = Path("export")
    export.mkdir(exist_ok=True)
    platform_name = platform.system()
    compress_files(
        dist_dir.glob("*"),
        export / f"{APP_NAME}-{platform_name.lower()}-{VERSION}.zip",
    )
except subprocess.CalledProcessError as e:
    print(f"应用构建失败: {e}")
