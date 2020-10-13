use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(
        "https://raw.githubusercontent.com/sameerkumar18/geek-joke-api/master/data.json",
    )
    .await?
    .json::<Vec<String>>()
    .await?;
    println!("{:#?}", resp);

    let mut file = File::create("jokes.bin")?;

    let resp = bincode::serialize(&resp)?;

    file.write_all(&resp)?;

    Ok(())
}
