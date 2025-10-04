use crate::error::Error;

pub fn main(request: Request) -> Result<Vec<String>, Error> {
    match request {
        Request::Github { nickname } => {
            let users = github::list(nickname)?;
            let lines = users
                .into_iter()
                .map(|user| format!("{} [{}], URL: {}", user.login, user.id, user.url))
                .collect();
            Ok(lines)
        },
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Request {
    Github { nickname: &'static str },
}

pub mod error;
pub mod github;
