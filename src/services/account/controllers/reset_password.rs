use actix::{Message, Handler};
use crate::{
    db::DbActor,
    services::account::domain::{
        Account,
        account,
        Session,
        session,
    },
    services::common::events::EventMetadata,
};
use crate::error::KernelError;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResetPassword {
    pub reset_password_id: uuid::Uuid,
    pub token: String,
    pub new_password: String,
    pub request_id: String,
}

impl Message for ResetPassword {
    type Result = Result<(Session, String), KernelError>;
}

impl Handler<ResetPassword> for DbActor {
    type Result = Result<(Session, String), KernelError>;

    fn handle(&mut self, msg: ResetPassword, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            account_accounts,
            account_accounts_events,
            account_sessions,
            account_sessions_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let user: Account = account_accounts::dsl::account_accounts
                .filter(account_accounts::dsl::password_reset_id.eq(msg.reset_password_id))
                .filter(account_accounts::dsl::deleted_at.is_null())
                .first(&conn)?;

            let metadata = EventMetadata{
                actor_id: Some(user.id),
                request_id: Some(msg.request_id.clone()),
            };

            let update_last_name_cmd = account::ResetPassword{
                new_password: msg.new_password,
                token: msg.token,
                metadata: metadata.clone(),
            };

            let (user, event, _) = eventsourcing::execute(&conn, user, &update_last_name_cmd)?;

            // update account
            diesel::update(account_accounts::dsl::account_accounts
                .filter(account_accounts::dsl::id.eq(user.id)))
                .set((
                    account_accounts::dsl::version.eq(user.version),
                    account_accounts::dsl::updated_at.eq(user.updated_at),
                    account_accounts::dsl::password.eq(&user.password),
                ))
                .execute(&conn)?;
            diesel::insert_into(account_accounts_events::dsl::account_accounts_events)
                .values(&event)
                .execute(&conn)?;

            // revoke all active sessions
            let sessions: Vec<Session> = account_sessions::dsl::account_sessions
                .filter(account_sessions::dsl::account_id.eq(user.id))
                .filter(account_sessions::dsl::deleted_at.is_null())
                .load(&conn)?;

            let revoke_cmd = session::Revoke{
                current_session_id: None,
                reason: session::RevokedReason::PasswordReseted,
                metadata: metadata.clone(),
            };

            for session in sessions {
                let (session, event, _) = eventsourcing::execute(&conn, session, &revoke_cmd)?;
                // update session
                diesel::update(account_sessions::dsl::account_sessions
                    .filter(account_sessions::dsl::id.eq(session.id)))
                    .set((
                        account_sessions::dsl::version.eq(session.version),
                        account_sessions::dsl::updated_at.eq(session.updated_at),
                        account_sessions::dsl::deleted_at.eq(session.deleted_at),
                    ))
                    .execute(&conn)?;
                diesel::insert_into(account_sessions_events::dsl::account_sessions_events)
                    .values(&event)
                    .execute(&conn)?;
            }

            // start new session
            let start_cmd = session::Start{
                account_id: user.id,
                ip: "127.0.0.1".to_string(), // TODO
                user_agent: "".to_string(), // TODO
                metadata,
            };
            let (new_session, event, non_stored) = eventsourcing::execute(&conn, Session::new(), &start_cmd)?;

            diesel::insert_into(account_sessions::dsl::account_sessions)
                .values(&new_session)
                .execute(&conn)?;
            diesel::insert_into(account_sessions_events::dsl::account_sessions_events)
                .values(&event)
                .execute(&conn)?;

            return Ok((new_session, non_stored.token_cleartext));
        })?);
    }
}