use crate::error::KernelError;


pub fn first_name(first_name: &str) -> Result<(), KernelError> {
    if first_name.is_empty() {
        return Err(KernelError::Validation("first_name cannot be empty".to_string()));
    }

    return Ok(());
}


pub fn last_name(last_name: &str) -> Result<(), KernelError> {
    if last_name.is_empty() {
        return Err(KernelError::Validation("last_name cannot be empty".to_string()));
    }

    return Ok(());
}

pub fn password(password: &str) -> Result<(), KernelError> {
    let length = password.len();

    if length < 8 {
        return Err(KernelError::Validation("password must be longer than 7 characters".to_string()));
    } else if length > 128 {
        return Err(KernelError::Validation("password must be shorter than 128 characters".to_string()));
    }

    return Ok(());
}


// TODO
pub fn email(disposable_emails: Vec<String>, email: &str) -> Result<(), KernelError> {
    let parts: Vec<&str> = email.split("@").collect();

    if parts.len() != 2 {
        return Err(KernelError::Validation("email is not valid".to_string()));
    }

    if parts[0].is_empty() || parts[1].is_empty() {
        return Err(KernelError::Validation("email is not valid".to_string()));
    }

    if email.trim() != email {
        return Err(KernelError::Validation("email must not contains whitesapces".to_string()));
    }

    if disposable_emails.contains(&parts[1].to_string()) {
        return Err(KernelError::Validation("email domain is not valid".to_string()));
    }

    return Ok(());
}

pub fn username(username: &str) -> Result<(), KernelError> {
    if username.is_empty() {
        return Err(KernelError::Validation("username cannot be empty".to_string()));
    }

    if username.len() < 4 {
        return Err(KernelError::Validation("username is too short".to_string()));
    }

    if username.len() > 16 {
        return Err(KernelError::Validation("username is too long".to_string()));
    }

    if username.to_lowercase() != username {
        return Err(KernelError::Validation("username must be lowercase".to_string()));
    }

    let invalid_usernames = vec![
        "admin",
        "sysy",
        "asministrator",
        "bloom",
        "bloom42",
        "support",
        "help",
        "settings",
        "admin1",
        "security",
        "profile",
        "42bloom",
    ];

    if invalid_usernames.contains(&username) {
        return Err(KernelError::Validation("username is not valid".to_string()));
    }

    if username.contains("administrator") {
        return Err(KernelError::Validation("username is not valid".to_string()));
    }

    if !username.chars().all(char::is_alphanumeric) {
        return Err(KernelError::Validation("username must contains only alphanumeric characters".to_string()));
    }

    return Ok(());
}
