use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use kernel::{
    KernelError,
    events::EventMetadata,
    db::DbActor,
};
use crate::domain::{
    playlist,
    Playlist,
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RenamePlaylist {
    pub playlist_id: uuid::Uuid,
    pub name: String,
    pub account_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
}

impl Message for RenamePlaylist {
    type Result = Result<Playlist, KernelError>;
}

impl Handler<RenamePlaylist> for DbActor {
    type Result = Result<Playlist, KernelError>;

    fn handle(&mut self, msg: RenamePlaylist, _: &mut Self::Context) -> Self::Result {
        use kernel::db::schema::{
            music_playlists,
            music_playlists_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let metadata = EventMetadata{
                actor_id: Some(msg.account_id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };
            let rename_cmd = playlist::Rename{
                name: msg.name,
                metadata,
            };

            let playlist_to_update: Playlist = music_playlists::dsl::music_playlists
                .filter(music_playlists::dsl::id.eq(msg.playlist_id))
                .filter(music_playlists::dsl::owner_id.eq(msg.account_id))
                .filter(music_playlists::dsl::deleted_at.is_null())
                .for_update()
                .first(&conn)?;

            let (playlist_to_update, event, _) = eventsourcing::execute(&conn, playlist_to_update, &rename_cmd)?;
            // update playlist
            diesel::update(&playlist_to_update)
                .set(&playlist_to_update)
                .execute(&conn)?;
            diesel::insert_into(music_playlists_events::dsl::music_playlists_events)
                .values(&event)
                .execute(&conn)?;

            return Ok(playlist_to_update);
        })?);
    }
}