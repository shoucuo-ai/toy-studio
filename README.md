# Tauri + Sycamore

This template should help get you started developing with Tauri and Sycamore.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Features

### PDF Document Translation

### Image Watermark Removal

## Develop

### start

```bash
cargo tauri dev
```

## 产品特性

### PDF 文档翻译

https://github.com/Byaidu/PDFMathTranslate

### 图片去水印

https://github.com/zuruoke/watermark-removal

### uv 管理

- 基础配置
  - 缓存目录：
    - 指定：`--cache-dir`、`UV_CACHE_DIR`、`tool.uv.cache-dir.`
    - Unix: `$XDG_CACHE_HOME/uv`、`$HOME/.cache/uv`
    - Windows：`%LOCALAPPDATA%\uv\cache`
