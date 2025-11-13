# Owl MCP Server

Owl MCP Server 是一个基于 Rust 开发的 Model Context Protocol (MCP) 服务器集合，提供多种功能模块，包括网页搜索、HTTP API 调用和图像问答等服务。

## 项目概述

本项目实现了多个 MCP 服务器模块，每个模块都可通过标准 MCP 协议与 AI 应用（如 Claude、ChatGPT 等）进行交互，为 AI 提供各种工具和数据源。

### 核心模块

1. **web-search** - 网页搜索和内容提取服务
   - 支持关键词搜索并返回相关网页信息
   - 支持从网页中提取正文内容并转换为 Markdown 格式
   - 提供静态和动态两种网页抓取模式

2. **http-api-call** - HTTP API 调用服务
   - 允许通过 MCP 调用任意 HTTP API
   - 支持动态注册和管理 API 端点
   - 支持 URL 参数、请求体和请求头的配置

3. **image-qa-online** - 在线图像问答服务
   - 支持分析图片内容并回答相关问题
   - 支持 OCR 文字提取功能
   - 支持本地图片路径和网络图片链接

## 技术栈

- **语言**: Rust
- **核心依赖**:
  - `rmcp` - Rust MCP 协议实现
  - `tokio` - 异步运行时
  - `serde` - 序列化/反序列化
  - `reqwest` - HTTP 客户端
- **其他依赖**:
  - `scraper` - HTML 解析
  - `headless_chrome` - 动态网页抓取
  - `clap` - 命令行参数解析

## 安装与构建

### 环境要求

- Rust 1.70 或更高版本
- Cargo 构建工具

### 构建项目

```bash
# 克隆项目
git clone <repository-url>
cd owl-mcp-server

# 构建所有模块
cargo build --release
```

### 构建单个模块

```bash
# 构建 web-search 模块
cargo build -p web-search --release

# 构建 http-api-call 模块
cargo build -p http-api-call --release

# 构建 image-qa-online 模块
cargo build -p image-qa-online --release
```

## 使用方法

### web-search 模块

```bash
# 以 MCP 服务模式启动
./target/release/web-search mcp

# 执行网页搜索
./target/release/web-search search -q "人工智能发展" -c 10

# 提取网页内容
./target/release/web-search extract -u "https://example.com"

# 以 HTTP 服务启动
./target/release/web-search http -p 10020
```

### http-api-call 模块

```bash
# 以 MCP 服务模式启动
./target/release/http-api-call
```

### image-qa-online 模块

```bash
# 以 MCP 服务模式启动
./target/release/image-qa-online
```