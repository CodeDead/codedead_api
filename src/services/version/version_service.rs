use crate::repository::v2::dao::version_repository::VersionRepository;
use crate::repository::v2::model::version::Version;
use log::info;
use mongodb::Database;

#[derive(Clone)]
pub struct VersionService {
    version_repository: VersionRepository,
}

impl VersionService {
    /// Initialize a new VersionService
    ///
    /// # Arguments
    ///
    /// * `version_repository` - An instance of `VersionRepository` to interact with the database
    ///
    /// # Returns
    ///
    /// A new instance of `VersionService`
    pub fn new(version_repository: VersionRepository) -> Self {
        Self { version_repository }
    }

    /// Find a single Version by its ID
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the Version
    /// * `db` - The MongoDB database
    ///
    /// # Returns
    ///
    /// A result containing an Option with the Version if found, or None if not found, or an error if the MongoDB query fails
    pub async fn find_by_id(
        &self,
        id: &str,
        db: &Database,
    ) -> Result<Option<Version>, mongodb::error::Error> {
        info!("Retrieving version with ID: {}", id);
        self.version_repository.find_by_id(id, db).await
    }

    /// Find all Version records sorted by ID
    ///
    /// # Arguments
    ///
    /// * `limit` - The maximum number of Version structs to return
    /// * `db` - The MongoDB database
    ///
    /// # Returns
    ///
    /// A Result containing a Vec of Version structs sorted by ID
    ///
    /// # Errors
    ///
    /// Returns an error if the MongoDB query fails
    pub async fn get_all_sorted_by_id(
        &self,
        limit: Option<i64>,
        db: &Database,
    ) -> Result<Vec<Version>, mongodb::error::Error> {
        info!(
            "Retrieving all versions sorted by ID with limit: {:?}",
            limit
        );
        self.version_repository
            .get_all_sorted_by_id(limit, db)
            .await
    }

    /// Find all Version records with an ID greater than the provided ID
    ///
    /// # Arguments
    ///
    /// * `id` - The ID from which to start searching
    /// * `limit` - The maximum number of Version structs to return
    ///
    /// # Returns
    ///
    /// A Result containing a Vec of Version structs with an ID greater than the provided ID, sorted by ID
    ///
    /// # Errors
    ///
    /// Returns an error if the MongoDB query fails
    pub async fn get_all_with_id_greater_than(
        &self,
        id: &str,
        limit: Option<i64>,
        db: &Database,
    ) -> Result<Vec<Version>, mongodb::error::Error> {
        info!(
            "Retrieving all versions with ID greater than {} sorted by ID with limit: {:?}",
            id, limit
        );
        self.version_repository
            .get_all_with_id_greater_than(id, limit, db)
            .await
    }
}
