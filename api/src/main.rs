use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware};
use actix_web::web::Data;
use logw::tracing::{error, info};
use sentry::ClientInitGuard;
use sqlx::{PgPool, Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use crate::util::configuration::Configuration;

mod route;

pub mod util {
    pub mod authentication;
    pub mod capture;
    pub mod configuration;
    pub mod response;
}

const MIDDLEWARE_LOG_FORMAT: &str = "%r (%s, %Dms, %bb, %a, %{User-Agent}i)";

pub struct PgDatabase {
    pool: PgPool
}

// TODO (sebba): Introduce anyhow for error handling

#[tokio::main]
async fn main() -> std::io::Result<()> {
    logw::init();
    let config: Configuration = util::configuration::load();
    util::authentication::set_credential(&config.management_api_key);

    let pool: Pool<Postgres> = match PgPoolOptions::new()
        .max_connections(config.database_max_connections)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            info!("Connected to the database");
            pool
        }
        Err(err) => {
            error!("Failed to connect to the database - {:?}", err);
            panic!("Exiting due to failed database connection attempt")
        }
    };

    let _guard: ClientInitGuard;

    match &config.sentry_dsn {
        Some(value) => {
            _guard = sentry::init((value.as_str(), sentry::ClientOptions {
                release: sentry::release_name!(),
                ..Default::default()
            }));
        }
        None => {}
    }

    let db_data: Data<PgDatabase> = Data::new(PgDatabase {
        pool: pool.clone()
    });

    // TODO (sebba): Implement prometheus metrics:
    // TODO (sebba): - Request and response metrics
    // TODO (sebba): - Database connection pool metrics
    // TODO (sebba): - Blog metrics

    info!("Starting server on http://{}:{}", config.service_ip, config.service_port);

    HttpServer::new(move || {
        // Set up CORS configuration
        let mut cors_config: Cors = Cors::default();
        for origin in &config.allowed_origins {
            cors_config = cors_config.allowed_origin(origin);
        }

        App::new()
            .app_data(db_data.clone())
            .configure(route::create_router)
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::new(MIDDLEWARE_LOG_FORMAT))
            .wrap(cors_config)
    }).bind((config.service_ip, config.service_port))?.run().await
}
