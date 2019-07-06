use crate::{
    config, error::KernelError, events::EventMetadata, myaccount, myaccount::domain::pending_email,
    myaccount::validators, utils,
};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};

#[derive(Clone, Debug)]
pub struct Create {
    pub email: String,
    pub account_id: uuid::Uuid,
    pub config: config::Config,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for Create {
    type Aggregate = pending_email::PendingEmail;
    type Event = Created;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        use crate::db::schema::kernel_accounts::dsl::*;
        use diesel::prelude::*;

        validators::email(self.config.disposable_email_domains.clone(), &self.email)?;

        // verify that an email isn't already in use
        let existing_email: i64 = kernel_accounts
            .filter(email.eq(&self.email))
            .filter(deleted_at.is_null())
            .count()
            .get_result(ctx)?;
        if existing_email != 0 {
            return Err(KernelError::Validation(format!(
                "Email: {} is already in use.",
                &self.email
            )));
        }

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        let new_pending_email_id = uuid::Uuid::new_v4();
        let code = utils::random_digit_string(8);
        let token = bcrypt::hash(&code, myaccount::PENDING_EMAIL_TOKEN_BCRYPT_COST)
            .map_err(|_| KernelError::Bcrypt)?;

        return Ok(Created {
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            id: new_pending_email_id,
            email: self.email.clone(),
            account_id: self.account_id,
            token,
            code,
        });
    }
}

// Event
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct Created {
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Event for Created {
    type Aggregate = pending_email::PendingEmail;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            id: data.id,
            created_at: self.timestamp,
            updated_at: self.timestamp,
            deleted_at: None,
            version: 0,
            email: data.email.clone(),
            token: data.token.clone(),
            trials: 0,
            account_id: data.account_id,
        };
    }
}
