pub fn list(request: Request) -> Result<Vec<String>, Error> {
    match request {
        Request::Github { nickname } => {
            let users = github::list(&nickname)?;
            let lines = users
                .into_iter()
                .map(|user| format!("{} [{}], URL: {}", user.login, user.id, user.url))
                .collect();
            Ok(lines)
        },
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Request {
    Github { nickname: String },
}

pub use crate::error::Error;

pub mod error;
pub mod github;
