use actix::{Message, Handler};
use crate::{
    db::DbActor,
    services::account::domain,
    services::account,
    services::account::validators,
    services::account::notifications::emails::send_account_verification_code,
    config::Config,
    services::common::utils,
    api::middlewares::logger::RequestLogger,
};
use crate::error::KernelError;
use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};


#[derive(Clone, Debug)]
pub struct Create {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub username: String,
}

impl Create {
    pub fn validate(&self, _conn: &PooledConnection<ConnectionManager<PgConnection>>) -> Result<(), KernelError> {
        validators::first_name(&self.first_name)?;
        validators::last_name(&self.last_name)?;
        validators::password(&self.password)?;
        // TODO: validate email

        // verify that an email isn't already in use
        // verify that username isn't already in use

        if self.password == self.email {
            return Err(KernelError::Validation("password must be different than your email address".to_string()));
        }

        return Ok(());
    }
}