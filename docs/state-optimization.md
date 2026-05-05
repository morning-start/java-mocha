# Java-Mocha 状态转换优化方案

## 一、优化理念

### 核心思想
**"标准状态 + 转换器"模式**：定义一个最小化的"标准状态"，所有复杂情况通过专用转换器转换成标准状态，然后按照统一的标准流程处理。

### 设计原则
1. **单一标准状态**：定义清晰、最小化的标准数据结构
2. **无差别处理**：所有输入经转换后，后续处理逻辑完全一致
3. **可扩展性**：新增场景只需添加新的转换器，无需修改核心逻辑

---

## 二、当前状态分析

### 2.1 现有数据结构

| 模块 | 文件 | 数据类型 | 职责 |
|------|------|----------|------|
| 核心类型 | `datatype.rs` | `PackageInfo`, `Distribution`, `VersionType` 等 | 定义基础数据结构 |
| API交互 | `foojay.rs` | `FooJay` | 处理 Foojay API 请求 |
| 文档处理 | `handler.rs` | `DocumentHandler` | JSON 文档操作 |
| 工具函数 | `utils.rs` | 各种工具函数 | 文件操作、下载、校验 |

### 2.2 当前流程痛点

**问题1：输入格式多样化**
```rust
// install.rs 中版本解析逻辑
let parts: Vec<&str> = jdk.split('@').collect();
if parts.len() != 2 { /* 错误处理 */ }
let distribution = parts[0];
let version = parts[1];

// 版本判断逻辑复杂
if version == "latest" { ... }
else if version == "lts" { ... }
else if version.chars().all(char::is_numeric) { ... }
else { ... }
```

**问题2：状态分散**
- 版本类型分布在多个地方（`VersionType`、`PackVersion`、`SupportTerm`）
- 架构和操作系统的别名处理分散在各自的 `from_str` 方法中

**问题3：缺乏统一状态机**
- 安装、切换、卸载流程各自独立实现
- 没有统一的状态转换管理

---

## 三、优化方案

### 3.1 标准状态定义

#### 3.1.1 核心标准状态

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JdkRequest {
    pub distribution: Distribution,
    pub version_spec: VersionSpec,
    pub target_os: OperatingSystem,
    pub target_arch: Architecture,
    pub pkg_type: PkgType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionSpec {
    Latest,
    LTS,
    STS,
    MTS,
    Major(i8),
    Exact(String),
    EA,
}
```

#### 3.1.2 操作状态机

```rust
pub enum OperationState {
    Initial,
    Validated(JdkRequest),
    Resolved(PackageInfo),
    Downloaded(PathBuf),
    Verified(PathBuf),
    Installed(String),
    Failed(Error),
}

pub enum OperationEvent {
    Validate(JdkRequest),
    Resolve,
    Download,
    Verify,
    Install,
    Fail(Error),
}
```

### 3.2 转换器设计

#### 3.2.1 输入转换器

```rust
pub trait InputConverter<T> {
    fn convert(input: T) -> Result<JdkRequest, Error>;
}

// 命令行输入转换器
pub struct CliInputConverter;

impl InputConverter<String> for CliInputConverter {
    fn convert(input: String) -> Result<JdkRequest, Error> {
        // 解析格式：distribution@version
        let parts: Vec<&str> = input.split('@').collect();
        
        let distribution = Distribution::from_str(parts[0])?;
        let version_spec = parse_version_spec(parts[1])?;
        
        Ok(JdkRequest {
            distribution,
            version_spec,
            target_os: OperatingSystem::get_local_os(),
            target_arch: Architecture::get_local_arch(),
            pkg_type: PkgType::Jdk,
        })
    }
}

// 版本规格解析器
fn parse_version_spec(version: &str) -> Result<VersionSpec, Error> {
    match version.to_lowercase().as_str() {
        "latest" => Ok(VersionSpec::Latest),
        "lts" => Ok(VersionSpec::LTS),
        "sts" => Ok(VersionSpec::STS),
        "mts" => Ok(VersionSpec::MTS),
        "ea" => Ok(VersionSpec::EA),
        v if v.chars().all(char::is_numeric) => {
            Ok(VersionSpec::Major(v.parse()?))
        }
        v => Ok(VersionSpec::Exact(v.to_string())),
    }
}
```

#### 3.2.2 状态转换器

```rust
pub struct VersionSpecConverter;

impl VersionSpecConverter {
    pub fn to_query_params(spec: &VersionSpec) -> UrlParams {
        let mut params = UrlParams::new();
        
        match spec {
            VersionSpec::Latest => params.add("version_by_definition", "latest_ga"),
            VersionSpec::LTS => params.add("version_by_definition", "latest_lts"),
            VersionSpec::STS => params.add("version_by_definition", "latest_sts"),
            VersionSpec::MTS => params.add("version_by_definition", "latest_mts"),
            VersionSpec::EA => params.add("version_by_definition", "latest_ea"),
            VersionSpec::Major(major) => params.add("jdk_version", major),
            VersionSpec::Exact(version) => params.add("version", version),
        }
        
        params
    }
    
    pub fn to_support_term(spec: &VersionSpec) -> Option<SupportTerm> {
        match spec {
            VersionSpec::LTS => Some(SupportTerm::LTS),
            VersionSpec::STS => Some(SupportTerm::STS),
            VersionSpec::MTS => Some(SupportTerm::MTS),
            _ => None,
        }
    }
}
```

### 3.3 统一流程处理

#### 3.3.1 状态驱动的安装流程

```rust
pub struct JdkInstaller {
    state: OperationState,
    config: Config,
}

impl JdkInstaller {
    pub fn new(config: Config) -> Self {
        Self {
            state: OperationState::Initial,
            config,
        }
    }
    
    pub async fn process(&mut self, event: OperationEvent) -> Result<(), Error> {
        self.state = match (&self.state, event) {
            (OperationState::Initial, OperationEvent::Validate(request)) => {
                OperationState::Validated(request)
            }
            
            (OperationState::Validated(request), OperationEvent::Resolve) => {
                let package_info = self.resolve_package(request).await?;
                OperationState::Resolved(package_info)
            }
            
            (OperationState::Resolved(info), OperationEvent::Download) => {
                let path = self.download_package(info).await?;
                OperationState::Downloaded(path)
            }
            
            (OperationState::Downloaded(path), OperationEvent::Verify) => {
                self.verify_package(path.clone()).await?;
                OperationState::Verified(path)
            }
            
            (OperationState::Verified(path), OperationEvent::Install) => {
                let version = self.install_package(path).await?;
                OperationState::Installed(version)
            }
            
            (_, OperationEvent::Fail(err)) => OperationState::Failed(err),
            
            _ => return Err("Invalid state transition".into()),
        };
        
        Ok(())
    }
    
    async fn resolve_package(&self, request: &JdkRequest) -> Result<PackageInfo, Error> {
        let params = VersionSpecConverter::to_query_params(&request.version_spec);
        params.add_iterable(request.distribution.as_ref(), "distribution");
        params.add_iterable(request.target_os.aliases(), "operating_system");
        params.add_iterable(request.target_arch.aliases(), "architecture");
        
        let foojay = FooJay::new(None, None);
        let result = foojay.search_packages(
            None, None, None,
            Some(vec![request.distribution]),
            Some(request.target_arch),
            Some(request.target_os),
            None, Some(request.pkg_type),
            VersionSpecConverter::to_support_term(&request.version_spec),
            None, None, None
        ).await?;
        
        parse_package_info(&result)
    }
    // ... 其他方法
}
```

### 3.4 目录结构调整

```
src/
├── core/
│   ├── datatype.rs        # 基础数据类型
│   ├── foojay.rs          # API 交互
│   ├── handler.rs         # 文档处理
│   ├── state.rs           # 标准状态定义（新增）
│   ├── converter.rs       # 转换器（新增）
│   └── utils.rs           # 工具函数
├── func/
│   ├── config.rs          # 配置管理
│   ├── install.rs         # 安装功能（重构）
│   ├── list.rs            # 列表显示
│   ├── query.rs           # 查询功能（重构）
│   ├── switch.rs          # 版本切换（重构）
│   ├── sync.rs            # 数据同步
│   └── uninstall.rs       # 卸载功能（重构）
├── lib.rs
└── main.rs
```

---

## 四、优化收益

### 4.1 代码质量提升

| 指标 | 优化前 | 优化后 |
|------|--------|--------|
| 版本解析代码行数 | ~30 行 | ~15 行 |
| 状态判断复杂度 | 高（分散多处） | 低（集中管理） |
| 新增场景改动 | 修改核心逻辑 | 新增转换器 |

### 4.2 可维护性提升

- **单一职责**：转换器只负责转换，处理器只负责处理
- **可测试性**：转换器可独立单元测试
- **可扩展性**：新增发行版/版本类型只需添加转换规则

### 4.3 流程可视化

```
输入 → [输入转换器] → 标准状态(JdkRequest)
                           ↓
                      [状态机处理]
                           ↓
              ┌───────────┴───────────┐
              ↓                       ↓
         已验证 → 已解析 → 已下载 → 已验证 → 已安装
              ↓                       ↓
              └───────────┬───────────┘
                          ↓
                       失败状态
```

---

## 五、实施步骤

### 阶段一：定义标准状态（1-2 天）
1. 创建 `state.rs`，定义 `JdkRequest` 和 `VersionSpec`
2. 创建状态机 `OperationState` 和 `OperationEvent`

### 阶段二：实现转换器（2-3 天）
1. 创建 `converter.rs`
2. 实现 `InputConverter` 特征
3. 实现 `VersionSpecConverter`

### 阶段三：重构功能模块（3-4 天）
1. 重构 `install.rs` 使用新状态机
2. 重构 `query.rs` 使用统一查询接口
3. 重构 `switch.rs` 和 `uninstall.rs`

### 阶段四：测试与验证（2 天）
1. 编写转换器单元测试
2. 编写状态机集成测试
3. 验证现有功能兼容性

---

## 六、风险评估

| 风险 | 等级 | 缓解措施 |
|------|------|----------|
| 现有功能兼容性 | 中 | 保留旧API包装层 |
| 转换器逻辑错误 | 中 | 完善单元测试 |
| 状态机设计缺陷 | 低 | 先进行设计评审 |
| 性能影响 | 低 | 转换器为纯函数，无额外开销 |

---

## 七、总结

通过引入"标准状态 + 转换器"模式，Java-Mocha 项目将获得：

1. **统一的状态表示**：所有 JDK 请求都转换为标准的 `JdkRequest`
2. **清晰的状态转换**：通过状态机管理复杂的操作流程
3. **高度可扩展**：新增输入格式或操作类型只需添加转换器
4. **易于测试**：转换器和状态机都可以独立测试

这种设计符合开闭原则（对扩展开放，对修改关闭），为项目未来的迭代打下坚实基础。