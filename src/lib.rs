use chrono::{DateTime, Datelike, Duration, Utc};
use github_flows::get_octo;
use http_req::uri::Uri;
// use openai_flows::encode;
use schedule_flows::schedule_cron_job;
use serde::Deserialize;
use serde_json::Value;
use slack_flows::send_message_to_channel;
use urlencoding::encode;
#[no_mangle]
pub fn run() {
    schedule_cron_job(
        String::from("36 * * * *"),
        String::from("cron_job_evoked"),
        callback,
    );
}

// let github_token = "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".to_string();

fn callback(_body: Vec<u8>) {
    let login = "jaykchen";
    let owner = "jaykchen";
    let repo = "vitesse-lite";
    let search_key_word = "GitHub WASMEDGE";
    let mut writer = Vec::new();

    let octocrab = get_octo(Some(String::from(login)));

    // let instance = octocrab.
    // q=windows+label:bug+language:python+state:open+comments:0+updated:<YYYY-MM-DD&sort=created&order=asc

    // let query_params = format!("q={search_keyword}+repo:jaykchen/vitesse-lite+is:issue+state:open+comments:0+updated:<2023-03-18&sort=created&order=asc");

    // let query_params: Value = serde_json::json!({
    //     "q": search_key_word,
    //     "sort": "created",
    //     "order": "desc"
    // });
    let query_params = encode("q=windows+label:bug+language:python+state:open+comments:0+updated:<2023-03-18&sort=created&order=asc");
    // let query_string = (&query_params).unwrap();
    let url_str = format!("https://api.github.com/search/issues?{}", query_params);

    let url = Uri::try_from(url_str.as_str()).unwrap();

    // match Request::new(&url)
    //     .method(Method::GET)
    //     .header("User-Agent", "flows-network connector")
    //     .header("Content-Type", "application/vnd.github.v3+json")
    //     .send(&mut writer)
    // {
    //     Ok(res) => {
    //         if !res.status_code().is_success() {}
    //         let response: Result<SearchResult, _> = serde_json::from_slice(&writer);
    //         match response {
    //             Err(_e) => {}

    //             Ok(search_result) => {
    //                 let now = Utc::now();
    //                 // let one_hour_ago = now - Duration::minutes(60);
    //                 let one_day_earlier = now - Duration::minutes(1440);
    //                 for item in search_result.items {
    //                     let name = item.user.login;
    //                     let title = item.title;
    //                     let html_url = item.html_url;
    //                     let time = item.created_at;
    //                     let parsed = DateTime::parse_from_rfc3339(&time).unwrap_or_default();

    //                     if parsed > one_day_earlier {
    //                         // let text = format!(
    //                         //     "{name} mentioned WASMEDGE in issue: {title}  @{html_url}\n{time}"
    //                         // );

    //                         let data = serde_json::json!({
    //                             "Name": name,
    //                             "Repo": html_url,
    //                             "Created": time,
    //                         });

    //                         send_message_to_channel("ik8", "ch_out", data.to_string());
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     Err(_e) => {}
    // }
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
