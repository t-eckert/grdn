use super::api::health_check;
use super::ui::index;

use rocket::fs::FileServer;

pub async fn run() -> Result<(), rocket::Error> {
    println!("Running server...");
    let _rocket = rocket::build()
        .mount("/api", routes![health_check])
        .mount("/hello", routes![index])
        .mount(
            "/",
            FileServer::from(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/server/.svelte-kit/output/prerendered/pages"
            )),
        )
        .launch()
        .await?;

    Ok(())
}
