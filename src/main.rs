mod courses;
mod database;
mod health;
mod logging;
mod pills;

use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::net::TcpListener;

use pills::application::command::CreatePillCommandHandler;
use pills::application::query::{FindAllPillsQueryHandler, FindPillQueryHandler};
use pills::domain::PillRepository;
use pills::infrastructure::controllers::create_pill_controller::create_pill_controller;
use pills::infrastructure::controllers::find_all_pills_controller::find_all_pills_controller;
use pills::infrastructure::controllers::find_pill_controller::find_pill_by_id_controller;
use pills::infrastructure::persistense::mongodb_repository::MongoDbPillRepository;

use courses::application::command::{AddPillToCourseCommandHandler, CreateCourseCommandHandler};
use courses::application::query::{
    FindAllCoursesQueryHandler, FindCourseQueryHandler, FindCourseWithPillsQueryHandler,
};
use courses::domain::CourseRepository;
use courses::infrastructure::controllers::add_pill_to_course_controller::add_pill_to_course_controller;
use courses::infrastructure::controllers::create_course_controller::create_course_controller;
use courses::infrastructure::controllers::find_all_courses_controller::find_all_courses_constroller;
use courses::infrastructure::controllers::find_course_controller::find_course_by_id_controller;
use courses::infrastructure::controllers::find_course_with_pills_controller::find_course_with_pills_controller;
use courses::infrastructure::persistence::mongodb_repository::MongoDbCourseRepository;

use database::DatabaseConfig;
use health::{
    health_check_handler, liveness_check_handler, readiness_check_handler, HealthService,
};

#[derive(Clone)]
struct AppState {
    // Pills handlers
    create_pill_handler: Arc<CreatePillCommandHandler>,
    find_pill_handler: Arc<FindPillQueryHandler>,
    find_all_pills_handler: Arc<FindAllPillsQueryHandler>,

    // Courses handlers
    create_course_handler: Arc<CreateCourseCommandHandler>,
    find_course_handler: Arc<FindCourseQueryHandler>,
    find_all_courses_handler: Arc<FindAllCoursesQueryHandler>,
    find_course_with_pills_handler: Arc<FindCourseWithPillsQueryHandler>,
    add_pill_to_course_handler: Arc<AddPillToCourseCommandHandler>,

    // Health service
    health_service: Arc<HealthService>,
}

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize logging
    logging::init();

    // Initialize database connection
    let db_config = DatabaseConfig::new()
        .await
        .expect("Failed to connect to MongoDB");

    // Create database indexes for better performance
    if let Err(e) = db_config.initialize_indexes().await {
        tracing::warn!("Failed to create database indexes: {}", e);
        tracing::warn!("Application will continue but performance may be affected");
    }

    let database = db_config.get_database();

    // Initialize health service
    let health_service = Arc::new(HealthService::new(Arc::new(database.clone())));

    // Initialize repositories
    let pill_repo: Arc<dyn PillRepository> = Arc::new(MongoDbPillRepository::new(database));
    let course_repo: Arc<dyn CourseRepository> = Arc::new(MongoDbCourseRepository::new(database));

    let app_state = AppState {
        // Pills handlers
        create_pill_handler: Arc::new(CreatePillCommandHandler::new(pill_repo.clone())),
        find_pill_handler: Arc::new(FindPillQueryHandler::new(pill_repo.clone())),
        find_all_pills_handler: Arc::new(FindAllPillsQueryHandler::new(pill_repo.clone())),

        // Courses handlers
        create_course_handler: Arc::new(CreateCourseCommandHandler::new(course_repo.clone())),
        find_course_handler: Arc::new(FindCourseQueryHandler::new(course_repo.clone())),
        find_all_courses_handler: Arc::new(FindAllCoursesQueryHandler::new(course_repo.clone())),
        find_course_with_pills_handler: Arc::new(FindCourseWithPillsQueryHandler::new(
            course_repo.clone(),
            pill_repo.clone(),
        )),
        add_pill_to_course_handler: Arc::new(AddPillToCourseCommandHandler::new(
            course_repo.clone(),
            pill_repo.clone(),
        )),
        health_service,
    };

    // Create separate routers for different handler states
    let health_router = Router::new()
        .route("/health", get(health_check_handler))
        .route("/health/ready", get(readiness_check_handler))
        .with_state(app_state.health_service.clone())
        .route("/health/live", get(liveness_check_handler));

    let pills_router = Router::new()
        .route("/pills", post(create_pill_controller))
        .with_state(app_state.create_pill_handler.clone())
        .route("/pills/:id", get(find_pill_by_id_controller))
        .with_state(app_state.find_pill_handler.clone())
        .route("/pills", get(find_all_pills_controller))
        .with_state(app_state.find_all_pills_handler.clone());

    let courses_router = Router::new()
        .route("/courses", post(create_course_controller))
        .with_state(app_state.create_course_handler.clone())
        .route("/courses/:id", get(find_course_by_id_controller))
        .with_state(app_state.find_course_handler.clone())
        .route("/courses", get(find_all_courses_constroller))
        .with_state(app_state.find_all_courses_handler.clone())
        .route("/courses/:id/pills", get(find_course_with_pills_controller))
        .with_state(app_state.find_course_with_pills_handler.clone())
        .route("/courses/:id/pills", post(add_pill_to_course_controller))
        .with_state(app_state.add_pill_to_course_handler.clone());

    let app = Router::new()
        .merge(health_router)
        .merge(pills_router)
        .merge(courses_router);

    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string());
    let address = format!("{}:{}", host, port);

    tracing::info!("🚀 Server listening on {}", address);
    tracing::info!("📊 Health check available at: http://{}/health", address);
    tracing::info!("🔍 API endpoints:");
    tracing::info!("   - POST /pills - Create a pill");
    tracing::info!("   - GET  /pills - Get all pills");
    tracing::info!("   - GET  /pills/{{id}} - Get pill by ID");
    tracing::info!("   - POST /courses - Create a course");
    tracing::info!("   - GET  /courses - Get all courses");
    tracing::info!("   - GET  /courses/{{id}} - Get course by ID");
    tracing::info!("   - GET  /courses/{{id}}/pills - Get course with pills");
    tracing::info!("   - POST /courses/{{id}}/pills - Add pill to course");
    tracing::info!("   - GET  /health - Health check");
    tracing::info!("   - GET  /health/ready - Readiness probe");
    tracing::info!("   - GET  /health/live - Liveness probe");

    let listener = TcpListener::bind(&address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
