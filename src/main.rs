mod courses;
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
use pills::infrastructure::controllers::create_pill_controller::create_pill_handler;
use pills::infrastructure::controllers::find_all_pills_controller::find_all_pills_handler;
use pills::infrastructure::controllers::find_pill_controller::find_pill_by_id_handler;
use pills::infrastructure::in_memory_repository::InMemoryPillRepository;

use courses::application::command::{AddPillToCourseCommandHandler, CreateCourseCommandHandler};
use courses::application::query::{
    FindAllCoursesQueryHandler, FindCourseQueryHandler, FindCourseWithPillsQueryHandler,
};
use courses::domain::CourseRepository;
use courses::infrastructure::controllers::add_pill_to_course_controller::add_pill_to_course_handler;
use courses::infrastructure::controllers::create_course_controller::create_course_handler;
use courses::infrastructure::controllers::find_all_courses_controller::find_all_courses_handler;
use courses::infrastructure::controllers::find_course_controller::find_course_by_id_handler;
use courses::infrastructure::controllers::find_course_with_pills_controller::find_course_with_pills_handler;
use courses::infrastructure::in_memory_course_repository::InMemoryCourseRepository;

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
}

#[tokio::main]
async fn main() {
    // Initialize repositories
    let pill_repo: Arc<dyn PillRepository> = Arc::new(InMemoryPillRepository::new());
    let course_repo: Arc<dyn CourseRepository> = Arc::new(InMemoryCourseRepository::new());

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
    };

    let app = Router::new()
        // Pills routes
        .route("/pills", post(create_pill_handler))
        .with_state(app_state.create_pill_handler.clone())
        .route("/pills/:id", get(find_pill_by_id_handler))
        .with_state(app_state.find_pill_handler.clone())
        .route("/pills", get(find_all_pills_handler))
        .with_state(app_state.find_all_pills_handler.clone())
        // Courses routes
        .route("/courses", post(create_course_handler))
        .with_state(app_state.create_course_handler.clone())
        .route("/courses/:id", get(find_course_by_id_handler))
        .with_state(app_state.find_course_handler.clone())
        .route("/courses", get(find_all_courses_handler))
        .with_state(app_state.find_all_courses_handler.clone())
        .route("/courses/:id/pills", get(find_course_with_pills_handler))
        .with_state(app_state.find_course_with_pills_handler.clone())
        .route("/courses/:id/pills", post(add_pill_to_course_handler))
        .with_state(app_state.add_pill_to_course_handler.clone());

    println!("🚀 Servidor escuchando en 0.0.0.0:3000");
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
