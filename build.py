import subprocess

APP_NAME = "jvm"
ICON = "logo.ico"
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
except subprocess.CalledProcessError as e:
    print(f"应用构建失败: {e}")
