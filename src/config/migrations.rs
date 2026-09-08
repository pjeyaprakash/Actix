use refinery::embed_migrations;
use deadpool_postgres::Pool;

embed_migrations!("migrations");


pub async fn run(pool: &Pool) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = pool.get().await?;

    migrations::runner()
        .run_async(&mut **client)
        .await?;

    Ok(())
}