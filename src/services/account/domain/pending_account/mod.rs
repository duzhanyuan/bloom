mod create;
mod created;
mod verify;
mod verified;
mod resend_code;
mod code_resent;

use serde::{Serialize, Deserialize};
use diesel::{Queryable};
use diesel_as_jsonb::AsJsonb;
use crate::{
    db::schema::account_pending_accounts,
    db::schema::account_pending_accounts_events,
};


pub use create::Create;
pub use created::CreatedV1;
pub use verify::Verify;
pub use verified::VerifiedV1;
pub use resend_code::ResendCode;
pub use code_resent::CodeResentV1;

pub const TOKEN_BCRYPT_COST: u32 = 11;


#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "account_pending_accounts"]
pub struct PendingAccount {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub version: i64,

    pub password: String, // hashed password
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub token: String, // hashed code
    pub trials: i64,
}

#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "account_pending_accounts_events"]
pub struct Event {
    pub id: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub data: EventData,
    pub aggregate_id: uuid::Uuid,
    pub metadata: EventMetadata, // TODO: change
}


#[derive(Clone, Debug)]
pub enum Command {
    Create(Create),
    ResendCode(ResendCode),
    Verify(Verify),
}

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub enum EventData {
    CreatedV1(CreatedV1),
    CodeResentV1(CodeResentV1),
    VerifiedV1(VerifiedV1),
}

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub struct EventMetadata {
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum NonPersistedData {
}
