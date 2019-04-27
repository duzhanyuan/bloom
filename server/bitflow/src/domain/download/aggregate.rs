use serde::{Serialize, Deserialize};
use diesel::{Queryable};
use diesel_as_jsonb::AsJsonb;
use kernel::{
    db::schema::{
        bitflow_downloads,
    },
};


#[derive(AsChangeset, Clone, Debug, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[table_name = "bitflow_downloads"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Download {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub version: i64,


    pub error: Option<String>,
    pub name: String,
    pub progress: i32,
    pub removed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub state: DownloadState,
    pub url: DownloadUrl,

    pub owner_id: uuid::Uuid,
}

#[derive(Clone, Debug, Deserialize, DieselEnum, Serialize)]
pub enum DownloadState {
    Queued,
    Downloading,
    Stopped,
    Completed,
    Failed,
}

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub enum DownloadUrl {
    Http(DownloadUrlHttp),
    TorrentMagnet(DownloadUrlTorrentMagnet),
    None,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DownloadUrlHttp {
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DownloadUrlTorrentMagnet {
    pub magnet: String,
}

impl Download {
    // create a new, unitialized note
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        return Download{
            id: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
            version: 0,

            error: None,
            name: String::new(),
            progress: 0,
            removed_at: None,
            state: DownloadState::Queued,
            url: DownloadUrl::None,

            owner_id: uuid::Uuid::new_v4(),
        };
    }
}

impl eventsourcing::Aggregate for Download {
    fn increment_version(&mut self) {
        self.version += 1;
    }

    fn update_updated_at(&mut self, timestamp: chrono::DateTime<chrono::Utc>) {
        self.updated_at = timestamp;
    }
}