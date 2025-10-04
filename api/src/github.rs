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

        let response = client
            .get(url)
            .headers(headers.clone())
            .send()
            .map_err(GithubError::SendFailed)?
            .text()
            .map_err(GithubError::SendFailed)?;

        if let Ok(error_message) = serde_json::from_str::<ErrorMessage>(&response) {
            return Err(GithubError::Api(error_message.to_string()));
        }

        let mut new_users: Vec<User> =
            serde_json::from_str(&response).map_err(GithubError::JsonConversion)?;

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

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ErrorMessage {
    pub message: String,
    pub documentation_url: String,
}

impl std::fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Message: {}. Documentation URL: {}",
            self.message, self.documentation_url
        )
    }
}

#[derive(Debug, Error)]
pub enum GithubError {
    #[error("{0}")]
    Api(String),

    #[error("Failed to build client. {0}")]
    ClientBuild(reqwest::Error),

    #[error("Invalid user agent.")]
    InvalidUserAgent,

    #[error("Failed to convert response to JSON. {0}")]
    JsonConversion(#[from] serde_json::Error),

    #[error("Failed to send request. {0}")]
    SendFailed(reqwest::Error),

    #[error("Invalid structure of response.")]
    StructureInvalid,
}
