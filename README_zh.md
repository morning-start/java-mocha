# Java-Mocha

Java-Mocha 是一个基于 Foojay API 开发的 Java 版本管理工具，支持通过命令行界面进行 JDK 版本管理，也可通过 API 集成使用。


> 目前仅适配oracle，需要支持更多JDK厂商或者有其他改进可以提issue

支持 scoop 集成

```
scoop bucket add code https://github.com/morning-start/code-bucket
```

## 功能特性
- **配置管理**：可配置 JDK 目录、缓存目录和代理服务器。
- **数据同步**：从 Foojay 同步 JDK 数据到本地 JSON 文件。
- **列表展示**：查看本地 JDK、发行商和版本信息。
- **查询功能**：根据发行商、版本号和支持期限查询可用的 JDK。
- **安装卸载**：安装和卸载指定版本的 JDK。
- **版本切换**：切换 JAVA_HOME 环境变量指向的 JDK 版本。

## 使用方法

可以事先配置环境变量 `JVM_ROOT` 来控制安装目录，默认值为 `%USERPROFILE%\.java-mocha`。

注意事项：
1. 使用前需要手动配置好 `JAVA_HOME` 环境变量为 `JVM_ROOT/current`。
2. 首次使用需要同步数据，来获取最新的 JDK 数据信息。

特点：
1. 使用无需系统权限
2. 软件会自动获取系统和架构信息，以获得对应架构的 JDK 版本。
2. jdk信息基于 [foojay Disco API](https://github.com/foojayio/discoapi)，有更好的扩展性。
3. 默认为生产免费jdk，不包含商业版。
4. 目前仅处理不含 javafx 的jdk
4. 支持jdk厂商如下
    - oracle




### 配置

```bash
jvm config
```

### 同步数据
```bash
jvm sync
```

### 查看列表
```bash
# 查看本地 JDK
jvm list

# 查看发行商信息
jvm list --publisher

# 查看版本信息
jvm list --version
```

### 查询 JDK
```bash
# 根据发行商查询
jvm query oracle

# 根据发行商和版本号查询
jvm query oracle -v 23

# 根据发行商和支持期限查询
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

## 开发文档
更多详细信息请查看 [开发文档](d:\Workplace\APP\Python\java-mocha\DEVELOP_DOC.md)。

## Thanks

- [foojay Disco API](https://github.com/foojayio/discoapi)
- [typer](https://github.com/tiangolo/typer)
- [typer/command alias](https://github.com/fastapi/typer/issues/132#issuecomment-2417492805)
- [typer/-h](https://github.com/fastapi/typer/issues/537)
