use async_openai::types::chat::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs, FinishReason, ResponseFormat, ResponseFormatJsonSchema,
};
use backon::{ExponentialBuilder, Retryable};

use crate::gaia::model::GaiaOutput;

/// 结构化输出的请求方式。
///
/// 各家平台的 OpenAI 兼容接口对 `response_format` 的支持程度不同：
/// - `JsonSchema`：最严格，OpenAI / 阿里云百炼部分模型 / OpenRouter 免费模型支持
/// - `JsonObject`：只保证是合法 JSON，DeepSeek / 多数国内平台支持
/// - `PromptOnly`：完全靠提示词约束，任何模型都能用（最后兜底）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputMode {
    JsonSchema,
    JsonObject,
    PromptOnly,
}

impl OutputMode {
    /// 依次尝试的降级顺序。
    const LADDER: [OutputMode; 3] = [
        OutputMode::JsonSchema,
        OutputMode::JsonObject,
        OutputMode::PromptOnly,
    ];

    fn label(&self) -> &'static str {
        match self {
            OutputMode::JsonSchema => "json_schema",
            OutputMode::JsonObject => "json_object",
            OutputMode::PromptOnly => "prompt",
        }
    }
}

pub async fn solve_problem_with_retry(
    model: &str,
    system: &str,
    prompt: &str,
) -> anyhow::Result<GaiaOutput> {
    let op = || async { solve_problem(model, system, prompt).await };
    op.retry(ExponentialBuilder::default().with_max_times(3))
        .await
}

async fn solve_problem(model: &str, system: &str, prompt: &str) -> anyhow::Result<GaiaOutput> {
    let schema = schemars::schema_for!(GaiaOutput);
    let schema_json = serde_json::to_value(&schema)?;

    let mut last_error = None;

    // 逐级降级：优先用最严格的结构化输出，被平台拒绝时退到更宽松的方式。
    for mode in OutputMode::LADDER {
        match request_once(model, system, prompt, mode, &schema_json).await {
            Ok(output) => {
                if mode != OutputMode::JsonSchema {
                    tracing::warn!("已降级到 {} 模式获取结构化输出", mode.label());
                }
                return Ok(output);
            }
            Err(err) => {
                // 只有「平台不支持该参数」才值得降级；鉴权/额度/网络问题应直接上抛。
                if !is_unsupported_param(&err) {
                    return Err(err);
                }
                tracing::warn!(
                    "平台不支持 {} 模式({err:#})，尝试降级",
                    mode.label()
                );
                last_error = Some(err);
            }
        }
    }

    Err(last_error.unwrap_or_else(|| anyhow::anyhow!("all output modes failed")))
}

/// 判断错误是否为「该参数不被支持」，而非鉴权、额度等其它问题。
fn is_unsupported_param(err: &anyhow::Error) -> bool {
    let text = err.to_string().to_lowercase();
    const MARKERS: [&str; 8] = [
        "response_format",
        "json_schema",
        "unsupported",
        "not support",
        "invalid_request_error",
        "unrecognized request argument",
        "unknown field",
        "400",
    ];
    MARKERS.iter().any(|marker| text.contains(marker))
}

/// 构造 OpenAI 兼容客户端。
///
/// 关键点：`OPENAI_BASE_URL` 若带尾斜杠（如 `https://host/v1/`），拼接后会变成
/// `/v1//chat/completions`，网关返回纯文本 `404 page not found` —— 这不是 JSON，
/// 反序列化会报出难以定位的 `invalid type: integer 404`。
/// 这里统一去掉尾斜杠，从源头规避。
fn build_client() -> anyhow::Result<async_openai::Client<async_openai::config::OpenAIConfig>> {
    let config = async_openai::config::OpenAIConfig::new()
        .with_api_key(std::env::var("OPENAI_API_KEY").unwrap_or_default())
        .with_api_base(normalized_api_base());

    Ok(async_openai::Client::with_config(config))
}

/// 规整 base url：去掉尾部多余的斜杠。
fn normalized_api_base() -> String {
    let raw = std::env::var("OPENAI_BASE_URL")
        .unwrap_or_else(|_| "https://api.openai.com/v1".to_string());

    let trimmed = raw.trim().trim_end_matches('/');
    if trimmed.is_empty() {
        return "https://api.openai.com/v1".to_string();
    }

    if trimmed != raw {
        tracing::warn!("OPENAI_BASE_URL 含有尾斜杠，已规整为 {trimmed}");
    }
    trimmed.to_string()
}

/// 给请求错误补充诊断信息。
///
/// 网关返回非 JSON 的纯文本（如 `404 page not found`）时，SDK 的反序列化错误
/// 会掩盖真实原因（实际是 URL 拼错或路径不存在）。这里还原成可读提示。
fn annotate_request_error(err: async_openai::error::OpenAIError, api_base: &str) -> anyhow::Error {
    let text = err.to_string();

    // 形如 "invalid type: integer `404` ... content:404 page not found"
    let looks_like_plain_text_404 = text.contains("404 page not found")
        || (text.contains("invalid type") && text.contains("expected struct"));

    if looks_like_plain_text_404 {
        return anyhow::anyhow!(
            "{text}\n\n提示：服务端返回了非 JSON 的 404 响应，通常是请求路径不对。\
             当前 OPENAI_BASE_URL={api_base}，实际请求路径为 {api_base}/chat/completions。\
             请检查 base url 是否多写了尾斜杠或前缀（如应为 .../v1 而非 .../v1/）。"
        );
    }

    anyhow::Error::new(err)
}

/// 按指定的输出方式发起一次请求。
async fn request_once(
    model: &str,
    system: &str,
    prompt: &str,
    mode: OutputMode,
    schema_json: &serde_json::Value,
) -> anyhow::Result<GaiaOutput> {
    let client = build_client()?;

    // PromptOnly 模式下，把 schema 写进提示词作为兜底约束。
    let system = match mode {
        OutputMode::PromptOnly => format!(
            "{system}\n\nYou must answer with a single JSON object that strictly matches this JSON schema:\n{schema_json}\n\
             Reply with the JSON object only, without any extra text, markdown fences, or explanation."
        ),
        _ => system.to_string(),
    };

    // builder 采用 mutable pattern：setter 返回 &mut Self，因此绑定到 mut 变量后链式调用。
    let mut request = CreateChatCompletionRequestArgs::default();
    request.model(model);
    request.messages([
        ChatCompletionRequestSystemMessageArgs::default()
            .content(system)
            .build()?
            .into(),
        ChatCompletionRequestUserMessageArgs::default()
            .content(prompt)
            .build()?
            .into(),
    ]);

    // 按模式设置 response_format。
    match mode {
        OutputMode::JsonSchema => {
            request.response_format(ResponseFormat::JsonSchema {
                json_schema: ResponseFormatJsonSchema {
                    description: Some("Gaia solve problem".to_string()),
                    name: "Gaia output".to_string(),
                    schema: schema_json.clone(),
                    strict: Some(true),
                },
            });
        }
        // json_object 模式要求提示词里出现 "json" 字样，上面已确保包含。
        OutputMode::JsonObject => {
            request.response_format(ResponseFormat::JsonObject);
        }
        OutputMode::PromptOnly => {}
    }

    let response = client
        .chat()
        .create(request.build()?)
        .await
        .map_err(|e| annotate_request_error(e, &normalized_api_base()))?;
    let choices = response
        .choices
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("No choices in response"))?;

    if choices.finish_reason == Some(FinishReason::ContentFilter) {
        return Ok(GaiaOutput {
            is_solvable: false,
            unsolvable_reason: "Model refuse to answer".to_string(),
            final_answer: String::new(),
        });
    }

    let content = choices
        .message
        .content
        .ok_or_else(|| anyhow::anyhow!("No content in response"))?;

    // 部分模型会把内容包在 ```json 代码块里，这里剥掉围栏再解析。
    let output: GaiaOutput = serde_json::from_str(&strip_code_fence(&content))
        .map_err(|e| anyhow::anyhow!("failed to parse model output as JSON: {e}; raw={content}"))?;

    Ok(output)
}

/// 去掉 Markdown 代码块围栏。
fn strip_code_fence(content: &str) -> &str {
    let trimmed = content.trim();
    if !trimmed.starts_with("```") {
        return trimmed;
    }

    // 跳过开头的 ```lang
    let after_open = trimmed.split_once('\n').map_or(trimmed, |(_, rest)| rest);
    // 去掉结尾的 ```
    after_open
        .strip_suffix("```")
        .unwrap_or(after_open)
        .trim()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, MutexGuard, OnceLock};

    /// 串行化会修改环境变量的用例。
    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    #[test]
    fn strips_plain_json_unchanged() {
        let raw = r#"{"is_solvable":true,"unsolvable_reason":"","final_answer":"121"}"#;
        assert_eq!(strip_code_fence(raw), raw);
    }

    #[test]
    fn strips_markdown_code_fence() {
        let raw = "```json\n{\"final_answer\": \"121\"}\n```";
        assert_eq!(strip_code_fence(raw), r#"{"final_answer": "121"}"#);
    }

    #[test]
    fn strips_fence_without_language() {
        let raw = "```\n{\"a\":1}\n```";
        assert_eq!(strip_code_fence(raw), r#"{"a":1}"#);
    }

    #[test]
    fn trims_surrounding_whitespace() {
        let raw = "  {\"a\":1}  ";
        assert_eq!(strip_code_fence(raw), r#"{"a":1}"#);
    }

    /// 参数不被支持时才降级；鉴权/额度等错误应直接上抛。
    #[test]
    fn detects_unsupported_param_errors() {
        let unsupported = anyhow::anyhow!(
            "error sending request: 400 Bad Request: Unsupported parameter: 'response_format' is not supported"
        );
        assert!(is_unsupported_param(&unsupported));

        let unrecognized = anyhow::anyhow!("Invalid request: unrecognized request argument supplied: json_schema");
        assert!(is_unsupported_param(&unrecognized));
    }

    #[test]
    fn does_not_treat_auth_or_quota_errors_as_unsupported() {
        let auth = anyhow::anyhow!("401 Unauthorized: Authentication failed");
        assert!(!is_unsupported_param(&auth));

        let quota = anyhow::anyhow!("429 Too Many Requests: Rate limit exceeded");
        assert!(!is_unsupported_param(&quota));
    }

    /// 降级阶梯的顺序：先最严格，最后兜底。
    #[test]
    fn ladder_order_is_strict_to_loose() {
        assert_eq!(
            OutputMode::LADDER,
            [
                OutputMode::JsonSchema,
                OutputMode::JsonObject,
                OutputMode::PromptOnly
            ]
        );
    }

    /// base url 的尾斜杠必须被去掉，否则会拼出 `/v1//chat/completions` 导致 404。
    ///
    /// 环境变量是进程级全局状态，这些用例必须串行执行，否则会互相污染。
    #[test]
    fn strips_trailing_slash_from_base_url() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());

        let cases = [
            ("https://api-inference.modelscope.cn/v1/", "https://api-inference.modelscope.cn/v1"),
            ("https://api-inference.modelscope.cn/v1", "https://api-inference.modelscope.cn/v1"),
            ("https://api-inference.modelscope.cn/v1///", "https://api-inference.modelscope.cn/v1"),
            ("  https://x.dev/v1/  ", "https://x.dev/v1"),
        ];

        for (input, expected) in cases {
            // SAFETY: 已用 env_lock 串行化。
            unsafe { std::env::set_var("OPENAI_BASE_URL", input) };
            assert_eq!(normalized_api_base(), expected, "input={input:?}");
        }

        unsafe { std::env::remove_var("OPENAI_BASE_URL") };
    }

    /// 空值或全空白时回退到官方默认地址，不应产生空 base url。
    #[test]
    fn falls_back_to_default_when_base_url_is_blank() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());

        for input in ["", "   ", "/", "///"] {
            // SAFETY: 已用 env_lock 串行化。
            unsafe { std::env::set_var("OPENAI_BASE_URL", input) };
            assert_eq!(normalized_api_base(), "https://api.openai.com/v1");
        }

        unsafe { std::env::remove_var("OPENAI_BASE_URL") };
    }

    /// 未设置环境变量时同样回退到默认值。
    #[test]
    fn falls_back_to_default_when_base_url_unset() {
        let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());

        unsafe { std::env::remove_var("OPENAI_BASE_URL") };
        assert_eq!(normalized_api_base(), "https://api.openai.com/v1");
    }
}
