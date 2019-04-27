use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use kernel::{
    db::DbActor,
    KernelError,
};
use crate::domain;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindDownload {
    pub download_id: uuid::Uuid,
    pub account_id: uuid::Uuid,
}

impl Message for FindDownload {
    type Result = Result<domain::Download, KernelError>;
}

impl Handler<FindDownload> for DbActor {
    type Result = Result<domain::Download, KernelError>;

    fn handle(&mut self, msg: FindDownload, _: &mut Self::Context) -> Self::Result {
        use kernel::db::schema::{
            bitflow_downloads,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        let download: domain::Download = bitflow_downloads::dsl::bitflow_downloads
            .filter(bitflow_downloads::dsl::id.eq(msg.download_id))
            .filter(bitflow_downloads::dsl::owner_id.eq(msg.account_id))
            .filter(bitflow_downloads::dsl::deleted_at.is_null())
            .filter(bitflow_downloads::dsl::removed_at.is_null())
            .first(&conn)?;

        return Ok(download);
    }
}