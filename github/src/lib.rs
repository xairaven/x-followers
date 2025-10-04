use reqwest::header;
use serde_json::Value;
use thiserror::Error;

pub fn list(nickname: &str) -> Result<Vec<User>, GithubError> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Connection", "keep-alive".parse().unwrap());

    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(GithubError::ClientBuild)?;

    let mut page = 1;
    let mut users: Vec<User> = vec![];
    loop {
        let url = format!("https://api.github.com/users/{}/followers?page={}", nickname, page);

        let response: Value = client.get(url)
            .headers(headers.clone())
            .send()
            .map_err(GithubError::SendFailed)?
            .json()
            .map_err(GithubError::JsonConversion)?;

        let array = response.as_array()
            .ok_or(GithubError::StructureInvalid)?;

        if array.is_empty() {
            break;
        }

        for value in array {
            let user = User {
                id: value["id"].as_i64().ok_or(GithubError::StructureInvalid)?,
                login: value["login"].as_str().ok_or(GithubError::StructureInvalid)?.to_string(),
                url: value["url"].as_str().ok_or(GithubError::StructureInvalid)?.to_string(),
            };
            users.push(user);
        }

        page += 1;
    }

    Ok(users)
}

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