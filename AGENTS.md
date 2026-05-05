# Skills Manager System

## 参考项目

- **路径**: `C:\Users\lt\Desktop\Write\custom-project\skills-manager`
- **仓库**: fork 自 `xingkongliang/skills-manager`
- **技术栈**: Tauri 2 + React 19 + TypeScript + Vite + Tailwind CSS + Rust + SQLite
- **用途**: 架构和实现模式参考（agent 检测、symlink/copy 同步方案等），本系统功能范围远小于该参考项目

## Skill 仓库

本系统管理的目标仓库：

- **本地路径**: `C:\Users\lt\Desktop\Write\custom-project\my-skills`（可在设置中更改）
- **远端**: `git@github.com:OCDcreator/my-skills.git`
- **分支**: `main`

### 仓库结构

```
my-skills/
├── custom/                  # 自编写的 skill（18+个）
│   ├── searxng/
│   │   └── SKILL.md
│   ├── opencode-loop/
│   │   └── SKILL.md
│   └── ...
├── external/                # 从上游克隆的外部 skill 源（10+个）
│   ├── anthropics-skills/
│   ├── awesome-claude-skills/
│   └── ...
├── automation/              # 自动化脚本
├── docs/                    # 状态文档
├── update.sh / update.bat   # 同步外部源
├── push.sh / push.bat       # 推送本地改动
└── pull.sh / pull.bat       # 拉取远端更新
```

- 每个 skill 是一个目录，根目录包含 `SKILL.md`
- `custom/` 下是自己编写的 skill，`external/` 下是从 GitHub 克隆的上游 skill 源

## 项目目标

构建一个**轻量级**桌面应用，管理 my-skills 仓库，核心做三件事：

1. **Git 同步 UI** — 浏览仓库状态、查看 diff、push/pull，替代手动跑脚本
2. **Skill 分配** — 将 skill 分配给不同的 AI agent（通过 symlink 或 copy，复用参考项目的方案）
3. **场景/项目管理** — 按场景或项目配置不同的 skill 分配方案，快速切换

### 不做什么（与参考项目的区别）

- ❌ Marketplace / 技能商店
- ❌ 复杂的安装流程（zip 导入、归档安装）
- ❌ AI 搜索
- ❌ 版本快照（Git 本身就是版本控制）
- ❌ 系统托盘 / 后台常驻
- ❌ 自动更新
- ❌ 批量操作 / 复杂标签系统

## 平台支持

- **Windows**（主要开发平台）
- **macOS**（必须兼容）

文件路径、shell 命令、文件系统操作必须考虑双平台差异。

## 技术栈

| 层 | 技术 |
|----|------|
| 前端 | React + TypeScript + Vite + Tailwind CSS |
| 桌面 | Tauri 2 |
| 后端 | Rust（Tauri commands） |
| 存储 | SQLite |
| 国际化 | react-i18next（中英文） |

## 项目结构（规划）

```
skills-manager-system/
├── src/                    # React 前端
│   ├── components/         # UI 组件
│   ├── context/            # React Context（全局状态）
│   ├── i18n/               # 国际化资源
│   ├── lib/                # Tauri API 封装
│   ├── views/              # 页面视图
│   ├── App.tsx
│   └── main.tsx
├── src-tauri/              # Rust 后端
│   └── src/
│       ├── commands/       # Tauri 命令层（薄层）
│       ├── core/           # 业务逻辑
│       ├── lib.rs
│       └── main.rs
├── AGENTS.md               # 本文件
└── package.json
```

## 模块化开发强约束

本项目必须保持“**模块化，但不碎片化**”。下面不是建议，而是默认硬约束；任何实现如果违反这些规则，应先重构到满足约束，再继续开发。

### 1. 文件厚度约束

- 默认目标：
  - `src/**/*.ts`、`src/**/*.tsx`、`src-tauri/src/**/*.rs`：**120-300 行**
  - `src/views/**/*.tsx`：**150-320 行**
  - `src/App.tsx`、`src/main.tsx`、`src-tauri/src/lib.rs`、`src-tauri/src/main.rs`、`src-tauri/src/commands/*.rs`：**80-180 行**
- 预警线：
  - 通用 TS/TSX/Rust 文件 **超过 320 行**，必须先评估拆分
  - `views` 文件 **超过 350 行**，必须优先拆分
  - 入口文件、Tauri command 文件 **超过 200 行**，必须优先拆分
- 硬上限：
  - 通用 TS/TSX/Rust 文件 **不得超过 480 行**
  - `views` 文件 **不得超过 480 行**
  - 入口文件、Tauri command 文件 **不得超过 300 行**
- 例外：
  - i18n JSON、枚举/常量映射、协议定义、测试数据可适度放宽
  - 但即使放宽，也必须保持**单一用途**，不能借例外堆业务逻辑
- 执行规则：
  - 一个文件一旦超过对应预警线，**禁止继续往里追加新逻辑**
  - 正确做法是：先拆分，再继续功能开发
  - 行数只是**结构告警器**，不是唯一标准；真正的硬约束仍然是职责边界和领域边界

### 2. 模块职责边界

- `src/views/`：页面级编排，只负责组装页面结构、调用 hooks/service、传递必要参数，不承载大段业务规则
- `src/components/`：可复用 UI 模块；共享组件与页面专属组件应分清，不要把页面逻辑塞进通用组件
- `src/context/`：全局状态与跨页共享状态；不要放 Git、文件系统、数据库等重 I/O 逻辑
- `src/lib/`：前端侧 Tauri API 封装、领域服务、数据转换；不要把页面 UI 拼装代码放进这里
- `src-tauri/src/commands/`：Tauri 命令层必须保持薄，只做参数接收、错误映射、调用 `core`
- `src-tauri/src/core/`：Rust 业务逻辑中心，Git、skill 扫描、agent 分配、场景切换等都应在这里按领域拆分
- 前端不得直接跨层调用底层实现细节；例如页面不要直接拼命令细节，command 不要直接膨胀成业务中心

### 3. 强制拆分触发条件

出现以下任一情况，就不是“可以拆”，而是“必须拆”：

- 一个文件同时承担 **页面渲染 + 状态管理 + 数据获取 + 业务规则** 中的 3 项及以上职责
- 一个文件同时处理 **Git 同步、Skill 扫描、Agent 分配、场景配置、项目分配** 中的 2 个及以上领域
- React 组件内部出现明显的大段 `useEffect`、数据整理、事件分发、视图分支混在一起
- Rust command 文件里开始出现路径解析、数据库读写、Git 操作、symlink/copy 策略等核心逻辑
- 同一文件需要依靠“第 1 段 / 第 2 段 / 第 3 段”这种人为分区才能勉强阅读

### 4. 防碎片化约束

- 禁止把项目拆成大量 **10-30 行** 的空壳文件，只为了表面上“看起来模块化”
- 新建文件前先问：它是否代表一个**稳定的职责单元**？如果只是简单转发、薄包装、顺手抽离，不应单独成文件
- 新文件如果只是：
  - 一个几乎不复用的超薄包装层
  - 一个只导出 1-2 个简单函数且与所属模块强耦合的碎片
  - 一个只是为了绕开行数限制而机械剪开的半成品
  - 那么应优先并回所属模块
- 禁止出现无领域语义的垃圾桶文件名，如 `utils.ts`、`helpers.ts`、`common.ts`、`misc.rs`
- 工具模块必须带领域前缀，例如 `git-status.ts`、`skill-parser.ts`、`agent-paths.rs`
- 新建子目录必须满足其一：
  - 该目录下会稳定存在 **3 个及以上相关文件**
  - 它代表一个明确的子领域边界（如 `git/`、`skills/`、`scenes/`、`agents/`）
- 如果拆分后出现多层透传、频繁 re-export、到处跳文件才能看懂一条流程，说明已经过度碎片化，应回收合并

### 5. 推荐拆分方式

- 按**领域**拆分：`git`、`skills`、`agents`、`scenes`、`projects`
- 按**层次**拆分：view / component / hook / service / command / core
- 按**职责**拆分：扫描器、解析器、同步器、仓库访问、状态计算、表单状态、列表渲染
- 优先提取“有独立输入输出边界”的模块，不要优先提取纯转发层

### 6. 开发与交付强制检查

每次新增功能、重构或修 bug，必须同步做以下自检：

- 是否有文件超过本节定义的预警线或硬上限
- 是否把本应在 `core` 的逻辑塞进了 `command` / `view`
- 是否新增了没有明确领域归属的 `utils`/`helpers` 类型文件
- 是否为了控行数而制造了过多超薄文件
- 是否一个目录只有 1-2 个文件却额外多包了一层目录

如果答案里有任一项是“是”，则当前实现不算完成。

### 7. 面向本项目的默认落地规则

- 新增前端功能时，优先采用“`view` + 领域组件 + 领域 service/hook”结构，而不是把页面写成超大总控文件
- 新增 Rust 能力时，优先采用“`commands` 薄层 + `core` 领域模块”结构，不允许在 command 中直接堆完整业务流程
- 参考 `skills-manager` 的实现时，要学习其**模块边界和职责划分**，不要照搬成一个大而全文件
- 如果以后增加测试，也应保持测试文件按领域归属，不要做一个超大集成测试文件吞掉所有场景

## 核心功能模块

### 1. Git 同步 UI
- 显示仓库当前状态（有无未提交改动、是否与远端同步）
- 查看 diff（文件级和内容级）
- 执行 push / pull / commit
- 触发 `update.sh`/`update.bat` 同步外部源
- 显示操作日志

### 2. Skill 浏览
- 扫描 `custom/` 和 `external/` 下的所有 skill 目录
- 解析每个 skill 的 `SKILL.md` 元数据（名称、描述、触发条件等）
- 按来源（custom/external）筛选、搜索 skill
- 启用 / 禁用 skill

### 3. Agent 分配
- 检测本机已安装的 AI 编码工具及其 skill 目录位置
- 支持的 Agent：Cursor、Claude Code、Codex、OpenCode、Amp、Kilo Code、Roo Code、Goose、Gemini CLI、GitHub Copilot、Windsurf 等
- 通过 symlink 或 copy 将选定的 skill 同步到目标 agent 的 skill 目录
- 支持自定义工具路径覆盖
- 参考 skills-manager 的 `tool_adapters.rs` 实现方案

### 4. 场景管理
- 创建、切换、删除场景
- 每个场景配置：启用的 skill 列表 + 目标 agent 列表
- 切换场景时自动更新 agent 目录的 symlink/copy
- 拖拽排序 skill 优先级

### 5. 项目分配
- 为特定项目目录配置 skill 分配
- 将 skill 写入项目本地的 agent 配置目录（如 `.claude/skills`、`.opencode/skills`、`.agents/skills`）
- 项目分配独立于全局场景，可以叠加

## 开发命令

```bash
npm install                    # 安装前端依赖
npm run tauri:dev              # 桌面应用开发模式
npm run dev                    # 仅前端开发
npm run build                  # 前端构建
npm run tauri:build            # 完整桌面应用构建
npm run check:architecture     # 架构约束检查（大文件 / 垃圾桶命名 / 过浅模块）
npm run verify                 # 当前最小 verify gate
npm run lint                   # ESLint 检查
cargo check --manifest-path src-tauri/Cargo.toml  # Rust 快速验证
cargo test --manifest-path src-tauri/Cargo.toml   # Rust 测试
cargo build --manifest-path src-tauri/Cargo.toml --no-default-features --features cli --bin skills-manager  # CLI-only 构建
./src-tauri/target/debug/skills-manager --help    # CLI 二进制 smoke
```

## 注意事项

- 用户界面文本使用 i18n，新增/修改文案需同步 `src/i18n/en.json`、`src/i18n/zh.json`
- Tauri 命令添加流程: Rust 实现 → `lib.rs` 的 `generate_handler!` 注册 → TS 封装/类型 → 前端调用
- CLI 命令添加流程: `src-tauri/src/cli/command_groups.rs` 参数 → `src-tauri/src/cli/mod.rs` 分发 → `src-tauri/src/cli/commands/` 适配器 → 对应模块文档与 CLI-only 测试
- 业务逻辑放 `src-tauri/src/core/`，Tauri 命令层 `src-tauri/src/commands/` 保持轻薄
- 提交前至少运行 `npm run verify`；如果后续接入 lint/test/build，继续把它们并入 `verify`
- Skill 仓库路径可在设置中配置，默认指向用户的 my-skills 仓库
- 文件系统操作注意 Windows (反斜杠) 与 macOS (正斜杠) 的路径差异
- symlink 在 Windows 上需要开发者模式或管理员权限，copy 模式作为备选
- 没有统一 formatter 配置时，匹配周围代码风格

## Module Documentation Guard

- `src/**/*.ts`、`src/**/*.tsx` 的模块文档一对一映射到 `docs/modules/frontend/**/*.md`
- `src-tauri/src/**/*.rs` 的模块文档一对一映射到 `docs/modules/tauri/**/*.md`
- 新增源码文件时，必须同时新增对应模块文档
- 修改源码文件时，必须在同一分支同步更新对应模块文档
- 删除源码文件时，必须删除对应模块文档
- 重命名源码文件时，必须同步重命名或替换对应模块文档
- 允许不参与覆盖校验的仅限 `docs/modules/README.md`、`docs/modules/_TEMPLATE.md`、`docs/modules/_WORKFLOW.md`、`docs/modules/frontend/README.md`、`docs/modules/tauri/README.md`，其他例外必须写进 `module-docs.config.json`
- 提交前必须运行：
  - `node scripts/check-module-doc-coverage.mjs`
  - `node scripts/check-module-doc-diff.mjs --range <base>...HEAD`
- `check-module-doc-diff` 默认会把 `<base>...HEAD` 与当前工作区（staged / unstaged / untracked）一起纳入校验，避免“文档已写但尚未提交”或“改了源码但本地还没补文档”漏检
- 需要定位当前分支必须更新哪些文档时，运行 `node scripts/list-module-doc-targets-from-diff.mjs --range <base>...HEAD`

<!-- lean-ctx:start -->
# lean-ctx — Directory Context Compression

lean-ctx compresses the current directory's code into a compact context cache, allowing the AI to "remember" what it was working on when you switch directories.

## How It Works

lean-ctx hooks into your shell's `cd` command. Every time you change directories:
1. **On exit** (leaving a dir): Compresses the directory's code into a cache file
2. **On enter** (entering a dir): Decompresses the cache back into the AI context

This means you can work on Project A, `cd` to Project B, and when you `cd` back to Project A, the AI still remembers the full context without re-reading all files.

## Usage

### Interactive Commands (in terminal)

```bash
lean-ctx              # Start MCP server (stdio)
lean-ctx cache stats  # Show cache status for current dir
lean-ctx cache clear  # Clear cache for current dir
lean-ctx doctor       # Run diagnostics (12 checks)
lean-ctx read <file>  # Read file with compression
```

### In OpenCode / AI Conversations

lean-ctx exposes MCP tools for context management. The AI can use them to read compressed context, manage cache, or analyze directory structure. Tool names are prefixed by the MCP server name (e.g., `lean-ctx/read`, `lean-ctx/cache_clear`).

### Shell Hook Activation

The hook was injected into your PowerShell profile. If not active:
```powershell
. $PROFILE
```

Or restart your terminal.

### When to Update

lean-ctx **auto-updates** on every `cd` — no manual action needed. But if you make significant changes within a session and want to force a refresh, ask the AI to re-read the directory context using lean-ctx MCP tools, or run:
```bash
lean-ctx cache clear && cd . && cd ..
```
<!-- lean-ctx:end -->

# GitNexus — Code Intelligence

This project is indexed by GitNexus as **skills-manager-system**. Use the GitNexus MCP tools to understand code, assess impact, and navigate safely.

> If any GitNexus tool warns the index is stale, run `npm run update:gitnexus` first.

## Always Do

- **MUST run impact analysis before editing any symbol.** Before modifying a function, class, or method, run `gitnexus_impact({target: "symbolName", direction: "upstream"})` and report the blast radius (direct callers, affected processes, risk level) to the user.
- **MUST run `gitnexus_detect_changes()` before committing** to verify your changes only affect expected symbols and execution flows.
- **MUST warn the user** if impact analysis returns HIGH or CRITICAL risk before proceeding with edits.
- When exploring unfamiliar code, use `gitnexus_query({query: "concept"})` to find execution flows instead of grepping. It returns process-grouped results ranked by relevance.
- When you need full context on a specific symbol — callers, callees, which execution flows it participates in — use `gitnexus_context({name: "symbolName"})`.

## Never Do

- NEVER edit a function, class, or method without first running `gitnexus_impact` on it.
- NEVER ignore HIGH or CRITICAL risk warnings from impact analysis.
- NEVER rename symbols with find-and-replace — use `gitnexus_rename` which understands the call graph.
- NEVER commit changes without running `gitnexus_detect_changes()` to check affected scope.

## Resources

| Resource | Use for |
|----------|---------|
| `gitnexus://repo/skills-manager-system/context` | Codebase overview, check index freshness |
| `gitnexus://repo/skills-manager-system/clusters` | All functional areas |
| `gitnexus://repo/skills-manager-system/processes` | All execution flows |
| `gitnexus://repo/skills-manager-system/process/{name}` | Step-by-step execution trace |

## CLI

| Task | Read this skill file |
|------|---------------------|
| Understand architecture / "How does X work?" | `.claude/skills/gitnexus/gitnexus-exploring/SKILL.md` |
| Blast radius / "What breaks if I change X?" | `.claude/skills/gitnexus/gitnexus-impact-analysis/SKILL.md` |
| Trace bugs / "Why is X failing?" | `.claude/skills/gitnexus/gitnexus-debugging/SKILL.md` |
| Rename / extract / split / refactor | `.claude/skills/gitnexus/gitnexus-refactoring/SKILL.md` |
| Tools, resources, schema reference | `.claude/skills/gitnexus/gitnexus-guide/SKILL.md` |
| Index, status, clean, wiki CLI commands | `.claude/skills/gitnexus/gitnexus-cli/SKILL.md` |

<!-- gitnexus:end -->
