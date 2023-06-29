use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, ACCEPT};
use reqwest::blocking::Client;
use serde::Serialize;

#[derive(Serialize)]
struct IssuePayload {
    title: String,
    body: String,
}

fn create_github_issue(
    repo_owner: &str,
    repo_name: &str,
    title: &str,
    body: &str,
    token: &str,
) -> Result<(), reqwest::Error> {
    let url = format!("https://api.github.com/repos/{}/{}/issues", repo_owner, repo_name);

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
    );
    headers.insert(ACCEPT, HeaderValue::from_static("application/vnd.github.v3+json"));

    let payload = IssuePayload {
        title: title.to_owned(),
        body: body.to_owned(),
    };

    let client = Client::new();
    let response = client
        .post(&url)
        .headers(headers)
        .json(&payload)
        .send()?;

    if response.status().is_success() {
        println!("Issue created successfully.");
    }

    Ok(())
}
