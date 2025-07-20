mod pills;

use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::net::TcpListener;

// Importamos los nuevos handlers y casos de uso
use pills::application::find::{FindAllPillsQueryHandler, FindPillQueryHandler};
use pills::application::{CreatePillCommandHandler, PillRepository};
use pills::infrastructure::controllers::create_pill_controller::create_pill_handler;
use pills::infrastructure::controllers::find_all_pills_controller::find_all_pills_handler;
use pills::infrastructure::controllers::find_pill_controller::find_pill_by_id_handler;
use pills::infrastructure::in_memory_repository::InMemoryPillRepository;

// Creamos un struct para agrupar el estado de la aplicación
#[derive(Clone)]
struct AppState {
    create_pill_handler: Arc<CreatePillCommandHandler>,
    find_pill_handler: Arc<FindPillQueryHandler>,
    find_all_pills_handler: Arc<FindAllPillsQueryHandler>,
}

#[tokio::main]
async fn main() {
    // 1. Crear el Adaptador de Salida (Repositorio)
    let repo: Arc<dyn PillRepository> = Arc::new(InMemoryPillRepository::new());

    // 2. Crear los Servicios de Aplicación (Handlers)
    let app_state = AppState {
        create_pill_handler: Arc::new(CreatePillCommandHandler::new(repo.clone())),
        find_pill_handler: Arc::new(FindPillQueryHandler::new(repo.clone())),
        find_all_pills_handler: Arc::new(FindAllPillsQueryHandler::new(repo.clone())),
    };

    // 3. Crear el Adaptador de Entrada (API) y registrar las rutas
    let app = Router::new()
        // Rutas para el caso de uso de creación
        .route("/pills", post(create_pill_handler))
        .with_state(app_state.create_pill_handler)
        // Rutas para los casos de uso de consulta
        .route("/pills/:id", get(find_pill_by_id_handler))
        .with_state(app_state.find_pill_handler)
        .route("/pills", get(find_all_pills_handler))
        .with_state(app_state.find_all_pills_handler);

    // 4. Iniciar el servidor
    println!("🚀 Servidor escuchando en 0.0.0.0:3000");
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
