use crate::github;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("GitHub. {0}")]
    Github(#[from] github::GithubError),
}
