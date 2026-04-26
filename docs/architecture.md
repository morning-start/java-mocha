# Java-Mocha 架构文档

## 项目结构

Java-Mocha 采用模块化设计，代码结构清晰，主要分为核心模块（core）和功能模块（func）两大部分。

```
/src
  /core            # 核心模块
    datatype.rs    # 数据类型定义
    foojay.rs      # Foojay API 相关功能
    handler.rs     # 处理器
    mod.rs         # 核心模块导出
    style.rs       # 样式处理
    utils.rs       # 工具函数
  /func            # 功能模块
    config.rs      # 配置管理
    install.rs     # 安装功能
    list.rs        # 列表显示
    mod.rs         # 功能模块导出
    query.rs       # 查询功能
    switch.rs      # 版本切换
    sync.rs        # 数据同步
    uninstall.rs   # 卸载功能
  lib.rs           # 库导出
  main.rs          # 程序入口
```

## 模块说明

### 核心模块 (core)

#### datatype.rs

定义了项目中使用的各种数据类型，包括：
- `PackageInfo`：JDK 包信息
- `DataFile`：数据文件类型
- `Distribution`：JDK 发行版
- `VersionType`：版本类型
- `PackVersion`：包版本
- `SupportTerm`：支持周期
- `Architecture`：系统架构
- `OperatingSystem`：操作系统
- `ArchiveType`：归档类型
- `PkgType`：包类型

#### foojay.rs

处理与 Foojay API 的交互，包括：
- 从 Foojay API 获取 JDK 数据
- 解析 API 响应
- 处理 API 错误

#### handler.rs

提供各种处理器，包括：
- 数据处理
- 文件操作
- 网络请求

#### style.rs

处理命令行输出的样式，包括：
- 表格显示
- 树状显示
- 颜色处理

#### utils.rs

提供各种工具函数，包括：
- 文件操作
- 路径处理
- 字符串处理

### 功能模块 (func)

#### config.rs

处理配置管理，包括：
- 加载和保存配置
- 初始化配置
- 验证配置

#### sync.rs

处理数据同步，包括：
- 从 Foojay API 同步数据
- 保存数据到本地文件

#### list.rs

处理列表显示，包括：
- 显示本地 JDK
- 显示发布商信息
- 显示版本信息

#### query.rs

处理 JDK 查询，包括：
- 按发布商查询
- 按版本号查询
- 按支持周期查询

#### install.rs

处理 JDK 安装，包括：
- 下载 JDK
- 验证文件完整性
- 解压和安装

#### switch.rs

处理 JDK 版本切换，包括：
- 更新 JAVA_HOME 环境变量
- 创建符号链接

#### uninstall.rs

处理 JDK 卸载，包括：
- 删除 JDK 文件
- 更新配置

## 程序流程

### 配置流程

1. 用户执行 `jvm config` 命令
2. 程序加载当前配置
3. 用户可以指定 JDK 目录、缓存目录、JAVA_HOME 和代理服务器
4. 程序保存配置并提示用户设置 JAVA_HOME 环境变量

### 同步流程

1. 用户执行 `jvm sync` 命令
2. 程序加载配置
3. 从 Foojay API 获取最新的 JDK 数据
4. 将数据保存到本地 JSON 文件

### 安装流程

1. 用户执行 `jvm install oracle@23` 命令
2. 程序加载配置
3. 查询指定版本的 JDK 信息
4. 下载 JDK 安装包
5. 验证文件完整性
6. 解压并安装 JDK
7. 更新配置

### 切换流程

1. 用户执行 `jvm switch oracle@23.0.1` 命令
2. 程序加载配置
3. 检查指定版本的 JDK 是否存在
4. 更新 JAVA_HOME 环境变量
5. 创建符号链接

### 卸载流程

1. 用户执行 `jvm uninstall oracle@23.0.1` 命令
2. 程序加载配置
3. 检查指定版本的 JDK 是否存在
4. 删除 JDK 文件
5. 更新配置

## 数据流向

1. 从 Foojay API 获取 JDK 数据
2. 保存数据到本地 JSON 文件
3. 读取本地数据进行查询和安装
4. 安装的 JDK 存储在指定的目录
5. 通过符号链接实现版本切换

## 技术栈

- **Rust**：主要开发语言
- **Clap**：命令行参数解析
- **Tokio**：异步运行时
- **Serde**：序列化和反序列化
- **strum**：枚举处理
- **Foojay API**：获取 JDK 数据

## 扩展性

Java-Mocha 的设计具有良好的扩展性：

1. **模块化设计**：核心功能和业务逻辑分离，便于添加新功能
2. **Foojay API 集成**：基于 Foojay API 获取 JDK 数据，支持更多 JDK 供应商
3. **配置系统**：灵活的配置系统，支持自定义目录和代理
4. **命令行接口**：清晰的命令行接口，便于使用和扩展

## 安全性

1. **文件完整性验证**：下载 JDK 时验证文件完整性
2. **无系统权限要求**：使用普通用户权限即可运行
3. **环境变量管理**：安全地管理 JAVA_HOME 环境变量
