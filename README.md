# LanChat 视觉识别插件

视觉识别从 LanChat 核心程序中完整拆出，以独立插件交付。核心安装包不包含视觉页面、识别逻辑、ORT/OpenVINO、模型脚本或模型资源。

## 架构

- `web/`：插件页面，通过标准 Host API 接入主题、通知、存储和生命周期。
- `sidecar/`：签名的本地识别进程，只开放受约束的 IPC 协议。
- `models/`：模型清单和下载规则；二进制模型不进入 Git 仓库。
- `packaging/`：模型准备、校验与插件发布脚本。

视觉插件需要 `native.sidecar` 权限。正式包必须由官方流水线签名，宿主在安装和启动前校验清单、哈希与签名。

## 本地验证

```powershell
npm ci
npm run build
cargo test --manifest-path sidecar/Cargo.toml --no-fail-fast
```

`npm run build` 生成插件 Web 入口，Rust 测试覆盖模型清单、运行时、跟踪、识别、告警和人员库。模型文件由发布流水线根据 `models/builtin/object-models/manifest*.json` 准备，不提交到仓库。

## 与核心程序的边界

插件只通过 LanChat 标准 Host API 获取设备、主题、私有存储和通知能力。摄像头帧传输与本地识别进程都属于插件；核心程序不链接 ORT/OpenVINO，也不包含视觉识别页面、模型和构建脚本。
