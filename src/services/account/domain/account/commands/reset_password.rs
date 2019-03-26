use crate::{
    services::account::domain::account as account_domain,
    services::common::events::EventMetadata,
    services::account::validators,
    services::account,
    error::KernelError,
};
use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};
use chrono::Utc;


#[derive(Clone, Debug)]
pub struct ResetPassword {
    pub new_password: String,
    pub token: String,
    pub metadata: EventMetadata,
}

impl<'a> eventsourcing::Command<'a> for ResetPassword {
    type Aggregate = account_domain::Account;
    type Event = account_domain::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        validators::password(&self.new_password)?;
        let timestamp = Utc::now();
        let duration = aggregate.updated_at.signed_duration_since(timestamp);

        if aggregate.email == self.new_password {
            return Err(KernelError::Validation("Password cannot be your email address".to_string()));
        }
        if aggregate.username == self.new_password {
            return Err(KernelError::Validation("Password cannot be your username".to_string()));
        }

        if duration.num_minutes() > 30 {
            return Err(KernelError::Validation("Code has expired, please reset your password again".to_string()));
        }

        // we can unwrap because if we are here it means that we found the account with it's password_reset_id
        if !bcrypt::verify(&self.token, aggregate.password_reset_token.as_ref().unwrap())? {
            return Err(KernelError::Validation("Token is not valid".to_string()));
        }

        return Ok(());
    }

    fn build_event(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let hashed_password = bcrypt::hash(&self.new_password, account::PASSWORD_BCRYPT_COST)
            .map_err(|_| KernelError::Bcrypt)?;

        let data = account_domain::EventData::PasswordResetedV1(account_domain::PasswordResetedV1{
            password: hashed_password,
        });

        return  Ok((account_domain::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data,
            aggregate_id: aggregate.id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}