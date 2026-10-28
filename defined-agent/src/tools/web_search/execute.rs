use chrono::Datelike;

use crate::tools::{
    ToolError,
    web_search::websearch::{
        TavilyRequest, TavilyResponse, WebSearchArgs, WebSearchOutput, max_results,
    },
};

fn query_with_current_year(query: &str, current_year: i32) -> String {
    let has_explicit_year = query
        .split(|character: char| !character.is_ascii_digit())
        .any(|part| {
            part.len() == 4
                && matches!(part.parse::<i32>(), Ok(year) if (1900..=2100).contains(&year))
        });

    if has_explicit_year {
        query.to_owned()
    } else {
        format!("{query} {current_year}年")
    }
}

pub async fn web_search(args: WebSearchArgs) -> Result<WebSearchOutput, ToolError> {
    let api_key = std::env::var("TAVILY_API_KEY")
        .map_err(|_| ToolError::Permanent("TAVILY_API_KEY not found".to_string()))?;
    let api_url = std::env::var("TAVILY_API_URL")
        .map_err(|_| ToolError::Permanent("TAVILY_API_URL not found".to_string()))?;
    let query = query_with_current_year(&args.query, chrono::Local::now().year());
    let request = TavilyRequest {
        api_key: &api_key,
        query: &query,
        // 检索条数由服务端决定，不采信模型传入的值。
        max_results: max_results(),
        topic: &args.topic,
        time_range: args.time_range.as_deref(),
        search_depth: "advanced",
        include_answer: true,
    };

    let resp = reqwest::Client::new()
        .post(api_url)
        .json(&request)
        .send()
        .await
        .map_err(|e| ToolError::Transient(format!("request to Tavily failed: {e}")))?;

    let status = resp.status();
    if !status.is_success() {
        // 5xx / 429 属于服务端或限流问题，重试可能成功；4xx 是请求本身有问题。
        return Err(if status.is_server_error() || status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            ToolError::Transient(format!("Tavily returned status {status}"))
        } else {
            ToolError::Permanent(format!("Tavily returned status {status}"))
        });
    }

    let parsed: TavilyResponse = resp.json().await.map_err(|e| {
        ToolError::Transient(format!("failed to parse Tavily response: {e}"))
    })?;

    Ok(WebSearchOutput {
        answer: parsed.answer,
        results: parsed.results,
    })
}

#[cfg(test)]
mod tests {
    use super::query_with_current_year;

    #[test]
    fn query_without_explicit_year_gets_current_year() {
        assert_eq!(
            query_with_current_year("Rust async runtime", 2026),
            "Rust async runtime 2026年"
        );
    }

    #[test]
    fn query_with_explicit_year_is_preserved() {
        assert_eq!(
            query_with_current_year("2025 Rust async runtime", 2026),
            "2025 Rust async runtime"
        );
    }
}
