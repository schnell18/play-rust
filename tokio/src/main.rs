use tokio::join;

#[tokio::main(worker_threads = 2)]
async fn main() {
    let a = tokio::spawn(get_page("https://news.ycombinator.com"));
    let b = tokio::spawn(get_page("https://www.lobste.rs"));
    let c = get_page("https://www.lobste.rs");
    let _ = join!(a, b, c);
}

pub async fn get_page(url: &str) {
    println!("Retrieving {}", url);
    let _ = reqwest::get(url).await.unwrap();
    println!("Completed {}", url);
}
