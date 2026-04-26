# Java-Mocha API 参考文档

## 核心模块 API

### datatype.rs

#### PackageInfo

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    pub filename: String,
    pub direct_download_uri: String,
    pub download_site_uri: String,
    pub signature_uri: String,
    pub checksum_uri: String,
    pub checksum: String,
    pub checksum_type: String,
}
```

**方法**：
- `checksum_type(&self) -> &str`：获取校验和类型
- `has_checksum_uri(&self) -> bool`：检查是否有有效的 checksum_uri
- `has_checksum(&self) -> bool`：检查是否已有 checksum 值

#### 枚举类型

- `DataFile`：数据文件类型
- `Distribution`：JDK 发行版
- `VersionType`：版本类型
- `PackVersion`：包版本
- `SupportTerm`：支持周期
- `Architecture`：系统架构
- `OperatingSystem`：操作系统
- `ArchiveType`：归档类型
- `PkgType`：包类型

### style.rs

#### 函数

- `show_table(data: &Vec<Vec<String>>, title: &str) -> String`：显示表格
- `show_tree(data: &Vec<(String, String)>, current: &Option<String>, title: &str) -> String`：显示树状结构

## 功能模块 API

### config.rs

#### Config 结构体

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub jvm_root: PathBuf,
    pub jdk_home: PathBuf,
    pub java_home: PathBuf,
    pub cache_home: PathBuf,
    pub data_dir: PathBuf,
    pub jdk_version: Option<String>,
    pub proxy: Option<String>,
}
```

**方法**：
- `load() -> Result<Self, Box<dyn Error>>`：加载配置
- `load_jvm() -> PathBuf`：加载 JVM 根目录
- `save(&self) -> Result<(), Box<dyn Error>>`：保存配置

#### 函数

- `init_config(jvm_root: PathBuf, jdk_home: Option<PathBuf>, java_home: Option<PathBuf>, cache_home: Option<PathBuf>, proxy: Option<String>) -> Result<Config, Box<dyn Error>>`：初始化配置

### sync.rs

#### 函数

- `sync_data(cfg: &Config) -> impl Future<Output = ()>`：同步数据

### list.rs

#### 函数

- `list_local_jdk(jdk_home: &PathBuf) -> Vec<(String, String)>`：列出本地 JDK
- `list_publisher(data_dir: &PathBuf) -> Vec<Vec<String>>`：列出发布商
- `list_version(data_dir: &PathBuf) -> Vec<Vec<String>>`：列出版本
- `list_publish_version(data_dir: &PathBuf) -> Vec<Vec<String>>`：列出发布商版本

### query.rs

#### 函数

- `query_info(data_dir: &PathBuf, publisher: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>>`：查询发布商信息
- `query_info_version(data_dir: &PathBuf, publisher: &str, major_version: i32) -> Result<Vec<Vec<String>>, Box<dyn Error>>`：查询版本信息
- `query_info_term(data_dir: &PathBuf, publisher: &str, term_of_support: SupportTerm) -> Result<Vec<Vec<String>>, Box<dyn Error>>`：查询支持周期信息

### install.rs

#### 函数

- `full_install_process(jdk: &str, cfg: &Config, force: bool, skip_check: bool) -> impl Future<Output = Result<String, Box<dyn Error>>>`：完整安装流程

### switch.rs

#### 函数

- `switch_jdk(jdk: &str, cfg: &Config) -> Result<bool, Box<dyn Error>>`：切换 JDK 版本

### uninstall.rs

#### 函数

- `uninstall_jdk(jdk: &str, cfg: &Config) -> bool`：卸载 JDK

## 命令行接口

### 配置命令

```bash
jvm config [OPTIONS]
```

**选项**：
- `--jdk-home`：JDK 目录
- `--cache-home`：缓存目录
- `--java-home`：JAVA_HOME 环境变量
- `--proxy`：代理服务器

### 同步命令

```bash
jvm sync
```

### 列表命令

```bash
jvm list [OPTIONS]
```

**选项**：
- `--publisher`：显示发布商信息
- `--version`：显示版本信息

### 查询命令

```bash
jvm query <PUBLISHER> [OPTIONS]
```

**选项**：
- `-v, --major-version`：详细的主版本信息
- `-t, --term-of-support`：支持周期

### 安装命令

```bash
jvm install <JDK> [OPTIONS]
```

**选项**：
- `-f, --force`：强制安装
- `-s, --skip-check`：跳过校验和验证

### 切换命令

```bash
jvm switch <JDK>
```

### 卸载命令

```bash
jvm uninstall <JDK>
```

## 环境变量

- `JVM_ROOT`：控制安装目录，默认值为 `%USERPROFILE%\.java-mocha`
- `JAVA_HOME`：需要手动配置为 `JVM_ROOT\default`
- `http_proxy`：默认的代理服务器

## 使用示例

### 配置 Java-Mocha

```rust
use jvm::func::config::{Config, init_config};
use std::path::PathBuf;

let jvm_root = Config::load_jvm();
let config = init_config(
    jvm_root,
    Some(PathBuf::from("C:\\jdk")),
    Some(PathBuf::from("C:\\jdk\\default")),
    Some(PathBuf::from("C:\\cache")),
    Some("http://proxy.example.com:8080"),
).unwrap();
```

### 同步数据

```rust
use jvm::func::{config::Config, sync::sync_data};

#[tokio::main]
async fn main() {
    let config = Config::load().unwrap();
    sync_data(&config).await;
}
```

### 安装 JDK

```rust
use jvm::func::{config::Config, install::full_install_process};

#[tokio::main]
async fn main() {
    let config = Config::load().unwrap();
    let jdk_version = full_install_process("oracle@23", &config, false, false).await.unwrap();
    println!("Installed JDK: {}", jdk_version);
}
```

### 切换 JDK

```rust
use jvm::func::{config::Config, switch::switch_jdk};

fn main() {
    let config = Config::load().unwrap();
    switch_jdk("oracle@23.0.1", &config).unwrap();
}
```

### 卸载 JDK

```rust
use jvm::func::{config::Config, uninstall::uninstall_jdk};

fn main() {
    let config = Config::load().unwrap();
    uninstall_jdk("oracle@23.0.1", &config);
}
```
