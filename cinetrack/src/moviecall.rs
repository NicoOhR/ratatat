use reqwest;

#[tokio::main]
pub async fn imdb_call(){
    let result = reqwest::get("https://api.spotify.com/v1/search").await;
    println!("{:?}", result);
}