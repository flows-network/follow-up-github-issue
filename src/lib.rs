use chrono::{DateTime, Datelike, Duration, Utc};
use github_flows::{
    get_octo, listen_to_event, octocrab::models::events::payload::PullRequestEventAction,
    EventPayload,
};
use http_req::uri::Uri;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use slack_flows::send_message_to_channel;
use dotenv::dotenv;
use std::env;
#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run()  {
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
        Err(_) => "vitesse-lite".to_string(),
        Ok(name) => name,
    };

    listen_to_event(&login, &owner, &repo, vec!["pull_request", "issues", "issue_comment"], |payload| {
        handler(&login, &owner, &repo, payload)
    })
    .await;

}

async fn handler(login: &str, owner: &str, repo: &str, payload: EventPayload) {
    let octocrab = get_octo(Some(String::from(login)));

    let search_key_word = "GitHub";
    let query_params = format!("q={search_key_word}+is:issue+state:open+comments:0+updated:<2023-03-18");
    let response = octocrab
        .search()
        .issues_and_pull_requests(&query_params)
        // .since("2023-03-18")
        .sort("created")
        .order("desc")
        .send()
        .await;

    match response {
        Err(_e) => {}

        Ok(search_result) => {
            let now = Utc::now();
            let one_week_earlier = now - Duration::days(7);
            for item in search_result.items {
                let name = item.user.login;
                let title = item.title;
                let html_url = item.html_url;
                let time = item.created_at;

                if time > one_week_earlier {
                    // let text = format!(
                    //     "{name} mentioned WASMEDGE in issue: {title}  @{html_url}\n{time}"
                    // );

                    let data = serde_json::json!({
                        "Name": name,
                        "Repo": html_url,
                        "Created": time,
                    });

                    send_message_to_channel("ik8", "ch_out", data.to_string());
                }
            }
        }
    };

}

#[derive(Debug, Deserialize)]
struct SearchResult {
    items: Vec<Issue>,
}

#[derive(Debug, Deserialize)]
struct Issue {
    html_url: String,
    title: String,
    user: User,
    created_at: String,
}

#[derive(Debug, Deserialize)]
struct User {
    login: String,
}
