use kernel::myaccount::domain::account;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountResponse {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub avatar_url: String,
    pub is_admin: bool,
    pub is_disabled: bool,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<account::Account> for AccountResponse {
    fn from(account: account::Account) -> Self {
        AccountResponse {
            id: account.id,
            created_at: account.created_at,
            first_name: account.first_name,
            last_name: account.last_name,
            username: account.username,
            email: account.email,
            avatar_url: account.avatar_url,
            is_admin: account.is_admin,
            is_disabled: account.is_disabled,
            deleted_at: account.deleted_at,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountsResponse {
    pub hits: Vec<AccountResponse>,
    pub total: u64,
}
