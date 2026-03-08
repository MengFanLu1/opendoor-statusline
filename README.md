# OpenDoor StatusLine

基于 Rust 的高性能 Claude Code 状态栏工具，支持余额/用量追踪、Git 信息展示、交互式 TUI 配置和多主题系统。

> 本项目基于 [CCometixLine](https://github.com/cometix-ai/ccline) 和 [ByeByeCode](https://github.com/byebye-code/byebyecode) 的优秀工作进行开发，感谢原作者们的辛勤付出。

![Language:Rust](https://img.shields.io/static/v1?label=Language&message=Rust&color=orange&style=flat-square)
![License:MIT](https://img.shields.io/static/v1?label=License&message=MIT&color=blue&style=flat-square)

## 截图

![opendoor-statusline](assets/img1.png)

状态栏显示：模型 | 目录 | Git 分支状态 | 上下文窗口 | 用量信息

## 安装

### npm 安装

```bash
npm install -g @code-opendoor-ai/statusline
```

国内加速：

```bash
npm install -g @code-opendoor-ai/statusline --registry=https://registry.npmmirror.com
```

### 手动安装

1. 访问 [Releases 页面](https://github.com/opendoor-ai/opendoor-statusline/releases) 下载对应平台的压缩包
2. 解压获取 `opendoor-statusline` 可执行文件
3. 放置到以下目录：
   - macOS/Linux: `~/.claude/opendoor-statusline/opendoor-statusline`
   - Windows: `%USERPROFILE%\.claude\opendoor-statusline\opendoor-statusline.exe`
4. 赋予执行权限 (macOS/Linux):
   ```bash
   chmod +x ~/.claude/opendoor-statusline/opendoor-statusline
   ```

### 初始化

```bash
opendoor-statusline --init
```

启动 Claude Code 即可看到状态栏。

## 特性

### 核心功能

- **Git 集成** - 显示分支、状态和跟踪信息
- **模型显示** - 简化的 Claude 模型名称（如 `claude-4-sonnet` -> `Sonnet 4`）
- **用量追踪** - 实时显示中转站套餐余额和用量
- **目录显示** - 显示当前工作空间
- **上下文窗口** - 令牌使用百分比和上下文限制跟踪
- **简洁设计** - 使用 Nerd Font 图标

### 中转站用量监控

- **自动检测 API Key** - 从 `~/.claude/settings.json` 自动读取
- **余额进度条** - 可视化显示套餐用量（如 `$13.86/$50 ▓▓▓░░░░░░░`）
- **多套餐支持** - 支持 PLUS/PRO/MAX/PAYGO 等套餐类型
- **智能 Fallback** - Usage API 失效时自动切换到 Subscription API

### 交互式 TUI

- **TUI 配置界面** - 实时预览配置效果
- **主题系统** - 多种内置主题（cometix, minimal, gruvbox, nord, powerline-dark 等）
- **段落自定义** - 精细化控制各段落的启用/禁用、颜色、图标

### Claude Code 增强

- **禁用上下文警告** - 移除 "Context low" 提示
- **启用详细模式** - 增强输出详细信息
- **自动备份** - 安全修改，支持轻松恢复

## 使用

### 配置管理

```bash
# 初始化配置文件
opendoor-statusline --init

# 检查配置有效性
opendoor-statusline --check

# 打印当前配置
opendoor-statusline --print

# 进入 TUI 配置模式
opendoor-statusline --config

# 通过 opendoor-statusline 启动 Claude Code
opendoor-statusline --wrap
```

### 主题切换

```bash
# 临时使用指定主题
opendoor-statusline --theme cometix
opendoor-statusline --theme minimal
opendoor-statusline --theme gruvbox
opendoor-statusline --theme nord
opendoor-statusline --theme powerline-dark

# 使用自定义主题
opendoor-statusline --theme my-custom-theme
```

### Claude Code 补丁

```bash
opendoor-statusline --patch /path/to/claude-code/cli.js
```

## 配置

配置文件路径：`~/.claude/opendoor-statusline/config.toml`

通过 `opendoor-statusline --config` 可以进入交互式 TUI 编辑配置并实时预览效果。

### 可用段落

| 段落 | 说明 |
|------|------|
| `model` | 当前使用的 AI 模型 |
| `directory` | 当前工作目录 |
| `git` | Git 分支和状态 |
| `context_window` | 上下文窗口使用情况 |
| `usage` | API 用量（原生） |
| `cost` | 会话费用 |
| `session` | 会话信息 |
| `output_style` | 当前输出样式 |
| `opendoor_usage` | 中转站套餐用量（带进度条） |

所有段落都支持：启用/禁用、自定义分隔符和图标、颜色自定义、格式选项。

### Git 状态指示器

- 带 Nerd Font 图标的分支名
- 状态：`✓` 清洁，`●` 有更改，`⚠` 冲突
- 远程跟踪：`↑n` 领先，`↓n` 落后

## 系统要求

- **Git**: 1.5+（推荐 2.22+ 以获得更好的分支检测）
- **终端**: 需支持 Nerd Font 图标
  - 安装 [Nerd Font](https://www.nerdfonts.com/) 字体
  - 中文用户推荐: [Maple Font](https://github.com/subframe7536/maple-font)
- **Claude Code**: 用于状态栏集成

## 开发

```bash
# 构建开发版本
cargo build

# 运行测试
cargo test

# 构建优化版本
cargo build --release
```

## 常见问题

### Binary not found

安装后运行报 `Binary not found`，通常是 npm 路径解析或网络下载失败。

```bash
# 强制重新安装
npm install -g @code-opendoor-ai/statusline --force
```

如果仍然失败，参考上方手动安装步骤。

### macOS 编译错误 (ring crate)

从源码编译遇到 `ring` 相关错误，安装 Xcode Command Line Tools：

```bash
xcode-select --install
```

## 致谢

- [CCometixLine](https://github.com/cometix-ai/ccline) - 核心架构和状态栏生成的基础
- [ByeByeCode](https://github.com/byebye-code/byebyecode) - API 集成和增强功能的参考
- [tweakcc](https://github.com/Piebald-AI/tweakcc) - 自定义 Claude Code 主题的命令行工具

## 许可证

[MIT](LICENSE)
