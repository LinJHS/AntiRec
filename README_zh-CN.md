<div align="center">
  
  <!-- # 🎧 AntiRec  -->
  
  <img src="https://github.com/LinJHS/AntiRec/blob/main/images/logo.png" alt="AntiRec Logo" width="500">

  [![GitHub Stars](https://img.shields.io/github/stars/LinJHS/AntiRec?style=flat-square)](https://github.com/LinJHS/AntiRec/stargazers)
  [![GitHub Forks](https://img.shields.io/github/forks/LinJHS/AntiRec?style=flat-square)](https://github.com/LinJHS/AntiRec/network)
  [![GitHub Issues](https://img.shields.io/github/issues/LinJHS/AntiRec?style=flat-square)](https://github.com/LinJHS/AntiRec/issues)
  [![GitHub Contributors](https://img.shields.io/github/contributors/LinJHS/AntiRec?style=flat-square)](https://github.com/LinJHS/AntiRec/graphs/contributors)
  [![GitHub All Releases](https://img.shields.io/github/downloads/LinJHS/AntiRec/total?style=flat-square)](https://github.com/LinJHS/AntiRec/releases)
  [![GitHub Release (latest SemVer)](https://img.shields.io/github/v/release/LinJHS/AntiRec?style=flat-square)](https://github.com/LinJHS/AntiRec/releases)
  [![GitHub License](https://img.shields.io/github/license/LinJHS/AntiRec?style=flat-square)](https://github.com/LinJHS/AntiRec/blob/main/LICENSE)

  
  [English](https://github.com/LinJHS/AntiRec/blob/main/README.md) | [简体中文](https://github.com/LinJHS/AntiRec/blob/main/README_zh-CN.md)
</div>

**AntiRec** 是一款跨平台客户端应用程序，设计用于实时音频捕获、处理和播放。基于现代的 [Tauri](https://tauri.app/) 框架，它在多个操作系统上提供了无缝体验，并具有高效的音频处理能力。🚀

利用其强大的音频处理能力，AntiRec 可以通过向麦克风输入的音频中引入可控的扰动来提供先进的隐私保护。此功能旨在不影响用户的正常通信，阻止自动语音识别（ASR）系统的识别，从而防止潜在的窃听行为，确保私人通信保持机密性。通过这些功能，AntiRec 致力于为用户提供一个安全的实时音频交互环境，同时保持所捕获音频的完整性和可用性。

## ✨ 功能

- **实时音频捕获**：从任何输入设备无缝捕获音频。
- **跨平台兼容性**：在 Windows、macOS 和 Linux 上流畅运行。
- **低资源消耗**：使用 Tauri 进行优化，实现轻量级性能。
- **高效处理**：快速的实时音频处理管道。
- **自定义播放选项**：针对不同使用场景的灵活播放配置。

## 🚀 开始使用

按照以下步骤在您的系统上安装并运行该应用程序。

### 前提条件
- [Node.js](https://nodejs.org/) v14 或更高版本
- [Tauri](https://tauri.app/)（按照官方指南安装）
- [pnpm](https://pnpm.io/)（一个快速且节省磁盘空间的包管理器）
- 合法的音频输入设备（比如：麦克风）
- 虚拟声卡驱动程序（比如 "Virtual Audio Cable" 或 "[VB-Audio Virtual Cable](https://vb-audio.com/Cable/)"）

### 安装
1. 克隆仓库：
   ```bash
   git clone https://github.com/LinJHS/AntiRec.git
   cd AntiRec
   ```

2. 安装依赖项：
   ```bash
   pnpm install
   ```

3. 构建并运行应用程序：
   ```bash
   pnpm tauri dev
   ```

<!-- 
## 📷 屏幕截图

[AntiRec 屏幕截图 1](https://your-image-link.com/screenshot1.png)
*Windows 平台上的 AntiRec 主界面*

[AntiRec 屏幕截图 2](https://your-image-link.com/screenshot2.png)
*实时音频处理视图* -->

## 🛠️ 技术栈

AntiRec 利用了强大而现代的技术堆栈：

- **[Tauri](https://tauri.app/)** — 跨平台应用程序开发框架。
- **[Vue.js](https://vuejs.org/)** — 用于用户界面的前端框架。
- **[Rust](https://www.rust-lang.org/)** — 适用于性能关键任务的强大系统编程语言。
- **JavaScript** — 实现与 UI 组件的无缝交互。

## 🤝 贡献

我们欢迎贡献！以下是您如何参与的方法：

1. fork 该仓库。
2. 创建您的 feature 分支 (`git checkout -b feature-my-feature`)。
3. 提交更改 (`git commit -m 'Add some feature'`)。
4. 推送到分支 (`git push origin feature-my-feature`)。
5. 提交 pull request。

## 📜 许可证

本项目采用 GPL-3.0 许可证 —— 详情请参阅 [LICENSE](https://github.com/LinJHS/AntiRec/blob/main/LICENSE) 文件。

## 🌟 支持我们

如果您喜欢这个项目，请在 [GitHub](https://github.com/LinJHS/AntiRec) 上给它点个星🌟！

## 📫 联系方式

有任何疑问或想要联系我们？请随时联系我们：

- GitHub 问题：[创建新 Issue](https://github.com/LinJHS/AntiRec/issues)
- 邮件：LinJHS@bupt.edu.cn

## ✨ 贡献者

感谢以下的贡献者们！

[![Contributors](https://contrib.rocks/image?repo=LinJHS/AntiRec)](https://github.com/LinJHS/AntiRec/graphs/contributors)
