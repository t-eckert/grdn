pub async fn run() -> Result<(), rocket::Error> {
    println!("Running client...");
    let _rocket = rocket::build().launch().await?;

    Ok(())
}
