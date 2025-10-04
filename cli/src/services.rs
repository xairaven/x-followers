use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, Debug, ValueEnum)]
pub enum Service {
    Github,
}

impl std::fmt::Display for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let service = match self {
            Self::Github => "Github",
        };
        write!(f, "{}", service)
    }
}
