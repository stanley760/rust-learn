# defined-agent

> 一个用 Rust 编写的轻量级 AI Agent 框架，基于 OpenAI 兼容 API 实现 Tool-Use（工具调用）循环。

## 项目概述

`defined-agent` 是 [rust-learn](../) 工作空间中的一个子项目，演示了如何从零构建一个具备 **工具调用能力** 的 AI Agent。它实现了完整的 Agent Loop 模式：接收用户指令 → LLM 推理 → 工具执行 → 结果反馈 → 继续推理，直到任务完成。

核心设计理念：

- **Provider-Agnostic**：通过 `ToolSpec` 抽象层，工具定义与具体 LLM 提供商解耦，可轻松适配 OpenAI / Anthropic / 其他兼容 API
- **安全优先**：内置命令黑名单、路径穿越防护、超时机制，确保 Agent 操作受控
- **跨平台**：自动检测操作系统，在 Windows 上使用 `cmd.exe`，在 Unix 上使用 `sh`
- **子 Agent 委派**：内置 `task` 工具，主 Agent 可生成独立上下文的子 Agent 来处理子任务，实现任务分解与并行

## 架构

```
defined-agent/
├── src/
│   ├── lib.rs                  # 模块导出（llm, structure, tools）+ extract_text 工具函数
│   ├── main.rs                 # 基础示例：流式对话 + 重试
│   ├── llm/                    # LLM 交互层
│   │   ├── mod.rs              # 导出 complete / agent_loop
│   │   ├── complete.rs         # 流式聊天补全 + 指数退避重试
│   │   └── agent_loop.rs       # 早期版 Agent Loop（仅 bash 工具）
│   ├── structure/              # Agent 状态与结构化输出
│   │   ├── mod.rs              # 导出 LoopState / get_llm_client / ActionPlan
│   │   ├── loop_state.rs       # 核心循环状态机 + 工具调度
│   │   └── act_plan.rs         # 结构化行动计划（ActionPlan / ActionStep）
│   └── tools/                  # 工具系统
│       ├── mod.rs              # Tool trait / ToolSpec / 注册表 / safe_path
│       ├── bash.rs             # Shell 命令执行工具
│       ├── read_file.rs        # 文件读取工具
│       ├── write_file.rs       # 文件写入工具
│       ├── edit_file.rs        # 文件精确替换工具
│       ├── sub_agent.rs        # 子 Agent 工具（task）
│       └── todo.rs             # （已弃用）会话计划管理工具
├── examples/
│   └── agent_loop_optimized.rs # 完整交互式 Agent 示例
├── .env                        # 环境变量配置
└── Cargo.toml
```

## 核心模块详解

### 1. 工具系统 (`tools/`)

工具系统是 Agent 的"手脚"，定义了 Agent 可以执行的所有操作。

#### Tool Trait

所有工具实现统一的 `Tool` trait：

```rust
#[async_trait]
pub trait Tool: Send + Sync {
    async fn invoke(&mut self, input: &Value) -> anyhow::Result<String>;
    fn name(&self) -> Cow<'_, str>;
    fn tool_spec(&self) -> ToolSpec;
}
```

- `invoke`：执行工具逻辑，接收 JSON 输入，返回字符串结果
- `name`：返回工具唯一标识符
- `tool_spec`：返回工具的 JSON Schema 描述，用于告知 LLM 该工具的用法

#### ToolSpec 抽象

`ToolSpec` 是提供商无关的工具规格描述，可转换为 OpenAI 兼容的 `ChatCompletionTools`：

```rust
pub struct ToolSpec {
    pub name: String,
    pub description: Option<String>,
    pub input_schema: serde_json::Value,  // JSON Schema
}
```

#### 内置工具

| 工具 | 名称 | 功能 | 安全特性 |
|------|------|------|----------|
| **BashTool** | `bash` | 执行 Shell 命令 | 危险命令黑名单、120s 超时、输出截断 50000 字符 |
| **ReadFileTool** | `read_file` | 读取文件内容 | `safe_path` 路径穿越防护、支持 `limit` 参数 |
| **WriteFileTool** | `write_file` | 写入文件 | 自动创建父目录、`safe_path` 防护 |
| **EditFileTool** | `edit_file` | 精确文本替换 | 原子替换（`replacen` 1次）、文本必须存在才替换 |
| **SubAgentTool** | `task` | 生成子 Agent 执行独立任务 | 子 Agent 拥有独立上下文，共享文件系统 |

#### 工具注册

主 Agent 和子 Agent 使用不同的工具集：

```rust
// 主 Agent 工具集（包含 task 子 Agent 工具）
pub fn agent_tools() -> HashMap<String, Box<dyn Tool>> {
    HashMap::from([
        ("bash".to_string(), bash_tool()),
        ("read_file".to_string(), read_file_tool()),
        ("write_file".to_string(), write_file_tool()),
        ("edit_file".to_string(), edit_file_tool()),
        ("task".to_string(), sub_agent_tool()),
    ])
}

// 子 Agent 工具集（不含 task，防止无限嵌套）
pub fn subagent_tools() -> HashMap<String, Box<dyn Tool>> {
    HashMap::from([
        ("bash".to_string(), bash_tool()),
        ("read_file".to_string(), read_file_tool()),
        ("write_file".to_string(), write_file_tool()),
        ("edit_file".to_string(), edit_file_tool()),
    ])
}
```

#### 路径安全 (`safe_path`)

所有文件操作工具都通过 `safe_path` 函数验证路径，防止路径穿越攻击：

- **读取操作**（`must_exist: true`）：对目标路径做 `canonicalize`，确保解析后的绝对路径在当前工作目录下
- **写入操作**（`must_exist: false`）：对父目录做 `canonicalize`，再拼接文件名——允许创建新文件，同时阻止 `../../etc/passwd` 类攻击

### 2. Agent Loop (`structure/loop_state.rs`)

这是 Agent 的核心循环——"大脑"与"手脚"的协调器。

#### LoopState 状态机

```rust
pub struct LoopState {
    client: Client<OpenAIConfig>,           // OpenAI API 客户端
    pub context: Vec<ChatCompletionRequestMessage>,  // 对话上下文
    tools: HashMap<String, Box<dyn Tool>>,  // 工具注册表
    pub system_prompt: String,              // 系统提示词
    pub max_round: usize,                   // 最大循环轮数
}
```

#### 循环流程

```
用户输入 → [System Prompt] → LLM 推理
                                    ↓
                            ┌── finish_reason?
                            │
                    ToolCalls ──→ 执行工具调用 → 结果推入上下文 → 回到 LLM 推理
                            │
                    Stop/其他 ──→ 返回最终回复
```

1. 将 System Prompt 推入上下文（自动适配 Windows/Unix 指令）
2. 构建请求（包含对话历史 + 工具定义），发送给 LLM
3. LLM 返回响应：
   - `finish_reason = ToolCalls`：解析工具调用，执行后将结果推入上下文，回到步骤 2
   - `finish_reason = Stop` 或其他：循环结束，返回最终回复

### 3. 子 Agent 系统 (`tools/sub_agent.rs`)

子 Agent 是实现任务分解的核心机制。主 Agent 可以通过 `task` 工具生成一个拥有 **独立上下文** 的子 Agent，将复杂任务委派给它执行。

#### 工作原理

```
主 Agent Loop
    │
    ├── LLM 决定调用 task 工具
    │       ↓
    ├── SubAgentTool.invoke()
    │       ↓
    ├── sub_agent_loop(prompt, description)
    │       │
    │       ├── 创建新的 LoopState（独立上下文，30 轮上限）
    │       ├── 使用 subagent_tools()（不含 task，防止嵌套）
    │       ├── 注入 System Prompt + 用户任务描述
    │       ├── 运行独立的 agent_loop()
    │       └── 提取最后一条 Assistant 消息作为摘要
    │       ↓
    └── 摘要返回给主 Agent，作为 task 工具的输出
```

#### 关键设计

- **上下文隔离**：子 Agent 不共享主 Agent 的对话历史，从零开始处理任务
- **文件系统共享**：子 Agent 可以读写同一工作目录的文件
- **防止无限嵌套**：`subagent_tools()` 不包含 `task` 工具，子 Agent 不能再生成子 Agent
- **自动摘要**：子 Agent 完成后，自动提取最后一条 Assistant 消息作为结果摘要返回给主 Agent

#### 触发方式

在交互式 Agent 中，使用以下类型的 prompt 可以触发子 Agent：

```
--- How can I help you?> Use a sub-agent task to list all Rust files in src/tools/ and summarize each one.
--- How can I help you?> Spawn a task to investigate the project structure and suggest improvements.
--- How can I help you?> Delegate a sub-agent to find all TODO comments in the codebase.
```

### 4. LLM 交互层 (`llm/`)

#### 流式补全 (`complete.rs`)

- `chat_complete_structed`：返回 `impl Stream<Item = Result<String>>`，逐 token 输出
- `chat_stream_with_retry`：在流式补全基础上增加指数退避重试（最多 3 次）
- 内置 TTFT（首字延迟）测量，打印到控制台

#### 早期 Agent Loop (`agent_loop.rs`)

这是早期版本的 Agent Loop，仅支持 `bash` 工具，使用 `join_all` 并行执行工具调用。已被 `loop_state.rs` 中的通用版本取代，保留作为参考。

### 5. 结构化输出 (`structure/act_plan.rs`)

定义了 `ActionPlan` 数据结构，用于 LLM 生成结构化的行动计划：

```rust
pub struct ActionPlan {
    pub goal: String,
    pub steps: Vec<ActionStep>,
    pub difficulty: Difficulty,    // Easy | Middle | Hard
    pub estimated_minutes: u32,
}

pub struct ActionStep {
    pub index: i32,
    pub description: String,
    pub tool_hint: Option<String>,  // 建议使用的工具
}
```

通过 `schemars` crate 自动生成 JSON Schema，可用于 LLM 的 Structured Output / JSON Mode。

## 快速开始

### 环境配置

在 `defined-agent/.env` 中设置以下环境变量：

```env
OPENAI_BASE_URL=https://openrouter.ai/api/v1   # 或其他 OpenAI 兼容 API 地址
OPENAI_MODEL=openai/gpt-oss-120b:free           # 模型名称
OPENAI_API_KEY=sk-your-api-key                  # API 密钥
```

### 运行基础示例

基础示例演示流式对话 + 指数退避重试：

```bash
cargo run -p defined-agent
```

### 运行交互式 Agent

完整的 Agent Loop 交互体验，支持多轮对话、工具调用和子 Agent 委派：

```bash
cargo run -p defined-agent --example agent_loop_optimized
```

交互界面：
```
--- How can I help you?> 帮我查看当前目录的文件
（Agent 自动调用 bash 工具执行 dir 命令，返回结果）

--- How can I help you?> Use a task to analyze all .rs files in src/tools/
（Agent 调用 task 工具，生成子 Agent 独立分析，返回摘要）

--- How can I help you?> exit()
（退出程序）
```

## 依赖说明

| 依赖 | 版本 | 用途 |
|------|------|------|
| `async-openai` | 0.41.1 | OpenAI 兼容 API 客户端（支持流式 + 工具调用） |
| `tokio` | 1.52.3 | 异步运行时 |
| `serde` / `serde_json` | 1.0 | 序列化/反序列化 |
| `schemars` | 1.2.1 | 从 Rust 类型自动生成 JSON Schema |
| `anyhow` | 1.0 | 错误处理 |
| `tracing` | 0.1 | 结构化日志 |
| `backon` | 1.6.0 | 指数退避重试策略 |
| `async-trait` | 0.1 | 异步 trait 支持 |
| `dialoguer` | 0.12 | 交互式命令行输入 |
| `strum` | 0.28 | 枚举派生宏（用于 PlanItemStatus 的 marker 属性） |
| `futures` | 0.3 | Stream / Future 工具 |
| `async-stream` | 0.3 | 使用 `stream!` 宏创建异步流 |
| `bytes` | 1.11 | 字节处理 |

## 设计亮点

### 1. 双层 Agent Loop 演进

项目保留了两个版本的 Agent Loop 实现，展示了从简单到复杂的演进路径：

- **V1**（`llm/agent_loop.rs`）：硬编码 bash 工具，`join_all` 并行执行
- **V2**（`structure/loop_state.rs`）：通用工具注册表，顺序执行，支持子 Agent 委派

### 2. 子 Agent 委派机制

通过 `task` 工具实现主 Agent → 子 Agent 的任务委派：

- **上下文隔离**：子 Agent 拥有独立对话历史，不会污染主 Agent 上下文
- **工具集隔离**：子 Agent 使用 `subagent_tools()`，不含 `task` 工具，防止无限嵌套
- **自动摘要**：子 Agent 完成后自动提取结果摘要，精简返回给主 Agent
- **灵活触发**：LLM 根据任务复杂度自主决定是否委派子 Agent

### 3. 安全沙箱

- **命令黑名单**：拦截 `rm -rf /`、`sudo`、`shutdown`、`format` 等危险命令
- **路径穿越防护**：`safe_path` 确保所有文件操作限制在工作目录内
- **超时保护**：命令执行 120 秒超时，`kill_on_drop` 确保子进程不会泄漏
- **输出截断**：命令输出和文件内容限制在 50000 字符内，防止上下文溢出

### 4. 跨平台适配

- 自动检测 OS，选择 `cmd.exe`（Windows）或 `sh`（Unix）
- Windows 下自动执行 `chcp 65001` 切换 UTF-8 编码，解决中文乱码
- System Prompt 根据平台自动调整（Windows 提示使用 `dir`/`type`/`findstr`）

### 5. 可扩展的工具系统

添加新工具只需三步：

1. 在 `src/tools/` 下创建新文件，实现 `Tool` trait
2. 在 `src/tools/mod.rs` 中注册模块
3. 在 `agent_tools()` 和/或 `subagent_tools()` 中添加实例

```rust
// 示例：添加一个 HTTP 请求工具
pub struct HttpTool;

#[async_trait]
impl Tool for HttpTool {
    async fn invoke(&mut self, input: &Value) -> anyhow::Result<String> {
        let url = input["url"].as_str().ok_or_else(|| anyhow::anyhow!("missing url"))?;
        // ... 发送 HTTP 请求并返回结果
        Ok(response_text)
    }

    fn name(&self) -> Cow<'_, str> { "http".into() }

    fn tool_spec(&self) -> ToolSpec {
        ToolSpec {
            name: "http".into(),
            description: Some("Send HTTP request".into()),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": { "url": { "type": "string" } },
                "required": ["url"]
            }),
        }
    }
}
```

## 在 rust-learn 工作空间中的位置

`defined-agent` 是 [rust-learn](../) 工作空间的成员之一，与其他子项目共同构成 Rust 学习实践集合：

| 子项目 | 领域 |
|--------|------|
| **defined-agent** | AI Agent / LLM Tool-Use |
| algorithm | 算法与数据结构 |
| simple-redis | 网络编程 / Redis 协议 |
| kv_server | 键值存储 / 分布式 |
| httpie | HTTP 客户端 |
| thumbor | 图像处理 / Web 服务 |
| rcli | 命令行工具开发 |
| grep | 文本搜索 |
| concurrency | 并发编程 |
| crypto | 加密 |
| bert_similarity | NLP / 相似度 |
| halo_web | Web 开发 |
| todolist | 全栈应用 |
| process_killer | 系统工具 |
| getinfo | 系统信息 |
| exercise | 练习 |
| macro_new | 宏编程 |

## 学习要点

通过这个项目，你可以学到：

1. **Agent Loop 模式**：如何实现 LLM 驱动的自主循环——推理 → 行动 → 观察 → 再推理
2. **Tool-Use / Function Calling**：如何定义工具 Schema、注册工具、解析 LLM 的工具调用请求并执行
3. **子 Agent 委派**：如何实现主 Agent → 子 Agent 的任务分解，上下文隔离与结果汇总
4. **流式处理**：使用 `async-stream` + `futures::Stream` 实现逐 token 输出
5. **异步 Rust**：`tokio` 异步运行时、`async-trait`、异步文件/进程操作
6. **安全编程**：路径穿越防护、命令注入防御、超时与资源限制
7. **JSON Schema 生成**：使用 `schemars` 从 Rust 类型自动生成 Schema，用于 LLM Structured Output
8. **跨平台 Rust**：`cfg!(target_os)` 条件编译、平台适配策略

## License

See [../LICENSE](../LICENSE) for details.
