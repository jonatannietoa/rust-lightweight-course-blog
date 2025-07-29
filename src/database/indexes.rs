use crate::database::DatabaseError;
use mongodb::bson::doc;
use mongodb::{Database, IndexModel};

/// Creates all necessary database indexes for optimal query performance
pub async fn create_all_indexes(database: &Database) -> Result<(), DatabaseError> {
    create_pills_indexes(database).await?;
    create_courses_indexes(database).await?;

    println!("Database: All indexes created successfully");
    Ok(())
}

/// Creates indexes for the pills collection
async fn create_pills_indexes(database: &Database) -> Result<(), DatabaseError> {
    let pills_collection = database.collection::<mongodb::bson::Document>("pills");

    // Create index on pill title for faster text searches
    pills_collection
        .create_index(
            IndexModel::builder().keys(doc! { "title": 1 }).build(),
            None,
        )
        .await?;

    println!("Database: Pills collection indexes created");
    Ok(())
}

/// Creates indexes for the courses collection
async fn create_courses_indexes(database: &Database) -> Result<(), DatabaseError> {
    let courses_collection = database.collection::<mongodb::bson::Document>("courses");

    // Create index on course title
    courses_collection
        .create_index(
            IndexModel::builder().keys(doc! { "title": 1 }).build(),
            None,
        )
        .await?;

    // Create index on course instructor
    courses_collection
        .create_index(
            IndexModel::builder().keys(doc! { "instructor": 1 }).build(),
            None,
        )
        .await?;

    println!("Database: Courses collection indexes created");
    Ok(())
}

/// Creates compound indexes for complex queries
#[allow(dead_code)]
pub async fn create_compound_indexes(database: &Database) -> Result<(), DatabaseError> {
    let courses_collection = database.collection::<mongodb::bson::Document>("courses");

    // Example: Compound index on instructor and title for efficient filtering
    courses_collection
        .create_index(
            IndexModel::builder()
                .keys(doc! { "instructor": 1, "title": 1 })
                .build(),
            None,
        )
        .await?;

    println!("Database: Compound indexes created");
    Ok(())
}

/// Creates text indexes for full-text search capabilities
#[allow(dead_code)]
pub async fn create_text_indexes(database: &Database) -> Result<(), DatabaseError> {
    let pills_collection = database.collection::<mongodb::bson::Document>("pills");
    let courses_collection = database.collection::<mongodb::bson::Document>("courses");

    // Text index for pills content search
    pills_collection
        .create_index(
            IndexModel::builder()
                .keys(doc! {
                    "title": "text",
                    "content": "text"
                })
                .build(),
            None,
        )
        .await?;

    // Text index for courses search
    courses_collection
        .create_index(
            IndexModel::builder()
                .keys(doc! {
                    "title": "text",
                    "description": "text"
                })
                .build(),
            None,
        )
        .await?;

    println!("Database: Text search indexes created");
    Ok(())
}
