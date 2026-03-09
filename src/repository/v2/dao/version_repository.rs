use crate::repository::v2::model::version::Version;
use futures::TryStreamExt;
use mongodb::Database;
use mongodb::bson::doc;

#[derive(Clone)]
pub struct VersionRepository {
    collection: String,
}

impl VersionRepository {
    /// Initialize a new VersionRepository
    ///
    /// # Arguments
    ///
    /// * `collection` - The name of the MongoDB collection to use for Version records
    ///
    /// # Returns
    ///
    /// A new instance of VersionRepository
    pub fn new(collection: String) -> Self {
        Self { collection }
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
        let collection = db.collection::<Version>(&self.collection);
        let filter = doc! { "_id": id };
        match collection.find_one(filter).await {
            Ok(version) => Ok(version),
            Err(e) => Err(e),
        }
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
        let sort = doc! {"_id": 1};
        let limit = limit.unwrap_or(0);

        let cursor = match db
            .collection::<Version>(&self.collection)
            .find(doc! {})
            .sort(sort)
            .limit(limit)
            .await
        {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![]))
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
        let filter = doc! { "_id": { "$gt": id } };
        let sort = doc! {"_id": 1};
        let limit = limit.unwrap_or(0);

        let cursor = match db
            .collection::<Version>(&self.collection)
            .find(filter)
            .limit(limit)
            .sort(sort)
            .await
        {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![]))
    }
}
