use reqwest::header;
use thiserror::Error;

pub fn list(nickname: &str) -> Result<Vec<User>, GithubError> {
    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(GithubError::ClientBuild)?;

    let mut headers = header::HeaderMap::new();

    headers.insert(
        "User-Agent",
        "X-Followers"
            .parse()
            .map_err(|_| GithubError::InvalidUserAgent)?,
    );

    let mut page = 1;
    let mut users: Vec<User> = vec![];
    loop {
        let url = format!(
            "https://api.github.com/users/{}/followers?page={}",
            nickname, page
        );

        let mut new_users: Vec<User> = client
            .get(url)
            .headers(headers.clone())
            .send()
            .map_err(GithubError::SendFailed)?
            .json::<Vec<User>>()
            .map_err(GithubError::JsonConversion)?;

        if new_users.is_empty() {
            break;
        }

        users.append(&mut new_users);

        page += 1;
    }

    Ok(users)
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct User {
    pub id: i64,
    pub login: String,
    pub html_url: String,
}

#[derive(Debug, Error)]
pub enum GithubError {
    #[error("Failed to build client. {0}")]
    ClientBuild(reqwest::Error),

    #[error("Invalid user agent.")]
    InvalidUserAgent,

    #[error("Failed to convert response to JSON. {0}")]
    JsonConversion(reqwest::Error),

    #[error("Failed to send request. {0}")]
    SendFailed(reqwest::Error),

    #[error("Invalid structure of response.")]
    StructureInvalid,
}
