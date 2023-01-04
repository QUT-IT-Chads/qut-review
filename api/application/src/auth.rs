use domain::enums::role::Role;
use rocket::{http::Status, serde::json::Json};
use shared::{response_models::ResponseMessage, token::JWT};
use uuid::Uuid;

pub fn has_user_permissions(token: &JWT, id: &Uuid) -> Result<(), (Status, Json<ResponseMessage>)> {
    if token.claims.sub != *id && token.claims.role != Role::Admin {
        let response = ResponseMessage {
            message: Some(String::from(
                "You do not have access to perform this action.",
            )),
        };

        return Err((Status::Unauthorized, Json(response)));
    }

    Ok(())
}

pub fn has_admin_permissions(token: &JWT) -> Result<(), (Status, Json<ResponseMessage>)> {
    if token.claims.role != Role::Admin {
        let response = ResponseMessage {
            message: Some(String::from(
                "You do not have access to perform this action.",
            )),
        };

        return Err((Status::Unauthorized, Json(response)));
    }

    Ok(())
}
