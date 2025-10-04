use reqwest::header;
use thiserror::Error;

pub fn list(nickname: &str) -> Result<Vec<User>, GithubError> {
    let headers = header::HeaderMap::new();

    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(GithubError::ClientBuild)?;

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

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[derive(Debug, Clone)]
pub struct User {
    pub id: i64,
    pub login: String,
    pub url: String,
}

#[derive(Debug, Error)]
pub enum GithubError {
    #[error("Failed to build client. {0}")]
    ClientBuild(reqwest::Error),

    #[error("Failed to convert response to JSON. {0}")]
    JsonConversion(reqwest::Error),

    #[error("Failed to send request. {0}")]
    SendFailed(reqwest::Error),

    #[error("Invalid structure of response.")]
    StructureInvalid,
}
