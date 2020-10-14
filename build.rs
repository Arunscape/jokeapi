use std::fs::OpenOptions;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(
        "https://raw.githubusercontent.com/sameerkumar18/geek-joke-api/master/data.json",
    )
    .await?
    .text()
    .await?;
    //println!("{:#?}", resp);

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("jokes.txt")?;

    file.write_all(resp.as_bytes())?;

    Ok(())
}
