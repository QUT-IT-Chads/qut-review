use crate::token::JWT;
use domain::enums::role::Role;
use rocket::http::Status;
use uuid::Uuid;

pub fn has_user_permissions(token: &JWT, id: &Uuid) -> Result<(), (Status, Option<String>)> {
    if token.claims.sub != *id && token.claims.role != Role::Admin {
        return Err((
            Status::Unauthorized,
            Some(String::from(
                "You do not have access to perform this action.",
            )),
        ));
    }

    Ok(())
}

pub fn has_admin_permissions(token: &JWT) -> Result<(), (Status, Option<String>)> {
    if token.claims.role != Role::Admin {
        return Err((
            Status::Unauthorized,
            Some(String::from(
                "You do not have access to perform this action.",
            )),
        ));
    }

    Ok(())
}
