use clap::Parser;
use reqwest;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: String,
}

fn add_protocol(url: &str) -> String {
    if !url.starts_with("https://") {
        return format!("https://{}", url);
    }
    url.to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let url = add_protocol(&args.url);
    let resp = reqwest::get(url).await?;

    resp.headers()
        .iter()
        .for_each(|(k, v)| println!("{}, {:?}", k, v));

    return Ok(());
}
