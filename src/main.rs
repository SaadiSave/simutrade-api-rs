use axum::{
    routing::{get, post},
    Router,
};
use mock::Db;

mod mock;
mod models;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let user_db = Db::init();

    // router subsection for api
    // share user database with only user route (with_state on)
    // note that with_state only allows Db<String, User> for this route,
    // since create_user has it in its type signature
    // that means it's completely type safe; you cannot pass the wrong state
    let api = Router::new().route("/user", post(routes::user::create_user).with_state(user_db));

    // nest api in main router
    let app = Router::new()
        .route("/check", get(|| async { "Ok" }))
        .nest("/api", api);

    // serve
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
