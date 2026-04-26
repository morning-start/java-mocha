# Java-Mocha 用户指南

## 简介

Java-Mocha 是一个基于 Rust 开发的 JDK 版本管理工具，它帮助用户在 Windows 系统上轻松安装、切换和管理多个 JDK 版本。本指南将详细介绍如何使用 Java-Mocha 工具。

## 安装

### 使用 Scoop 安装

```bash
# 添加 scoop 桶
scoop bucket add code https://github.com/morning-start/code-bucket

# 安装 Java-Mocha
scoop install code/jvm

# 更新 Java-Mocha
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

## 配置

### 环境变量

在使用 Java-Mocha 之前，建议设置以下环境变量：

- **JVM_ROOT**：控制安装目录，默认值为 `%USERPROFILE%\.java-mocha`
- **JAVA_HOME**：需要手动配置为 `JVM_ROOT\default`

### 配置命令

```bash
jvm config [OPTIONS]
```

**选项**：
- `--jdk-home`：JDK 目录，默认为 JVM 根目录下的 `jdk` 目录
- `--cache-home`：缓存目录，默认为 JVM 根目录下的 `cache` 目录
- `--java-home`：JAVA_HOME 环境变量
- `--proxy`：代理服务器，默认为 `http_proxy` 环境变量

**示例**：

```bash
# 设置自定义 JDK 目录
jvm config --jdk-home "D:\Java\jdk"

# 设置代理服务器
jvm config --proxy "http://proxy.example.com:8080"
```

## 数据同步

首次使用 Java-Mocha 时，需要同步数据以获取最新的 JDK 信息：

```bash
jvm sync
```

该命令会从 Foojay API 获取最新的 JDK 数据，并保存到本地 JSON 文件中。

## 查看列表

### 查看本地 JDK

```bash
jvm list
```

此命令会显示所有已安装的 JDK 版本，并标记当前正在使用的版本。

### 查看发布商信息

```bash
jvm list --publisher
```

此命令会显示所有可用的 JDK 发布商信息。

### 查看版本信息

```bash
jvm list --version
```

此命令会显示所有可用的 JDK 版本信息。

### 查看发布商版本信息

```bash
jvm list --publisher --version
```

此命令会显示每个发布商可用的主版本信息。

## 查询 JDK

### 按发布商查询

```bash
jvm query <PUBLISHER>
```

**示例**：

```bash
jvm query oracle
```

此命令会显示指定发布商的最新 JDK 版本信息。

### 按发布商和版本号查询

```bash
jvm query <PUBLISHER> -v <MAJOR_VERSION>
```

**示例**：

```bash
jvm query oracle -v 23
```

此命令会显示指定发布商和主版本号的 JDK 版本信息。

### 按发布商和支持周期查询

```bash
jvm query <PUBLISHER> -t <SUPPORT_TERM>
```

**支持周期**：
- `sts`：短期支持
- `mts`：中期支持
- `lts`：长期支持

**示例**：

```bash
jvm query oracle -t lts
```

此命令会显示指定发布商和支持周期的最新 JDK 版本信息。

## 安装 JDK

```bash
jvm install <JDK> [OPTIONS]
```

**JDK 格式**：`publisher@version`，例如 `oracle@23`、`oracle@23.0.2`、`oracle@latest`、`oracle@lts`

**选项**：
- `-f, --force`：强制安装，即使已存在相同版本
- `-s, --skip-check`：跳过校验和验证

**示例**：

```bash
# 安装 Oracle JDK 23
jvm install oracle@23

# 强制安装并跳过校验和验证
jvm install oracle@23 -f -s
```

## 切换 JDK

```bash
jvm switch <JDK>
```

**JDK 格式**：`publisher@version`，例如 `oracle@23.0.1`

**示例**：

```bash
# 切换到 Oracle JDK 23.0.1
jvm switch oracle@23.0.1
```

此命令会更新 JAVA_HOME 环境变量并创建符号链接，使指定的 JDK 版本成为当前使用的版本。

## 卸载 JDK

```bash
jvm uninstall <JDK>
```

**JDK 格式**：`publisher@version`，例如 `oracle@23.0.1`

**示例**：

```bash
# 卸载 Oracle JDK 23.0.1
jvm uninstall oracle@23.0.1
```

此命令会删除指定的 JDK 版本。

## 常见问题

### 1. 首次使用时出现 "No JDK found, please install first." 错误

这是正常现象，首次使用时还没有安装任何 JDK 版本。您需要先运行 `jvm sync` 同步数据，然后使用 `jvm install` 命令安装 JDK。

### 2. 安装 JDK 时出现 "Download failed" 错误

可能的原因：
- 网络连接问题
- 代理服务器配置错误
- Foojay API 暂时不可用

解决方案：
- 检查网络连接
- 配置正确的代理服务器：`jvm config --proxy "http://proxy.example.com:8080"`
- 稍后重试

### 3. 切换 JDK 后 `java -version` 显示的版本没有变化

可能的原因：
- 环境变量没有立即生效
- 命令行窗口需要重新打开

解决方案：
- 关闭当前命令行窗口并重新打开
- 或者运行 `refreshenv` 命令（如果使用的是 PowerShell）

### 4. 卸载 JDK 后仍然显示在列表中

可能的原因：
- 卸载过程中出现错误
- 配置文件没有更新

解决方案：
- 手动删除 JDK 目录
- 检查配置文件并更新

## 最佳实践

1. **定期同步数据**：定期运行 `jvm sync` 以获取最新的 JDK 信息
2. **使用 LTS 版本**：对于生产环境，建议使用 LTS（长期支持）版本的 JDK
3. **保持环境变量正确**：确保 JAVA_HOME 环境变量设置为 `JVM_ROOT\default`
4. **合理管理 JDK 版本**：只安装和保留需要的 JDK 版本，避免占用过多磁盘空间

## 故障排除

### 查看详细日志

如果遇到问题，可以查看详细的日志信息：

```bash
# 设置 RUST_LOG 环境变量为 debug
set RUST_LOG=debug

# 运行 Java-Mocha 命令
jvm <command>
```

### 检查配置文件

配置文件位于 `JVM_ROOT\config.toml`，您可以手动检查和修改配置文件。

### 检查数据文件

数据文件位于 `JVM_ROOT\data` 目录，包括：
- `distributions.json`：发布商信息
- `versions.json`：版本信息
- `packages.json`：包信息

如果数据文件损坏，可以删除它们并重新运行 `jvm sync` 命令重新获取数据。

## 联系我们

如果您遇到任何问题或有任何建议，请在 GitHub 仓库中提交 issue：

[https://github.com/morning-start/java-mocha/issues](https://github.com/morning-start/java-mocha/issues)
