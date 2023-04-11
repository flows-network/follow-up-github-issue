use chrono::{Duration, Utc};
use dotenv::dotenv;
use github_flows::get_octo;
use schedule_flows::schedule_cron_job;
use slack_flows::send_message_to_channel;
use std::env::var;

#[no_mangle]
pub fn run() {
    schedule_cron_job(
        String::from("05 * * * *"),
        String::from("cron_job_evoked"),
        callback,
    );
}
#[tokio::main(flavor = "current_thread")]
async fn callback(_body: Vec<u8>) {
    dotenv().ok();

    let login = var("login").unwrap_or("jaykchen".to_string());
    let owner = var("owner").unwrap_or("jaykchen".to_string());
    let repo = var("repo").unwrap_or("a-test".to_string());
    let team = var("team").unwrap_or("ik8".to_string());
    let channel = var("channel").unwrap_or("ch_out".to_string());

    let octocrab = get_octo(Some(login));

    let now = Utc::now();
    let a_week_ago = now - Duration::hours(12);
    let a_week_ago_formatted = a_week_ago.format("%Y-%m-%d").to_string();
    let query = format!(
        "repo:{owner}/{repo} is:issue state:open comments:0 updated:>={a_week_ago_formatted}"
    );

    let res = octocrab
        .search()
        .issues_and_pull_requests(&query)
        .send()
        .await;

    if let Ok(page) = res {
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

            send_message_to_channel(&team, &channel, msg);
        }
        send_message_to_channel(&team, &channel, query);
    }
}
