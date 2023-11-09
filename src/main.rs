use clap::Parser;
use reqwest;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: String,
}

fn add_protocol<'a>(url: &'a str) -> String {
    if !url.starts_with("https://") {
        return format!("https://{}", url);
    }
    url.to_string()
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let url = add_protocol(&args.url);
    let resp = reqwest::get(url).await.unwrap();

    println!("{:?}", resp.headers())
}
