mod pills;

use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::net::TcpListener;

use pills::application::find::{FindAllPillsQueryHandler, FindPillQueryHandler};
use pills::application::{CreatePillCommandHandler, PillRepository};
use pills::infrastructure::controllers::create_pill_controller::create_pill_handler;
use pills::infrastructure::controllers::find_all_pills_controller::find_all_pills_handler;
use pills::infrastructure::controllers::find_pill_controller::find_pill_by_id_handler;
use pills::infrastructure::in_memory_repository::InMemoryPillRepository;

#[derive(Clone)]
struct AppState {
    create_pill_handler: Arc<CreatePillCommandHandler>,
    find_pill_handler: Arc<FindPillQueryHandler>,
    find_all_pills_handler: Arc<FindAllPillsQueryHandler>,
}

#[tokio::main]
async fn main() {
    let repo: Arc<dyn PillRepository> = Arc::new(InMemoryPillRepository::new());

    let app_state = AppState {
        create_pill_handler: Arc::new(CreatePillCommandHandler::new(repo.clone())),
        find_pill_handler: Arc::new(FindPillQueryHandler::new(repo.clone())),
        find_all_pills_handler: Arc::new(FindAllPillsQueryHandler::new(repo.clone())),
    };

    let app = Router::new()
        .route("/pills", post(create_pill_handler))
        .with_state(app_state.create_pill_handler)
        .route("/pills/:id", get(find_pill_by_id_handler))
        .with_state(app_state.find_pill_handler)
        .route("/pills", get(find_all_pills_handler))
        .with_state(app_state.find_all_pills_handler);

    println!("🚀 Servidor escuchando en 0.0.0.0:3000");
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
