# Java-Mocha 项目概述

## 项目简介

Java-Mocha 是一个基于 Rust 开发的 JDK 版本管理工具，它帮助用户在 Windows 系统上轻松安装、切换和管理多个 JDK 版本。该工具基于 Foojay API 开发，支持通过命令行界面进行 JDK 版本管理，也可以通过 API 集成使用。

## 主要功能

- **配置管理**：配置 JDK 目录、缓存目录和代理服务器
- **数据同步**：从 Foojay 同步 JDK 数据到本地 JSON 文件
- **列表显示**：查看本地 JDK、发布商和版本信息
- **查询功能**：根据发布商、版本号和支持周期查询可用的 JDK
- **安装和卸载**：安装和卸载指定的 JDK 版本
- **版本切换**：切换 JAVA_HOME 环境变量以指向不同的 JDK 版本

## 支持的 JDK 供应商

目前，Java-Mocha 仅支持以下 JDK 供应商：
- Oracle
- Oracle OpenJDK

## 系统要求

- Windows 操作系统
- Rust 开发环境（仅用于从源代码构建）

## 安装方法

### 使用 Scoop 安装

```bash
# 安装包
scoop bucket add code https://github.com/morning-start/code-bucket
scoop install code/jvm

# 更新包
scoop update jvm
```

### 从源代码构建

```bash
# 克隆仓库
git clone https://github.com/morning-start/java-mocha.git
cd java-mocha

# 构建项目
cargo build --release

# 运行项目
cargo run --release
```

## 使用说明

### 配置

在使用前，您可以通过以下命令配置 Java-Mocha：

```bash
jvm config
```

### 同步数据

首次使用前，需要同步数据以获取最新的 JDK 数据信息：

```bash
jvm sync
```

### 查看列表

```bash
# 查看本地 JDK
jvm list

# 查看发布商信息
jvm list --publisher

# 查看版本信息
jvm list --version
```

### 查询 JDK

```bash
# 按发布商查询
jvm query oracle

# 按发布商和版本号查询
jvm query oracle -v 23

# 按发布商和支持周期查询
jvm query oracle -t sts
```

### 安装 JDK

```bash
jvm install oracle@23
```

### 切换 JDK

```bash
jvm switch oracle@23.0.1
```

### 卸载 JDK

```bash
jvm uninstall oracle@23.0.1
```

## 环境变量

- **JVM_ROOT**：控制安装目录，默认值为 `%USERPROFILE%\.java-mocha`
- **JAVA_HOME**：需要手动配置为 `JVM_ROOT\default`

## 特点

1. 使用时不需要系统权限
2. 软件自动获取系统和架构信息以获取相应的 JDK 版本
3. JDK 信息基于 [foojay Disco API](https://github.com/foojayio/discoapi)，提供更好的可扩展性
4. 默认是无生产 JDK，不包括商业版本
5. 目前只处理没有 JavaFX 的 JDK
