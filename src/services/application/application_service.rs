use crate::repository::application::dao::application_repository::ApplicationRepository;
use crate::repository::application::model::application::Application;
use log::info;
use mongodb::Database;

#[derive(Clone)]
pub struct ApplicationService {
    application_repository: ApplicationRepository,
}

impl ApplicationService {
    /// Create a new instance of ApplicationService
    ///
    /// # Arguments
    ///
    /// * `application_repository` - An instance of `ApplicationRepository`
    ///
    /// # Returns
    ///
    /// A new instance of `ApplicationService`
    pub fn new(application_repository: ApplicationRepository) -> Self {
        Self {
            application_repository,
        }
    }

    /// Find a single Application by its ID
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the Application
    /// * `db` - The MongoDB database
    ///
    /// # Returns
    ///
    /// An Option containing the Application if found, or None if not found
    ///
    /// # Errors
    ///
    /// Returns an error if the MongoDB query fails
    pub async fn get_by_id(
        &self,
        id: &str,
        db: &Database,
    ) -> Result<Option<Application>, mongodb::error::Error> {
        info!("Fetching application with ID: {}", id);
        self.application_repository.get_by_id(id, db).await
    }

    /// Find all Applications sorted by ID
    ///
    /// # Arguments
    ///
    /// * `limit` - The maximum number of Applications to return
    /// * `db` - The MongoDB database
    ///
    /// # Returns
    ///
    /// A Result containing a Vec of Applications
    ///
    /// # Errors
    ///
    /// Returns an error if the MongoDB query fails
    pub async fn get_all_sorted_by_id(
        &self,
        limit: Option<i64>,
        db: &Database,
    ) -> Result<Vec<Application>, mongodb::error::Error> {
        info!(
            "Fetching all applications sorted by ID with limit: {}",
            limit.unwrap_or(0)
        );
        self.application_repository
            .get_all_sorted_by_id(limit, db)
            .await
    }

    /// Find all Applications with an ID greater than the provided ID
    ///
    /// # Arguments
    ///
    /// * `id` - The ID from which to start searching
    /// * `limit` - The maximum number of Applications to return
    ///
    /// # Returns
    ///
    /// A Result containing a Vec of Applications
    ///
    /// # Errors
    ///
    /// Returns an error if the MongoDB query fails
    pub async fn get_all_with_id_greater_than(
        &self,
        id: &str,
        limit: Option<i64>,
        db: &Database,
    ) -> Result<Vec<Application>, mongodb::error::Error> {
        info!(
            "Fetching all applications with ID greater than: {} with limit: {}",
            id,
            limit.unwrap_or(0)
        );
        self.application_repository
            .get_all_with_id_greater_than(id, limit, db)
            .await
    }
}
