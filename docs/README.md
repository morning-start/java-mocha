# Java-Mocha 文档

欢迎使用 Java-Mocha 文档！本目录包含了 Java-Mocha 项目的详细文档，帮助您了解和使用这个 JDK 版本管理工具。

## 文档结构

- [项目概述](project-overview.md)：介绍 Java-Mocha 的基本信息、功能和使用方法
- [架构文档](architecture.md)：详细描述项目的代码结构和模块关系
- [API 参考](api-reference.md)：详细描述项目中主要的 API 和函数
- [用户指南](user-guide.md)：详细说明如何使用 Java-Mocha 工具

## 项目简介

Java-Mocha 是一个基于 Rust 开发的 JDK 版本管理工具，它帮助用户在 Windows 系统上轻松安装、切换和管理多个 JDK 版本。该工具基于 Foojay API 开发，支持通过命令行界面进行 JDK 版本管理，也可以通过 API 集成使用。

## 主要功能

- **配置管理**：配置 JDK 目录、缓存目录和代理服务器
- **数据同步**：从 Foojay 同步 JDK 数据到本地 JSON 文件
- **列表显示**：查看本地 JDK、发布商和版本信息
- **查询功能**：根据发布商、版本号和支持周期查询可用的 JDK
- **安装和卸载**：安装和卸载指定的 JDK 版本
- **版本切换**：切换 JAVA_HOME 环境变量以指向不同的 JDK 版本

## 快速开始

1. **安装 Java-Mocha**：使用 Scoop 安装或从源代码构建
2. **配置环境变量**：设置 JVM_ROOT 和 JAVA_HOME
3. **同步数据**：运行 `jvm sync` 获取最新的 JDK 信息
4. **安装 JDK**：运行 `jvm install oracle@23` 安装指定版本的 JDK
5. **切换 JDK**：运行 `jvm switch oracle@23.0.1` 切换到指定版本的 JDK

## 支持的 JDK 供应商

目前，Java-Mocha 仅支持以下 JDK 供应商：
- Oracle
- Oracle OpenJDK

## 系统要求

- Windows 操作系统
- Rust 开发环境（仅用于从源代码构建）

## 联系我们

如果您遇到任何问题或有任何建议，请在 GitHub 仓库中提交 issue：

[https://github.com/morning-start/java-mocha/issues](https://github.com/morning-start/java-mocha/issues)
