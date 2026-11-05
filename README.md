# money_clock

> 基于 Vue 3 + Tauri 2 的模块化桌面应用基础架构

money_clock 是一个桌面应用工程骨架：前端使用 Vue 3 + TypeScript + Vite，桌面端使用 Tauri 2 + Rust，
两者通过明确的 IPC 契约通信。当前版本只包含基础架构、平台适配层与运行环境诊断页面，
**尚未实现时钟、天气、日历等任何具体业务功能**。

> 项目正式名称、仓库目录、应用名称、包名与 Rust crate 统一为 `money_clock`。
> 后续业务功能应在 `src/features/` 中扩展，不需要改动系统能力层。

## 技术栈

| 层次     | 技术                  | 版本 |
| -------- | --------------------- | ---- |
| 前端框架 | Vue 3                 | 3.5  |
| 开发语言 | TypeScript            | 5.9  |
| 前端构建 | Vite                  | 8.x  |
| 桌面容器 | Tauri 2 + Rust        | 2.x  |
| 状态管理 | Pinia                 | 4.x  |
| 路由     | Vue Router            | 5.x  |
| 样式     | Tailwind CSS          | 4.x  |
| 图标     | Lucide（@lucide/vue） | 1.x  |
| 包管理   | pnpm                  | 10.x |

## 环境要求

### 通用

- **Node.js** `>= 20.19.0`（推荐 22 LTS）
- **pnpm** `>= 10.0.0`（可通过 `corepack enable` 启用）
- **Rust** stable `>= 1.90.0`（Tauri 2.12 的最低要求；通过 [rustup](https://rustup.rs/) 安装，推荐使用最新 stable）
- **Tauri 2 系统依赖**：见下方各平台说明

### Windows

- **Microsoft C++ Build Tools**：安装「使用 C++ 的桌面开发」工作负载，包含 MSVC v143 与
  Windows 10/11 SDK
- **WebView2 Runtime**：Windows 10/11 通常已内置；缺失时安装 Evergreen Runtime

安装完成后建议在 **x64 Native Tools Command Prompt for VS** 或已加载 VS 开发环境的终端中执行 Rust 命令，
避免 Git Bash 自带的 `link.exe`（GNU coreutils）遮蔽 MSVC 链接器。

### macOS

```bash
xcode-select --install
```

WebKit 由系统提供，无需额外安装。

### Linux

以 Ubuntu / Debian 为例：

```bash
sudo apt update
sudo apt install -y build-essential curl wget file \
  libwebkit2gtk-4.1-dev libxdo-dev libssl-dev \
  libayatana-appindicator3-dev librsvg2-dev
```

其他发行版请参考
[Tauri 官方依赖说明](https://v2.tauri.app/start/prerequisites/)。

### 环境自检

```bash
node -v
pnpm -v
rustc --version
cargo --version
pnpm tauri info
```

`pnpm tauri info` 会检查 WebView2、Rust 工具链、Node 与 Tauri 配置是否就绪。

## 依赖安装

```bash
pnpm install
```

Rust 依赖会在首次构建时自动下载；如需提前拉取：

```bash
cargo fetch --manifest-path src-tauri/Cargo.toml
```

## 开发启动

### 桌面应用（推荐）

```bash
pnpm dev:tauri
```

该命令会先启动 Vite（固定端口 `1420`），再打开 Tauri 原生窗口，支持前端热更新。
前端或 Rust 代码修改后，Tauri 会自动重建。

### 仅前端预览

```bash
pnpm dev
```

浏览器打开 <http://localhost:1420>。此模式下 Tauri IPC 不可用，应用会自动进入
**浏览器预览模式**，不会调用任何系统能力，适合只调试界面。运行环境页面会明确提示该状态。

## 构建

```bash
# 前端类型检查 + 生产构建，输出到 dist/
pnpm build

# 完整桌面应用构建（先执行 pnpm build，再编译 Rust 并打包）
pnpm build:tauri
```

产物位置：`src-tauri/target/release/bundle/`

| 平台    | 产物                               |
| ------- | ---------------------------------- |
| Windows | `msi/`（WiX）、`nsis/`（安装程序） |
| macOS   | `dmg/`、`macos/*.app`              |
| Linux   | `deb/`、`rpm/`、`appimage/`        |

需要更快的调试构建时可使用 `pnpm tauri build --debug`。

## 代码规范与检查

| 命令                                              | 说明                                         |
| ------------------------------------------------- | -------------------------------------------- |
| `pnpm typecheck`                                  | `vue-tsc` 类型检查                           |
| `pnpm lint`                                       | ESLint 检查（Vue + TypeScript，flat config） |
| `pnpm lint:fix`                                   | ESLint 自动修复                              |
| `pnpm format`                                     | Prettier 格式化全部文件                      |
| `pnpm format:check`                               | 校验格式，不写入文件                         |
| `pnpm check:rust`                                 | `cargo check`                                |
| `pnpm fmt:rust`                                   | `cargo fmt`                                  |
| `pnpm lint:rust`                                  | `cargo clippy --all-targets -- -D warnings`  |
| `cargo test --manifest-path src-tauri/Cargo.toml` | 运行 Rust 单元测试（存储层）                 |

相关配置：

- `eslint.config.js`：ESLint flat config，Vue 推荐规则 + TypeScript 推荐规则
- `.prettierrc.json` / `.prettierignore`：统一格式化规则
- `.editorconfig`：跨编辑器基础约定（UTF-8、LF、2 空格，Rust 为 4 空格）
- `tsconfig.json`：严格模式、路径别名 `@/*`
- `src-tauri/rustfmt.toml`：Rust 格式化规则
- `.vscode/`：推荐扩展与保存时自动格式化配置

## 项目结构

```text
money_clock/
├── src/                          # 前端源码
│   ├── components/               # 跨模块复用组件
│   │   ├── common/               # 通用展示组件（Logo、卡片、状态标签等）
│   │   └── layout/               # 布局级组件（侧边栏、顶栏）
│   ├── features/                 # 业务域模块（当前仅 system 运行环境模块）
│   │   └── system/
│   │       ├── components/       # 业务域私有组件
│   │       └── index.ts          # 业务域公开入口
│   ├── layouts/                  # 页面布局骨架
│   ├── views/                    # 路由页面
│   ├── router/                   # 路由表与路由元信息
│   ├── services/                 # IPC 封装与前端服务
│   ├── stores/                   # Pinia 状态
│   ├── types/                    # 前后端共享契约类型
│   ├── assets/styles/            # Tailwind 入口与主题变量
│   ├── App.vue
│   └── main.ts
├── src-tauri/                    # Rust 后端
│   ├── capabilities/             # Tauri 权限能力配置
│   ├── icons/                    # 桌面应用图标（由 tauri icon 生成）
│   ├── src/
│   │   ├── commands/             # IPC 命令入口（app / platform / storage）
│   │   ├── services/             # 服务层编排（app / platform）
│   │   ├── storage/              # JSON 文档持久化
│   │   ├── platform/             # 平台适配层
│   │   │   ├── traits.rs         # PlatformAdapter 契约
│   │   │   ├── windows.rs
│   │   │   ├── macos.rs
│   │   │   ├── linux.rs
│   │   │   └── unsupported.rs
│   │   ├── error.rs              # 统一错误与序列化
│   │   ├── state.rs              # 共享应用状态
│   │   ├── lib.rs                # 应用装配与命令注册
│   │   └── main.rs               # 二进制入口
│   ├── build.rs
│   ├── Cargo.toml
│   ├── rustfmt.toml
│   └── tauri.conf.json
├── assets/branding/              # 应用图标源文件
├── scripts/                      # 开发辅助脚本（图标生成）
├── public/                       # 前端静态资源
├── index.html
├── vite.config.ts
├── tsconfig.json
├── eslint.config.js
├── package.json
├── pnpm-lock.yaml
└── README.md
```

## 架构说明

### 分层与依赖方向

```text
┌────────────────────────────────────────────────────┐
│ Vue 视图层                                          │
│ views / layouts / components / features            │
└──────────────────────┬─────────────────────────────┘
                       │ 只通过 services 访问系统能力
┌──────────────────────▼─────────────────────────────┐
│ 前端服务层 services + 状态层 stores                 │
│ IPC 封装 / 错误归一化 / 浏览器预览降级              │
└──────────────────────┬─────────────────────────────┘
                       │ invoke（JSON 契约）
┌──────────────────────▼─────────────────────────────┐
│ Rust commands（参数校验 / 响应序列化）              │
└──────────────────────┬─────────────────────────────┘
┌──────────────────────▼─────────────────────────────┐
│ services / storage / platform                      │
│ 业务编排 / 本地持久化 / 平台差异                    │
└────────────────────────────────────────────────────┘
```

依赖方向始终向下，任何一层都不反向依赖上层。

### 前端职责

- **views / layouts**：页面与布局只做编排和组合，不直接调用 Tauri API。
- **components**：跨模块复用的展示型组件，不持有业务状态。
- **features**：按业务域组织模块，包含该业务域私有的 components / stores / services / types，
  通过 `index.ts` 暴露稳定 API。当前只有 `system`（运行环境诊断）模块。
- **services**：唯一允许接触 IPC 的前端层。`services/ipc.ts` 统一封装 `invoke`、错误归一化与
  浏览器环境降级；其他模块只能导入具体 service。
- **stores**：Pinia 管理跨组件状态。`app.store.ts` 只保存应用外壳状态（运行环境、平台信息），
  业务状态应放到 feature 自己的 store 中。
- **types**：与 Rust 序列化结构一一对应的 TypeScript 类型（`camelCase`）。

### Rust 职责

- **commands**：webview 唯一入口，只负责参数接收与调用编排，不写业务逻辑。
- **services**：服务层编排，例如汇总应用元信息、组合平台适配器与 Tauri 路径信息。
- **storage**：`JsonDocumentStore` 负责目录解析、文档名校验、原子写入（临时文件 + rename）、
  读取、列出与删除。文件路径只由后端决定，前端无法传入路径。
- **platform**：`PlatformAdapter` 定义平台能力契约，Windows / macOS / Linux / 未知平台各自实现；
  `platform/mod.rs` 在编译期选择适配器。业务代码不写 `cfg!(target_os = ...)`。
- **state**：`AppState` 持有服务与存储实例，由 Tauri 托管并注入命令。
- **error**：`AppError` 统一错误类型，序列化为 `{ code, message }`，前端归一化为 `IpcError`。

### IPC 契约

| 命令                      | 参数          | 返回                | 说明                     |
| ------------------------- | ------------- | ------------------- | ------------------------ |
| `get_app_info`            | —             | `AppInfo`           | 应用名称、版本与构建模式 |
| `get_platform_info`       | —             | `PlatformInfo`      | 操作系统与平台适配能力   |
| `storage_list_documents`  | —             | `string[]`          | 列出已持久化的文档       |
| `storage_read_document`   | `name`        | `JsonValue \| null` | 读取 JSON 文档           |
| `storage_write_document`  | `name, value` | `void`              | 原子写入 JSON 文档       |
| `storage_delete_document` | `name`        | `boolean`           | 删除文档                 |

错误统一为 `{ code, message }`，由 `src/services/ipc.ts` 转换为 `IpcError`。

存储文档名限制为 `[A-Za-z0-9_-]{1,64}`，文件保存在应用数据目录的 `storage/*.json`，
例如 Windows 下为 `%APPDATA%\com.moneyclock.desktop\storage\`。

### 平台适配层

`PlatformAdapter` 声明两类信息：

- **capabilities**：系统托盘、全局快捷键、窗口特效、开机自启是否可用
- **windowDefaults**：是否使用系统标题栏、是否透明、最小窗口尺寸

当前实现：

| 适配器               | 说明                                           |
| -------------------- | ---------------------------------------------- |
| `WindowsAdapter`     | 托盘、快捷键、窗口特效、注册表自启             |
| `MacosAdapter`       | 菜单栏托盘、快捷键、vibrancy、LaunchAgent 自启 |
| `LinuxAdapter`       | 托盘/快捷键依赖桌面环境，不假设原生窗口特效    |
| `UnsupportedAdapter` | 未知平台的保守降级                             |

前端只消费 `PlatformInfo`，不判断操作系统，因此新增平台时无需改动页面代码。

### 设计约束

1. 不引入后端服务器，所有持久化均为本地 JSON 文件。
2. 系统能力只存在于 Rust 层；前端不直接访问文件系统或操作系统 API。
3. 业务逻辑与系统能力分离：`features/` 不导入 `@tauri-apps/api`。
4. 业务功能尚未实现，`features/` 只提供扩展约定，不包含时钟、收益等逻辑。
5. 仅安装基础依赖，未引入 UI 组件库、HTTP 客户端、数据库或 Tauri 插件。

## 应用窗口与配置

`src-tauri/tauri.conf.json`：

- 应用名：`money_clock`，标识：`com.moneyclock.desktop`
- 主窗口 `label: main`，默认 `1080 × 760`，最小 `900 × 620`，居中显示，保留系统标题栏
- 深色主题与 `#060a13` 窗口背景，减少启动时的白屏闪烁
- 开发地址：`http://localhost:1420`，生产资源：`../dist`
- 权限：`src-tauri/capabilities/default.json` 仅授予主窗口 `core:default`
- CSP：当前为 `null`（与 Tauri 默认模板一致），发布前应按实际资源收紧

## 图标

源图标为 `assets/branding/app-icon.png`，由 `scripts/generate-app-icon.py` 生成（依赖 Pillow）。
需要修改图标时：

```bash
python scripts/generate-app-icon.py
pnpm tauri icon assets/branding/app-icon.png
```

第二条命令会在 `src-tauri/icons/` 下生成各平台图标；若项目没有移动端目标，可删除生成的
`icons/android/` 与 `icons/ios/` 目录。

## 常见问题

### `link.exe` 报错或提示 `link: extra operand`

说明 Rust 找到的不是 MSVC 链接器（常见于 Git Bash 中 GNU `link` 抢先命中），或未安装
VS Build Tools 的 C++ 工作负载。请安装「使用 C++ 的桌面开发」工作负载，并在
**x64 Native Tools Command Prompt for VS** 中执行命令。

### `rustc: not installed`

安装 rustup 后需要重新打开终端，使 `%USERPROFILE%\.cargo\bin` 进入 `PATH`。

### 浏览器中运行提示 IPC 不可用

这是预期行为。请使用 `pnpm dev:tauri` 启动桌面窗口；浏览器预览模式不会调用任何系统能力。

### Linux 启动报 WebKitGTK 相关错误

确认已安装 `libwebkit2gtk-4.1-dev` 等系统依赖，并重新登录会话使库路径生效。

### 端口 1420 被占用

Vite 配置了 `strictPort: true`，请释放端口或修改 `vite.config.ts` 与
`tauri.conf.json` 中的端口配置。

## 开发状态

- [x] 初始化 Vue 3 + TypeScript + Vite 前端
- [x] 初始化 Tauri 2 + Rust 后端
- [x] 前端按 components / views / layouts / features / stores / services / types 分层
- [x] Rust 按 commands / services / storage / platform 分层
- [x] Windows / macOS / Linux 平台适配层
- [x] ESLint、Prettier、EditorConfig、rustfmt 配置
- [x] Tauri 窗口、权限、应用入口与图标
- [x] 前端构建、Rust 检查与单元测试
- [ ] 业务功能（时钟、收益展示等）
- [ ] 发布前收紧 CSP 与打包配置

## 许可证

本项目基于 [Apache License 2.0](LICENSE) 开源。
