# 晓林一键开关DOH

一个使用Rust和egui开发的跨平台桌面应用程序，用于快速开关谷歌浏览器和Edge浏览器的DOH（DNS over HTTPS）功能。

## 功能特点

- 🖥️ **跨平台支持**：支持Windows和macOS操作系统
- 🎯 **多架构支持**：
  - Windows：32位（i686）和64位（x86_64）
  - macOS：Intel（x86_64）和Apple Silicon（aarch64）
- 📊 **实时状态显示**：自动检测并显示浏览器DOH状态
- 🎮 **灵活控制**：
  - 单独控制每个浏览器的DOH开关
  - 一键全开/全关所有浏览器的DOH
- 🚀 **高性能**：使用Rust开发，响应迅速，资源占用低
- 🎨 **简洁界面**：基于egui的现代化用户界面

## 系统要求

### Windows
- Windows 7或更高版本
- 需要管理员权限（修改注册表）

### macOS
- macOS 10.13或更高版本
- 需要管理员权限（修改系统配置）

## 安装方法

### 从GitHub Actions下载构建产物

1. 访问项目的[Actions页面](https://github.com/zzxym/DoH-Toggle/actions)
2. 选择最新的工作流运行
3. 在Artifacts部分下载适合您系统的版本：
   - `doh-toggle-windows-x86_64`：Windows 64位
   - `doh-toggle-windows-i686`：Windows 32位
   - `doh-toggle-macos-x86_64`：macOS Intel
   - `doh-toggle-macos-aarch64`：macOS Apple Silicon

### 从源代码编译

```bash
# 克隆仓库
git clone https://github.com/zzxym/DoH-Toggle.git
cd DoH-Toggle

# 编译发布版本
cargo build --release

# 运行程序
# Windows
./target/release/doh_toggle.exe

# macOS
./target/release/doh_toggle
```

## 使用说明

1. **启动程序**：双击运行可执行文件（需要管理员权限）
2. **查看状态**：程序启动后会自动显示当前浏览器的DOH状态
3. **单独控制**：
   - 点击"开启"或"关闭"按钮来切换单个浏览器的DOH状态
4. **批量操作**：
   - 点击"一键全开"按钮开启所有浏览器的DOH
   - 点击"一键全关"按钮关闭所有浏览器的DOH
5. **重启浏览器**：操作完成后请重启浏览器以使更改生效

## 技术架构

- **编程语言**：Rust
- **UI框架**：egui + eframe
- **配置管理**：
  - Windows：通过注册表（HKLM\SOFTWARE\Policies）
  - macOS：通过defaults命令（com.google.Chrome、com.microsoft.Edge）

## 构建说明

### 本地构建

```bash
# 安装依赖
cargo build

# 编译特定目标
cargo build --release --target x86_64-pc-windows-msvc
cargo build --release --target i686-pc-windows-msvc
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
```

### GitHub Actions构建

项目配置了GitHub Actions工作流，支持：
- 自动触发：代码推送到main分支或创建Pull Request时
- 手动触发：在GitHub Actions界面手动选择平台和选项

## 许可证

本项目采用MIT许可证。

## 贡献

欢迎提交Issue和Pull Request！

## 联系方式

- GitHub：https://github.com/zzxym/DoH-Toggle
- 作者：晓林

## 注意事项

- 修改系统配置需要管理员权限
- 操作完成后需要重启浏览器才能生效
- 关闭DOH可能会影响网络安全性，请根据实际情况使用