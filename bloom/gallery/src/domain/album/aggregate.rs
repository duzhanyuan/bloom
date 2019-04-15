use serde::{Serialize, Deserialize};
use diesel::{Queryable};
use kernel::{
    db::schema::{
        gallery_albums,
        gallery_albums_items,
        drive_files,
    },
};


#[derive(AsChangeset, Clone, Debug, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[table_name = "gallery_albums"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Album {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub version: i64,

    pub name: String,

    pub owner_id: uuid::Uuid,
}

impl Album {
    // create a new, unitialized note
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        return Note{
            id: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
            version: 0,

            name: String::new(),

            owner_id: uuid::Uuid::new_v4(),
        };
    }
}

impl eventsourcing::Aggregate for Album {
    fn increment_version(&mut self) {
        self.version += 1;
    }

    fn update_updated_at(&mut self, timestamp: chrono::DateTime<chrono::Utc>) {
        self.updated_at = timestamp;
    }
}



#[derive(Clone, Debug, Deserialize, QueryableByName, Serialize)]
#[table_name = "drive_files"]
pub struct File {
    pub id: uuid::Uuid,
    pub name: String,
}

#[derive(AsChangeset, Clone, Debug, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[table_name = "gallery_albums_items"]
pub struct AlbumItem {
    pub id: uuid::Uuid,

    pub album_id: uuid::Uuid,
    pub file_id: uuid::Uuid,
}