use serde::Deserialize;
use std::{collections::{HashSet, HashMap}, env};

#[derive(Deserialize, Debug)]
pub struct Repos {
    //total_count: i32,
    //incomplete_results: bool,
    items: HashSet<Repo>,
}

#[derive(Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Repo {
    id: usize,
    node_id: String,
    name: String,
    full_name: String,
    private: bool,
}


#[derive(Deserialize, Debug, PartialEq, Eq, Hash)]
struct Author{
    name: String,
    email: String,
    date: String,
}



#[derive(Deserialize, PartialEq, Eq, Hash)]
struct Commit{
    author: HashMap<String, String>,
}



#[derive(Deserialize, PartialEq, Eq)]
struct Commits(HashSet<Commit>);


//const REPO: &str = "https://api.github.com/users/ilyes-guy/repos?visibility=all&per_page=1000";
const API: &str = "https://api.github.com/search/repositories?q=user:";
//const API: &str = "https://api.github.com/repos/ilyes-guy";




#[tokio::main]
async fn main() {
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() != 3 {
        println!("requires 3 args github_username github_oken ");
        std::process::exit(1)
    }




    let github_username = &arguments[1];
    let github_token = &arguments[2];

    println!("{:?}", github_username);
    println!("{:?}", github_token);





    let mut repos = String::new();
    repos.push_str(API);
    repos.push_str(github_username);
    repos.push_str("&per_page=1000");



    let client = reqwest::Client::new();
    let mut token_bearer = String::from("Bearer ");
    token_bearer.push_str(github_token);

    
    let res = client
        .get(repos)
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "random dudde")
        .header(
            "Authorization",
            token_bearer,
        )
        .send()
        .await;

    let mut all_repos: Vec<String> = Vec::new();
    match res {
        Ok(result) => {
            match result.json::<Repos>().await {
                Ok(result) => {
                    println!("{:?}", result.items.len());
                    for repo in result.items {
                        println!("{:?}", repo.name);
                        all_repos.push(repo.name);
                    }
                }
                Err(err) => {
                    println!("{:?}", err);   
                    println!(
                        "*********************** maybe a wrong username *********************"
                    );
                }
            };
        }
        Err(..) => {
            println!("idk what the error");
        }
    };




    let mut token_bearer = String::from("Bearer ");
    token_bearer.push_str(github_token);

    for repo in all_repos{
        let mut repo_url = String::new();
        repo_url.push_str("https://api.github.com/repos/");
        repo_url.push_str(github_username);
        repo_url.push_str("/");
        repo_url.push_str(&repo);
        repo_url.push_str("/commits");
        //repo_url.push_str("&per_page=10000");
        println!("{:?}", repo_url);   

        let res = client
        .get(repo_url)
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "random dudde")
        .header(
            "Authorization",
            &token_bearer,
        )
        .send()
        .await;



        match res {
            Ok(result) => {
                //println!("{:?}", result.text().await);   
                
                match result.json::<Commits>().await {
                //match result.text().await {
                    Ok(result) => {
                        //let pp: String = serde_json::from_str(&result).expect("Failed to parse json.");
                        let commits = result;
                        println!("{:?}", commits);
                    },
                    Err(err) => {
                        println!("{:?}", err);   
                    }
                }
            },
            Err(err) => {
                println!("{:?}", err);
            },
        }
    }








}
