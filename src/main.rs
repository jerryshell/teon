use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short = 'u', long)]
    url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let reponse_url = teon::run(&args.url).await?;
    println!("{}", reponse_url);
    Ok(())
}
