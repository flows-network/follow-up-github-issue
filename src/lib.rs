use chrono::{DateTime, Datelike, Duration, Utc};
use dotenv::dotenv;
use github_flows::{
    get_octo, listen_to_event, octocrab::models::events::payload::PullRequestEventAction,
    EventPayload,
};
use http_req::uri::Uri;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use slack_flows::send_message_to_channel;
use std::env;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    dotenv().ok();

    let login: String = match env::var("login") {
        Err(_) => "jaykchen".to_string(),
        Ok(name) => name,
    };

    let owner: String = match env::var("owner") {
        Err(_) => "jaykchen".to_string(),
        Ok(name) => name,
    };

    let repo: String = match env::var("repo") {
        Err(_) => "a-test".to_string(),
        Ok(name) => name,
    };

    listen_to_event(
        &login,
        &owner,
        &repo,
        vec!["pull_request", "issues", "issue_comment"],
        |payload| handler(&login, &owner, &repo, payload),
    )
    .await;
}

async fn handler(login: &str, owner: &str, repo: &str, payload: EventPayload) {
    let octocrab = get_octo(Some(String::from(login)));

    let now = Utc::now();
    let a_week_ago = now - chrono::Duration::days(7);
    let a_week_ago_formatted = a_week_ago.format("%Y-%m-%d").to_string();
    let query = format!(
        "repo:{owner}/{repo} is:issue state:open comments:0 updated:>{a_week_ago_formatted}"
    );

    let res = octocrab
        .search()
        .issues_and_pull_requests(&query)
        .send()
        .await;

    match res {
        Ok(page) => {
            for item in page {
                let title = item.title;
                let html_url = item.html_url;
                let time = item.created_at.format("%Y-%m-%d").to_string();
                let msg = format!(
                    r#"These issues call for your attention:
                {title}
                {html_url}
                created@{time}"#
                );

                send_message_to_channel("ik8", "ch_out", msg);
            }
        }
        Err(error) => {}
    }
}
