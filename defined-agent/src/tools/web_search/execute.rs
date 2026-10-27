use anyhow::Context;

use crate::tools::web_search::websearch::{TavilyRequest, TavilyResponse, WebSearchArgs, WebSearchOutput};

pub async fn web_search(args: WebSearchArgs) -> anyhow::Result<WebSearchOutput> {
    let api_key = std::env::var("TAVILY_API_KEY").context("TAVILY_API_KEY not found")?;
    let api_url = std::env::var("TAVILY_API_URL").context("TAVILY_API_URL not found")?;
    let request = TavilyRequest {
        api_key: &api_key,
        query: &args.query,
        max_results: args.max_results,
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
        .context("request to Tavily failed")?;


    if !resp.status().is_success() {
        anyhow::bail!("Tavily returned status {}", resp.status());
    }

    let parsed: TavilyResponse = resp
        .json()
        .await
        .context("Failed to parse Tavily response")?;

    Ok(WebSearchOutput {
        answer: parsed.answer,
        results: parsed.results,
    })
}
