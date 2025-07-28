use reqwest::Error;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
struct Repository {
    name: String,
    html_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let username = "martin-g"; // Replace with the GitHub username
    let url = format!("https://api.github.com/users/{username}/repos");

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Rust-Reqwest")
        .send()
        .await?;

    if response.status().is_success() {
        let repos: Vec<Repository> = response.json().await?;
        println!("Repositories for user '{username}':");
        for repo in repos {
            println!("Name: {}, URL: {}", repo.name, repo.html_url);
        }
    } else {
        println!("Failed to fetch repositories: {}", response.status());
    }

    Ok(())
}
