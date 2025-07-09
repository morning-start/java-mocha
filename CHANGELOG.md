
## 变更日志 (Changelog)

### 版本 1.2.0 (2025-07-07)

- feat(func): 移除冗余导入并添加配置加载逻辑
- refactor(config): 修改对scoop升级的java版本bug

### 版本 1.1.0 (2025-07-07)

- feat: 添加对Oracle OpenJDK的支持并改进错误处理
- feat(install): 支持安装长期支持版本的 JDK
- feat(install): 添加跳过验证的功能

### 版本 1.0.2 (2025-07-07)

- feat(config): 重构配置管理并更新 JAVA_HOME 设置
- docs(README): 添加 scoop 安装支持

### 版本 1.0.1 (2025-07-07)

- build(release): 更新构建流程以使用标签名作为版本号
- fix(config): 修复配置保存和 JAVA_HOME 设置的问题
- feat: 添加 AGPL-3.0 许可证并更新 README_zh.md

### 版本 1.0.0 (2025-07-07)

- feat: 添加JDK切换功能并改进命令行界面
- refactor(config): 将load_jvm方法移至Config类并优化配置加载
- refactor: 重构文件操作和数据处理逻辑
- feat: 实现JDK切换和卸载功能
