use actix::{Message, Handler};
use crate::{
    db::DbActor,
    error::KernelError,
    config::Config,
    services::account::domain::{
        Account,
        PendingAccount,
        pending_account,
        account,
    },
    services::account::domain,
};
use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompleteRegistration {
    pub id: String,
    pub code: String,
    pub username: String,
    pub config: Config,
}

impl Message for CompleteRegistration {
    type Result = Result<bool, KernelError>;
}

impl Handler<CompleteRegistration> for DbActor {
    type Result = Result<bool, KernelError>;

    fn handle(&mut self, msg: CompleteRegistration, _: &mut Self::Context) -> Self::Result {
        // verify pending account
        use crate::db::schema::{
            account_pending_accounts,
            account_pending_accounts_events,
            account_accounts,
            account_accounts_events,
        };
        use diesel::RunQueryDsl;
        use diesel::query_dsl::filter_dsl::FilterDsl;
        use diesel::ExpressionMethods;

        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        let pending_account_id = uuid::Uuid::parse_str(&msg.id)
            .map_err(|_| KernelError::Validation("id is not a valid uuid".to_string()))?;

        let mut pending_account_to_update: PendingAccount = account_pending_accounts::dsl::account_pending_accounts
            .filter(account_pending_accounts::dsl::id.eq(pending_account_id))
            .filter(account_pending_accounts::dsl::deleted_at.is_null())
            .first(&conn)?;

        println!("pending_account: {:?}", &pending_account_to_update);

        let now = Utc::now();
        let complete_registration_cmd = pending_account::CompleteRegistration{
            id: msg.id.clone(),
            code: msg.code.clone(),
        };

        // validate
        complete_registration_cmd.validate(&conn, &pending_account_to_update)?;

        pending_account_to_update.version += 1;
        pending_account_to_update.updated_at = now;

        diesel::update(account_pending_accounts::dsl::account_pending_accounts
            .filter(account_pending_accounts::dsl::id.eq(pending_account_id))
        )
            .set((
                account_pending_accounts::dsl::version.eq(pending_account_to_update.version),
                account_pending_accounts::dsl::updated_at.eq(pending_account_to_update.updated_at),
                account_pending_accounts::dsl::deleted_at.eq(Some(now)),
            ))
            .execute(&conn)?;

        let event = pending_account::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: now,
            data: pending_account::EventData::RegistrationCompletedV1,
            aggregate_id: pending_account_to_update.id,
            metadata: pending_account::EventMetadata{},
        };
        diesel::insert_into(account_pending_accounts_events::dsl::account_pending_accounts_events)
            .values(&event)
            .execute(&conn)?;


        // create account
        // build_event
        let created = domain::account::CreatedV1{
            id: uuid::Uuid::new_v4(),
            password: pending_account_to_update.password.clone(),
            email: pending_account_to_update.email.clone(),
            first_name: pending_account_to_update.first_name.clone(),
            last_name: pending_account_to_update.last_name.clone(),
            username: msg.username.clone(),
            avatar_url: format!("{}/imgs/profile.jpg", msg.config.www_host()),
            is_admin: false,
        };

        // apply event to aggregate
        let created_account = domain::Account{
            id: created.id.clone(),
            created_at: now.clone(),
            updated_at: now.clone(),
            deleted_at: None,
            version: 1,

            password: created.password.clone(),
            email: created.email.clone(),
            first_name: created.first_name.clone(),
            last_name: created.last_name.clone(),
            username: created.username.clone(),
            avatar_url: created.avatar_url.clone(),
            recovery_token: None,
            recovery_id: None,
            is_admin: created.is_admin,
        };


        let event = account::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: now,
            data: account::EventData::CreatedV1(created),
            aggregate_id: created_account.id.clone(),
            metadata: account::EventMetadata{},
        };

        diesel::insert_into(account_accounts::dsl::account_accounts)
            .values(&created_account)
            .execute(&conn)?;
        diesel::insert_into(account_accounts_events::dsl::account_accounts_events)
            .values(&event)
            .execute(&conn)?;

        // start Session

        return Ok(false);
    }
}
