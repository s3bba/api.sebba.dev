use actix_web::web;

// TODO (sebba): This should probably be moved to a different file
mod health {
    use actix_web::{HttpResponse, Responder, web};

    pub fn configure(cfg: &mut web::ServiceConfig) {
        cfg.route(
            "/health",
            web::get().to(health)
        );
    }

    async fn health() -> impl Responder {
        HttpResponse::Ok()
    }
}

mod v1 {
    pub mod blog;
}

pub fn create_router(cfg: &mut web::ServiceConfig) {
    health::configure(cfg);
    v1::blog::configure(cfg);
}