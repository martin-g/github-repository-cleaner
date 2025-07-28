use reqwest::Error;
use serde::Deserialize;
use std::io::{self, Write};

#[derive(Deserialize, Debug)]
struct Repository {
    name: String,
    html_url: String,
    fork: bool,
}

const USERNAME: &str = "martin-g";
const PER_PAGE: u32 = 250;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv().ok();

    let url = format!("https://api.github.com/users/{USERNAME}/repos?per_page={PER_PAGE}&type=forks");
    let client = reqwest::Client::new();
    let github_token = std::env::var("GITHUB_TOKEN")
        .expect("GITHUB_TOKEN environment variable not set");

    let response = client
        .get(&url)
        .header("User-Agent", "Rust-Reqwest")
        .header("Accept", "application/vnd.github.v3+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("Authorization",format!("Bearer {github_token}"))
        .send()
        .await?;

    if response.status().is_success() {
        let repos: Vec<Repository> = response.json().await?;
        println!("Repositories for user '{USERNAME}':");
        for repo in repos {
            if repo.fork {
                print!("Do you want to delete repository: {}, URL: {} ? y/n: ", repo.name, repo.html_url);
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                if input.trim().eq_ignore_ascii_case("y") {
                    let delete_url = format!("https://api.github.com/repos/{USERNAME}/{}", repo.name);
                    let delete_response = client
                        .delete(&delete_url)
                        .header("User-Agent", "Rust-Reqwest")
                        .header("Authorization", format!("Bearer {github_token}"))
                        .send()
                        .await?;

                    if delete_response.status().is_success() {
                        println!("Repository '{}' deleted successfully.", repo.name);
                    } else {
                        println!("Failed to delete repository '{}': {:?}", repo.name, delete_response);
                    }
                }
            }
        }
    } else {
        println!("Failed to fetch repositories: {}", response.status());
    }

    Ok(())
}
