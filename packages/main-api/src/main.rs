use by_axum::{auth::authorization_middleware, axum::middleware};
use by_types::DatabaseConfig;
use dto::error::ServiceError;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

mod controllers {
    pub mod users {
        pub mod v1;
    }
}

pub mod config;

#[tokio::main]
async fn main() -> Result<(), ServiceError> {
    let conf = config::get();
    let _ = tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .try_init();

    let pool = if let DatabaseConfig::Postgres { url, pool_size } = conf.database {
        PgPoolOptions::new()
            .max_connections(pool_size)
            .connect(url)
            .await?
    } else {
        panic!("Database is not initialized. Call init() first.");
    };

    let app = by_axum::new()
        .nest(
            "/users/v1",
            controllers::users::v1::UserControllerV1::route(pool.clone()).await?,
        )
        .layer(middleware::from_fn(authorization_middleware));

    let port = option_env!("PORT").unwrap_or("3000");
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    by_axum::serve(listener, app).await.unwrap();

    Ok(())
}
