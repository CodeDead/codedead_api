use crate::repository::application::model::application::Application;
use futures::TryStreamExt;
use mongodb::Database;
use mongodb::bson::doc;

#[derive(Clone)]
pub struct ApplicationRepository {
    collection: String,
}

impl ApplicationRepository {
    /// Initialize a new ApplicationRepository
    ///
    /// # Arguments
    ///
    /// * `collection` - The name of the collection.
    ///
    /// # Returns
    ///
    /// A new instance of ApplicationRepository.
    pub(crate) fn new(collection: String) -> Self {
        ApplicationRepository { collection }
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
    pub(crate) async fn get_by_id(
        &self,
        id: &str,
        db: &Database,
    ) -> Result<Option<Application>, mongodb::error::Error> {
        let collection = db.collection::<Application>(&self.collection);
        let filter = doc! { "_id": id };
        match collection.find_one(filter).await {
            Ok(application) => Ok(application),
            Err(e) => Err(e),
        }
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
    pub(crate) async fn get_all_sorted_by_id(
        &self,
        limit: i64,
        db: &Database,
    ) -> Result<Vec<Application>, mongodb::error::Error> {
        let sort = doc! {"_id": 1};
        let cursor = match db
            .collection::<Application>(&self.collection)
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
    pub(crate) async fn get_all_with_id_greater_than(
        &self,
        id: &str,
        limit: i64,
        db: &Database,
    ) -> Result<Vec<Application>, mongodb::error::Error> {
        let filter = doc! { "_id": { "$gt": id } };
        let cursor = match db
            .collection::<Application>(&self.collection)
            .find(filter)
            .limit(limit)
            .await
        {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![]))
    }
}
