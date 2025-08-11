# Java-Mocha 开发文档

Java-Mocha 是一个基于 Rust 的 JDK 版本管理工具，它可以帮助你在 Windows 系统上轻松安装、切换和管理多个 JDK 版本。

## 项目结构

```
java-mocha/
├── src/
│   ├── core/           # 核心模块
│   │   ├── datatype.rs # 数据类型定义
│   │   ├── foojay.rs   # FooJay API 交互
│   │   ├── handler.rs  # JSON 数据处理
│   │   ├── style.rs    # 样式和显示
│   │   └── utils.rs    # 工具函数
│   ├── func/           # 功能模块
│   │   ├── config.rs   # 配置管理
│   │   ├── install.rs  # JDK 安装
│   │   ├── list.rs     # JDK 列表显示
│   │   ├── query.rs    # JDK 查询
│   │   ├── switch.rs   # JDK 切换
│   │   ├── sync.rs     # 数据同步
│   │   └── uninstall.rs# JDK 卸载
│   ├── lib.rs          # 库入口
│   └── main.rs         # 程序入口
├── Cargo.toml          # 项目配置
└── README.md           # 项目说明
```

## 核心模块

### datatype.rs

定义了项目中使用的核心数据类型：

- `PackageInfo`: JDK 包信息，包含下载链接、校验和等
- `Distribution`: JDK 发行版枚举，如 Oracle、OracleOpenJdk 等
- `Architecture`: 系统架构枚举
- `OperatingSystem`: 操作系统枚举
- `ArchiveType`: 压缩包类型枚举
- `SupportTerm`: 支持期限枚举

### foojay.rs

与 [FooJay API](https://api.foojay.io/) 交互的模块，提供以下功能：

- `search_distributions`: 搜索 JDK 发行版
- `search_versions`: 搜索 JDK 版本
- `search_packages`: 搜索 JDK 包

### handler.rs

JSON 数据处理器，提供数据查询、筛选、排序等功能。

### style.rs

样式和显示模块，负责在终端中格式化输出：

- `show_table`: 以表格形式显示数据
- `show_tree`: 以树形结构显示版本列表

### utils.rs

工具函数模块，包含：

- 文件解压缩（zip, tar.gz）
- HTTP 客户端构建
- 文件下载和进度显示
- SHA256 校验和计算
- JSON 数据读写
- 符号链接创建

## 功能模块

### config.rs

配置管理模块，负责：

- JVM 根目录管理
- 配置文件加载和保存
- 路径初始化

### sync.rs

数据同步模块，从 FooJay API 同步 JDK 发行版、版本和包信息到本地 JSON 文件。

### query.rs

JDK 查询模块，支持：

- 按发行版查询
- 按版本查询
- 按支持期限查询

### install.rs

JDK 安装模块，实现完整的安装流程：

1. 查询包 URL
2. 获取包信息和校验和
3. 下载并校验文件
4. 解压缩到目标目录

### switch.rs

JDK 切换模块，通过创建符号链接实现 JDK 版本切换。

### list.rs

JDK 列表显示模块，支持显示：

- 本地已安装的 JDK
- 支持的发行版
- 可用版本信息

### uninstall.rs

JDK 卸载模块，负责删除已安装的 JDK 目录。

## 构建和运行

### 环境要求

- Rust 1.60+
- Windows 系统

### 构建

```bash
cargo build --release
```

### 运行

```bash
cargo run --release
```

### 测试

```bash
cargo test
```

## API 使用

Java-Mocha 可以作为命令行工具使用，也提供了 API 供其他程序集成。

主要的 API 包括：

- `sync_data`: 同步 JDK 数据
- `query_info`: 查询 JDK 信息
- `full_install_process`: 安装 JDK
- `switch_jdk`: 切换 JDK 版本
- `uninstall_jdk`: 卸载 JDK

## 贡献指南

欢迎提交 Issue 和 Pull Request 来改进 Java-Mocha。

### 代码规范

- 遵循 Rust 官方编码规范
- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量

### 提交 Pull Request

1. Fork 项目
2. 创建功能分支
3. 提交更改
4. 发起 Pull Request