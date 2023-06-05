use chrono::{Duration, Utc};
use dotenv::dotenv;
use github_flows::{get_octo, GithubLogin::Default};
use schedule_flows::schedule_cron_job;
use slack_flows::send_message_to_channel;
use std::env;

#[no_mangle]
pub fn run() {
    dotenv().ok();
    //time_to_invoke is a string of 3 numbers separated by spaces, representing minute, hour, and day
    //* is the spaceholder for non-specified numbers
    let mut time_to_invoke = env::var("time_to_invoke").unwrap_or("* 12 *".to_string());
    time_to_invoke.push_str(" * *");

    schedule_cron_job(time_to_invoke, String::from("cron_job_evoked"), callback);
}


#[tokio::main(flavor = "current_thread")]
async fn callback(_body: Vec<u8>) {
    let github_owner = env::var("github_owner").unwrap_or("alabulei1".to_string());
    let github_repo = env::var("github_repo").unwrap_or("a-test".to_string());
    let slack_workspace = env::var("slack_workspace").unwrap_or("secondstate".to_string());
    let slack_channel = env::var("slack_channel").unwrap_or("github-status".to_string());

    let octocrab = get_octo(&Default);

    let now = Utc::now();
    let a_week_ago = now - Duration::days(7);
    let a_week_ago_formatted = a_week_ago.format("%Y-%m-%d").to_string();
    let query = format!(
        "repo:{github_owner}/{github_repo} is:issue state:open comments:0 updated:>{a_week_ago_formatted}"
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

            send_message_to_channel(&slack_workspace, &slack_channel, msg);
        }
    }
}
