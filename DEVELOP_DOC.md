# Java-Mocha 开发文档

## 项目概述
Java-Mocha 是一个用于管理 JDK 发行版的工具，支持查询、安装、卸载和切换 JDK 版本等功能。用户可以通过命令行界面轻松管理不同版本的 JDK。

## 项目结构
```plaintext
java-mocha/
├── .github/
│   └── workflows/
│       └── release.yml
├── .gitignore
├── .python-version
├── .vscode/
│   ├── launch.json
│   └── settings.json
├── README.md
├── build.py
├── core/
│   ├── foojay.py
│   ├── handler.py
│   ├── log.py
│   ├── style.py
│   ├── type.py
│   └── utils.py
├── func/
│   ├── config.py
│   ├── install.py
│   ├── list.py
│   ├── query.py
│   ├── sync.py
│   ├── uninstall.py
│   └── switch.py
├── main.py
├── pyproject.toml
└── uv.lock
```

### 主要目录和文件说明
- `core/`：存放核心功能模块，包含数据处理、日志记录、样式展示等功能的实现。
- `func/`：实现 JDK 的查询、安装、卸载、切换等具体功能。
- `build.py`：项目构建脚本，用于打包项目。
- `pyproject.toml`：项目依赖和配置文件，记录项目所需的依赖库及其版本。

### 构建项目
运行 `build.py` 脚本可以构建项目，该脚本会使用 `pyinstaller` 将项目打包成可执行文件，然后根据操作系统类型将打包文件压缩成对应的压缩包。在命令行中执行以下命令：

```bash
uv run build.py
```

## 开发流程
1. **克隆项目仓库**：从代码仓库克隆项目到本地。
2. **创建虚拟环境**：建议使用虚拟环境管理项目依赖，避免不同项目之间的依赖冲突。
3. **安装项目依赖**：运行 `uv sync` 安装项目所需的依赖库。
4. **开发新功能或修复问题**：在本地进行代码开发，添加新功能或修复现有问题。
5. **运行测试**：确保代码质量，在开发过程中编写并运行测试用例。
6. **提交代码**：将修改后的代码提交到代码仓库，并创建 Pull Request 进行代码审查。