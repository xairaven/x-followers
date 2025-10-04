use crate::cli::Cli;
use crate::services::Service;
use chrono::Local;
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    let date = Local::now();

    let request = match cli.service {
        Service::Github => api::Request::Github {
            nickname: cli.nickname.clone(),
        },
    };

    let users = api::list(request);
    let users = match users {
        Ok(users) => users,
        Err(error) => {
            eprintln!("API Error: {}", error);
            return;
        },
    };

    let mut text = String::new();

    if !cli.exclude_description {
        let description = format!(
            "# List of {} followers on {} for {}. Date: {}",
            users.len(),
            cli.service,
            cli.nickname,
            date.format("%d.%m.%Y %H:%M")
        );
        text.push_str(&description);
        text.push('\n');
    }

    for user in users {
        text.push_str(&user);
        text.push('\n');
    }

    match cli.output_path {
        None => println!("{}", text),
        Some(path) => {
            let filename = cli.file_name.unwrap_or(format!(
                "{}-{}-{}.txt",
                &cli.service,
                &cli.nickname,
                date.format("%d-%m-%Y-%H-%M")
            ));
            let path = path.join(filename);

            std::fs::write(path, text).unwrap_or_else(|error| {
                eprintln!("IO Error: {}", error);
            });
        },
    };
}

pub mod cli;
pub mod services;
