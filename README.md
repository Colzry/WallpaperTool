# WallpaperTool

## 概述
`WallpaperTool` 是一个用于 Windows 的壁纸设置工具，支持：

- 设置单张壁纸
- 在文件夹中轮换壁纸（支持顺序或随机模式）
- 自定义轮换时间间隔

## 安装
### 依赖
本程序基于 Rust 语言开发，需安装 Rust 运行环境：
1. 下载并安装 [Rust](https://www.rust-lang.org/)
2. 安装依赖：
```sh
cargo install clap rand widestring winapi
```

### 编译
```sh
cargo build --release
```
可执行文件位于 `target/release/WallpaperTool.exe`。

## 使用方法

### 设置单张壁纸
```sh
WallpaperTool.exe "C:\path\to\image.jpg"
```
> **注意**: 仅支持 `jpg`, `jpeg`, `png`, `bmp` 格式。

### 启用壁纸轮换
```sh
WallpaperTool.exe "C:\path\to\wallpapers" --rotate
```
- `--rotate`：启用轮换模式，默认间隔 `30` 分钟，按顺序轮换

#### 选项
- **设置轮换间隔（分钟）**
  ```sh
  WallpaperTool.exe "C:\path\to\wallpapers" --rotate --interval 10
  ```
  每 `10` 分钟更换一次壁纸。

- **使用随机模式**
  ```sh
  WallpaperTool.exe "C:\path\to\wallpapers" --rotate --mode random
  ```
  以随机顺序更换壁纸。

## 示例
- **设置单张壁纸**
  ```sh
   WallpaperTool.exe "C:\Users\Public\Pictures\image.jpg"
  ```
- **每 15 分钟顺序轮换壁纸**
  ```sh
  WallpaperTool.exe "C:\Users\Public\Pictures\Wallpapers" --rotate --interval 15
  ```
- **每 5 分钟随机轮换**
  ```sh
  WallpaperTool.exe "C:\Users\Public\Pictures\Wallpapers" --rotate --interval 5 --mode random
  ```

## 兼容性
仅支持 Windows。

## 许可证
MIT License

Copyright ©️2025 Colzry