# ToyStudio

当前项目基于 Tauri 框架，使用 Web 框架 Sycamore 和 CSS 框架 tailwind css。

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).


## Develop

### 前置安装

- Git
- uv

### start

```bash
cargo install wasm-bindgen-cli
cargo tauri dev
```

## 产品特性



### 图片去水印

https://github.com/zuruoke/watermark-removal

### uv 管理

- 基础配置
  - 缓存目录：
    - 指定：`--cache-dir`、`UV_CACHE_DIR`、`tool.uv.cache-dir.`
    - Unix: `$XDG_CACHE_HOME/uv`、`$HOME/.cache/uv`
    - Windows：`%LOCALAPPDATA%\uv\cache`

## TODO 确认


### 周期扫描已安装应用




